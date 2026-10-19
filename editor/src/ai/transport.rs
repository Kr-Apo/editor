//! One bounded worker per panel; async HTTP futures are dropped on Stop/timeout.
use super::{
    protocol::{self, Decoder, Message, Provider, Settings},
    tools,
    vault::Secret,
};
use futures_util::StreamExt;
use serde_json::{Value, json};
use std::{
    sync::mpsc::{Receiver, SyncSender, sync_channel},
    time::Duration,
};

pub struct Turn {
    pub messages: Vec<Message>,
    pub plan: Option<tools::EditPlan>,
    pub proposal_ids: Vec<String>,
    pub proposal_id: String,
    pub failure: Option<tools::ToolError>,
}
pub enum Event {
    ViewportRequested,
    Activity(super::activity::Phase),
    Diagnostic(tools::ToolError, String),
    Text(String),
    Done(Result<Turn, String>),
    Models(Result<Vec<String>, String>),
}
pub struct Job {
    pub receiver: Receiver<Event>,
    cancel: Option<tokio::sync::oneshot::Sender<()>>,
    pub stopping: bool,
}
impl Job {
    pub fn stop(&mut self) {
        self.stopping = true;
        if let Some(c) = self.cancel.take() {
            let _ = c.send(());
        }
    }
}
impl Drop for Job {
    fn drop(&mut self) {
        self.stop();
    }
}

fn client(base: &str) -> Result<reqwest::Client, String> {
    let mut builder = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(10));
    if reqwest::Url::parse(base)
        .ok()
        .and_then(|u| u.host_str().map(str::to_owned))
        .is_some_and(|h| {
            h == "localhost"
                || h.trim_matches(['[', ']'])
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        })
    {
        builder = builder.no_proxy();
    }
    builder
        .build()
        .map_err(|_| "Could not initialize secure HTTP client".into())
}
fn authorize(
    request: reqwest::RequestBuilder,
    provider: Provider,
    key: &Secret,
) -> Result<reqwest::RequestBuilder, String> {
    use reqwest::header::{AUTHORIZATION, HeaderValue};
    if key.0.is_empty() {
        return if provider.cloud() {
            Err("This cloud provider requires an API key".into())
        } else {
            Ok(request)
        };
    }
    let value = if provider == Provider::Anthropic {
        key.0.to_string()
    } else {
        format!("Bearer {}", key.0.as_str())
    };
    let mut header = HeaderValue::from_str(&value).map_err(|_| "Invalid API key characters")?;
    header.set_sensitive(true);
    let request = if provider == Provider::Anthropic {
        request
            .header("x-api-key", header)
            .header("anthropic-version", "2023-06-01")
    } else {
        request.header(AUTHORIZATION, header)
    };
    Ok(request)
}
fn status(code: reqwest::StatusCode) -> Result<(), String> {
    if code.is_success() {
        return Ok(());
    }
    Err(match code.as_u16(){401|403=>"Authentication refused (401/403). Check the key and provider.",429=>"Rate limit or quota reached (429). No automatic retry was made.",400|422=>"Request rejected. Check the model; if tools are unsupported, disable Model tools and use conversation only.",_=>"Provider HTTP error. No project changes were made."}.into())
}

pub struct Framer {
    buffer: Vec<u8>,
    data: String,
    bytes: usize,
}
impl Framer {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            data: String::new(),
            bytes: 0,
        }
    }
    pub fn push(
        &mut self,
        bytes: &[u8],
        provider: Provider,
        decoder: &mut Decoder,
    ) -> Result<(), String> {
        self.bytes += bytes.len();
        if self.bytes > 2 * 1024 * 1024 {
            return Err("Stream exceeded the 2 MiB transport limit".into());
        }
        self.buffer.extend_from_slice(bytes);
        while let Some(i) = self.buffer.iter().position(|b| *b == b'\n') {
            let tail = self.buffer.split_off(i + 1);
            let line = std::mem::replace(&mut self.buffer, tail);
            let text = std::str::from_utf8(&line[..i])
                .map_err(|_| "Invalid UTF-8 stream")?
                .trim_end_matches('\r');
            self.line(text, provider, decoder)?;
        }
        if self.buffer.len() > 256 * 1024 {
            return Err("Oversized stream line".into());
        }
        Ok(())
    }
    fn line(&mut self, line: &str, p: Provider, d: &mut Decoder) -> Result<(), String> {
        if p == Provider::Ollama {
            if !line.trim().is_empty() {
                d.frame(p, line)?;
            }
        } else if line.is_empty() {
            if !self.data.is_empty() {
                d.frame(p, self.data.trim_end_matches('\n'))?;
                self.data.clear();
            }
        } else if let Some(data) = line.strip_prefix("data:") {
            self.data.push_str(data.strip_prefix(' ').unwrap_or(data));
            self.data.push('\n');
        }
        Ok(())
    }
    pub fn end(&mut self, p: Provider, d: &mut Decoder) -> Result<(), String> {
        let bytes = std::mem::take(&mut self.buffer);
        let line = std::str::from_utf8(&bytes).map_err(|_| "Invalid UTF-8 stream")?;
        if !line.is_empty() {
            self.line(line, p, d)?;
        }
        self.line("", p, d)
    }
}

/// Exact wire context includes every advertised schema and required continuation.
/// Binary PNG is bounded independently; never drop schemas/history to fit.
fn bounded_request(
    settings: &Settings,
    messages: &[Message],
    defs: &[Value],
) -> Result<Value, String> {
    let body = protocol::request(settings, messages, defs);
    let bytes = serde_json::to_vec(&body)
        .map_err(|_| "Request serialization failed")?
        .len();
    let image_bytes: usize = messages
        .iter()
        .filter_map(|m| m.image.as_ref())
        .map(|i| i.base64().len())
        .sum();
    if bytes > 4 * 1024 * 1024 || bytes.saturating_sub(image_bytes) > settings.context_kib * 1024 {
        return Err("Context byte budget exceeded (includes tool schemas, manual and history). Reduce reads/history or increase context budget; nothing was silently dropped.".into());
    }
    Ok(body)
}

async fn response(
    client: &reqwest::Client,
    base: &str,
    settings: &Settings,
    key: &Secret,
    messages: &[Message],
    defs: &[Value],
    progress: (&SyncSender<Event>, &eframe::egui::Context),
) -> Result<Message, String> {
    let (sender, ctx) = progress;
    let req = client
        .post(format!("{}{}", base, settings.provider.chat_path()))
        .json(&bounded_request(settings, messages, defs)?);
    let resp = authorize(req, settings.provider, key)?
        .send()
        .await
        .map_err(|_| "Cannot connect to provider or connection was lost")?;
    if !resp.status().is_success() {
        let code = resp.status();
        if matches!(code.as_u16(), 400 | 422 | 413) {
            let mut bytes = vec![];
            let mut stream = resp.bytes_stream();
            while let Some(Ok(chunk)) = stream.next().await {
                if bytes.len() + chunk.len() > 16 * 1024 {
                    break;
                }
                bytes.extend_from_slice(&chunk);
            }
            let error = serde_json::from_slice::<Value>(&bytes).unwrap_or_default();
            if matches!(
                error["error"]["code"].as_str(),
                Some("context_length_exceeded" | "context_window_exceeded")
            ) {
                return Err("Provider context window exceeded. Reduce history/reads/image quality or choose a larger model context. No retry was made.".into());
            }
            if messages.iter().any(|m| m.image.is_some()) {
                return Err("Request carrying a viewport image rejected (400/413/422). Cause not certified: check vision/model/parameters/context, or remove the image for text-only. No retry; no edits applied.".into());
            }
        }
        return Err(status(code)
            .err()
            .unwrap_or_else(|| "Provider HTTP error".into()));
    }
    notify(
        progress,
        Event::Activity(super::activity::Phase::Generating),
    );
    let mut stream = resp.bytes_stream();
    let mut framer = Framer::new();
    let mut decoder = Decoder::default();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|_| "Provider stream disconnected")?;
        let old = decoder.text.len();
        framer.push(&chunk, settings.provider, &mut decoder)?;
        if decoder.text.len() != old {
            let _ = sender.try_send(Event::Text(decoder.text.clone()));
            ctx.request_repaint();
        }
        if decoder.done {
            break;
        }
    }
    framer.end(settings.provider, &mut decoder)?;
    decoder.finish()
}

fn notify((sender, ctx): (&SyncSender<Event>, &eframe::egui::Context), event: Event) {
    ctx.request_repaint();
    let _ = sender.send(event);
}
static REQUEST_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
const MAX_CORRECTIONS: usize = 2;

async fn run(
    settings: &Settings,
    key: &Secret,
    mut messages: Vec<Message>,
    context: Option<tools::Snapshot>,
    sender: &SyncSender<Event>,
    ctx: &eframe::egui::Context,
    base: &str,
) -> Result<Turn, String> {
    use super::activity::Phase;
    settings.validate_capabilities()?;
    for message in &messages {
        if let Some(image) = &message.image {
            if message.role != "user" {
                return Err("Images are permitted only in user messages".into());
            }
            image.validate()?;
            settings.authorize_image(image)?;
        }
    }
    if messages.iter().filter(|m| m.image.is_some()).count() > 1 {
        return Err("At most one authorized viewport image per Send".into());
    }
    let client = client(base)?;
    let defs = if settings.tools_enabled && context.is_some() {
        tools::definitions(settings.editing)
    } else if settings.tools_enabled {
        vec![super::vision::definition()]
    } else {
        vec![]
    };
    let max_image_requests = messages
        .iter()
        .filter_map(|m| m.image.as_ref())
        .map(|i| i.max_requests.min(settings.image_requests))
        .min();
    let mut read_cache = std::collections::BTreeMap::<String, String>::new();
    let request_id = REQUEST_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let mut remaining = settings.tool_budget;
    let mut corrections = 0;
    for round in 0..=settings.tool_budget {
        // A loop is bounded by both native calls and wire history, not just tokens.
        if serde_json::to_vec(&messages)
            .map_err(|_| "Invalid history")?
            .len()
            > settings.context_kib * 1024
        {
            return Err("Context budget reached during tool use; clear history, reduce data or increase context budget".into());
        }
        if max_image_requests.is_some_and(|max| round >= max) {
            return Err("Authorized image-request budget reached. No edits applied. Capture/authorize a new Send or continue text-only; no automatic retry.".into());
        }
        notify((sender, ctx), Event::Activity(Phase::Connecting));
        let mut message = response(
            &client,
            base,
            settings,
            key,
            &messages,
            &defs,
            (sender, ctx),
        )
        .await?;
        if settings.provider == Provider::Ollama {
            for (index, call) in message.calls.iter_mut().enumerate() {
                call.id = format!("ollama_{request_id}_{round}_{index}");
            }
        }
        if message.calls.is_empty() {
            messages.push(message);
            return Ok(Turn {
                messages,
                plan: None,
                proposal_ids: vec![],
                proposal_id: String::new(),
                failure: None,
            });
        }
        if defs.is_empty() || message.calls.len() > remaining {
            return Err(
                "Tool-call budget exceeded or tools were not authorized. No writes were made."
                    .into(),
            );
        }
        remaining -= message.calls.len();
        let calls = message.calls.clone();
        messages.push(message);
        let proposal_id = format!("proposal_{request_id}_{round}");
        let mut actions = vec![];
        let mut summaries = vec![];
        let mut proposal_ids = vec![];
        let mut failure = None;
        for call in calls {
            if !defs.iter().any(|d| d["name"] == call.name) {
                return Err("Unknown or unauthorized native tool. No writes were made.".into());
            }
            let cache_key = matches!(call.name.as_str(), "ankhimate_read" | "ankhimate_read_mesh")
                .then(|| format!("{}:{}", call.name, call.arguments));
            if let Some(source) = cache_key.as_ref().and_then(|key| read_cache.get(key)) {
                messages.push(Message::result(&call.id,json!({"cache_hit":true,"source_call_id":source,"revision":context.as_ref().map(|c|c.revision),"note":"Unchanged native read already present in this Send; use the earlier tool result."})));
                continue;
            }
            notify(
                (sender, ctx),
                Event::Activity(Phase::NativeTool(call.name.clone())),
            );
            let result = match call.name.as_str() {
                "ankhimate_viewport" => {
                    if call.arguments.as_object().is_none_or(|o| !o.is_empty()) {
                        return Err("Viewport request takes an empty object".into());
                    }
                    notify((sender, ctx), Event::ViewportRequested);
                    json!({"status":"awaiting_image_consent","image_sent":false,"instruction":"Ask the user to Capture viewport, review it and authorize a new Send. You have NOT observed any image from this tool."})
                }
                "ankhimate_read_mesh" => {
                    let source = context
                        .as_ref()
                        .ok_or("Project metadata sharing was not authorized")?;
                    match serde_json::from_value::<super::weights::ReadMesh>(call.arguments) {
                        Ok(args)=>super::weights::read(source,&args).unwrap_or_else(|error|json!({"status":"unavailable","cause":error,"document_changed":false})),
                        Err(_)=>json!({"status":"unavailable","cause":"Use the exact paginated mesh-read schema","document_changed":false}),
                    }
                }
                "ankhimate_read" => {
                    #[derive(serde::Deserialize)]
                    #[serde(deny_unknown_fields)]
                    struct Read {
                        section: String,
                        bone: Option<String>,
                        offset: Option<usize>,
                        limit: Option<usize>,
                    }
                    let args: Read = serde_json::from_value(call.arguments)
                        .map_err(|_| "Invalid read-tool arguments")?;
                    let data = &context
                        .as_ref()
                        .ok_or("Project metadata sharing was not authorized")?
                        .metadata;
                    if args.offset.is_some() || args.limit.is_some() {
                        let offset = args.offset.unwrap_or(0);
                        let limit = args.limit.unwrap_or(32);
                        if offset > 4096 || !(1..=64).contains(&limit) {
                            return Err("Read page requires offset <=4096 and limit 1..64".into());
                        }
                        let rows=data[&args.section].as_array().ok_or("Pagination requires the bones, slots, attachments or animations array section")?;
                        json!({"items":rows.iter().skip(offset).take(limit).collect::<Vec<_>>(),"offset":offset,"available":rows.len(),"next_offset":(offset.saturating_add(limit)<rows.len()).then_some(offset+limit),"revision":context.as_ref().unwrap().revision,"note":"Bounded permitted snapshot, not an unbounded project dump"})
                    } else {
                        match args.section.as_str() {
                            "bone" => {
                                let name = args
                                    .bone
                                    .as_deref()
                                    .ok_or("bone section requires exact bone name")?;
                                let matches: Vec<_> = data["bones"]
                                    .as_array()
                                    .into_iter()
                                    .flatten()
                                    .filter(|b| b["name"] == name)
                                    .collect();
                                match matches.as_slice() {
                                    [bone] => (*bone).clone(),
                                    _ => {
                                        json!({"status":"unavailable","cause":"Bone missing or ambiguous in the permitted bounded metadata. Read exact names; never guess."})
                                    }
                                }
                            }
                            "overview" => data.clone(),
                            "bones" | "attachments" | "slots" | "animations" | "selection" => {
                                data[args.section].clone()
                            }
                            _ => return Err("Unknown read section".into()),
                        }
                    }
                }
                "ankhimate_animate" | "ankhimate_rig" | "ankhimate_assemble"
                | "ankhimate_weights"
                    if settings.editing && context.is_some() =>
                {
                    proposal_ids.push(call.id.clone());
                    match serde_json::from_value::<tools::EditPlan>(call.arguments) {
                        Ok(plan) => {
                            if plan.actions.iter().any(|a| match call.name.as_str() {
                                "ankhimate_weights" => !a.is_weights(),
                                "ankhimate_assemble" => !(a.is_rigging() || a.is_assembly()),
                                "ankhimate_rig" => !a.is_rigging(),
                                _ => a.is_setup(),
                            }) {
                                return Err(
                                    "Wrong native tool for action type; no edits made".into()
                                );
                            }
                            actions.extend(plan.actions);
                            summaries.push(plan.summary);
                        }
                        Err(e) => {
                            let mut error =
                                tools::ToolError::arguments(&e, context.as_ref().unwrap().revision);
                            error.operation = call.name.clone();
                            failure = Some(error);
                        }
                    }
                    json!({"status":"validating","proposal_id":proposal_id,"document_changed":false})
                }
                _ => return Err("Unknown or unauthorized native tool. No writes were made.".into()),
            };
            if let Some(key) = cache_key {
                read_cache.insert(key, call.id.clone());
            }
            messages.push(Message::result(&call.id, result));
        }
        if !proposal_ids.is_empty() {
            notify(
                (sender, ctx),
                Event::Activity(Phase::Validating(actions.len())),
            );
            let plan = tools::EditPlan {
                summary: summaries.join("; "),
                actions,
            };
            if failure.is_none() {
                failure = tools::validate(context.as_ref().unwrap(), &plan)
                    .err()
                    .map(|e| *e);
            }
            if let Some(error) = failure {
                let mut result =
                    serde_json::to_value(&error).map_err(|_| "Invalid native diagnostic")?;
                result["status"] = json!("invalid");
                result["proposal_id"] = json!(proposal_id);
                mark_results(&mut messages, &proposal_id, &proposal_ids, &result);
                notify(
                    (sender, ctx),
                    Event::Diagnostic(error.clone(), proposal_id.clone()),
                );
                if remaining == 0 || corrections >= MAX_CORRECTIONS {
                    return Ok(Turn {
                        messages,
                        plan: None,
                        proposal_ids,
                        proposal_id,
                        failure: Some(error),
                    });
                }
                corrections += 1;
                continue;
            }
            mark_results(
                &mut messages,
                &proposal_id,
                &proposal_ids,
                &json!({"status":"awaiting_user_approval","proposal_id":proposal_id,"revision":context.as_ref().unwrap().revision,"document_changed":false}),
            );
            notify((sender, ctx), Event::Activity(Phase::ProposalReady));
            return Ok(Turn {
                messages,
                plan: Some(plan),
                proposal_ids,
                proposal_id,
                failure: None,
            });
        }
        if remaining == 0 {
            return Err(
                "Tool-call budget reached. No writes were made; send a follow-up to continue."
                    .into(),
            );
        }
    }
    Err("Tool-call budget reached".into())
}

/// Match both local proposal identity and provider call ID. Provider IDs can be
/// reused between turns; never rewrite an unrelated historical failure.
pub fn mark_results(messages: &mut [Message], proposal: &str, ids: &[String], result: &Value) {
    for message in messages {
        if message.call_id.as_ref().is_some_and(|id| ids.contains(id))
            && serde_json::from_str::<Value>(&message.text)
                .ok()
                .is_some_and(|v| v["proposal_id"] == proposal)
        {
            message.text = result.to_string();
        }
    }
}

/// UI polling receives only owned data. No worker ever owns live editor state.
pub fn start(
    settings: Settings,
    key: Secret,
    messages: Vec<Message>,
    context: Option<tools::Snapshot>,
    models: bool,
    ctx: eframe::egui::Context,
) -> Result<Job, String> {
    let base = settings.base_url()?;
    if !models {
        settings.validate()?;
    }
    Ok(start_at(
        settings, key, messages, context, models, ctx, base,
    ))
}
pub(crate) fn start_at(
    settings: Settings,
    key: Secret,
    messages: Vec<Message>,
    context: Option<tools::Snapshot>,
    models: bool,
    ctx: eframe::egui::Context,
    base: String,
) -> Job {
    let (sender, receiver) = sync_channel(32);
    let (cancel, mut stopped) = tokio::sync::oneshot::channel();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();
        let result = if let Ok(rt) = rt {
            rt.block_on(async{
            tokio::select!{
                _=&mut stopped=>Err("Stopped by user; no changes applied".into()),
                value=tokio::time::timeout(Duration::from_secs(settings.timeout_seconds.clamp(1,600)),async{
                    if models {
                        let client=client(&base)?;let response=authorize(client.get(format!("{}{}",base,settings.provider.models_path())),settings.provider,&key)?.send().await.map_err(|_|"Cannot reach model server")?;status(response.status())?;
                        if response.content_length().is_some_and(|n|n>1024*1024){return Err("Model list is too large".into());}
                        let mut bytes=vec![];let mut stream=response.bytes_stream();while let Some(chunk)=stream.next().await{let chunk=chunk.map_err(|_|"Model list disconnected")?;bytes.extend_from_slice(&chunk);if bytes.len()>1024*1024{return Err("Model list is too large".into());}}
                        let data:Value=serde_json::from_slice(&bytes).map_err(|_|"Invalid model-list response")?;
                        let array=if settings.provider==Provider::Ollama{&data["models"]}else{&data["data"]};let mut ids:Vec<_>=array.as_array().ok_or("Missing model list")?.iter().filter_map(|v|v[if settings.provider==Provider::Ollama{"name"}else{"id"}].as_str()).take(512).map(str::to_string).collect();ids.sort();ids.dedup();
                        let _=sender.send(Event::Models(Ok(ids)));ctx.request_repaint();Ok(None)
                    }else{run(&settings,&key,messages,context,&sender,&ctx,&base).await.map(Some)}
                })=>value.unwrap_or_else(|_|Err("Request timed out; no project changes were made".into()))
            }
        })
        } else {
            Err("Could not start network worker".into())
        };
        ctx.request_repaint();
        match result {
            Ok(Some(turn)) => {
                let _ = sender.send(Event::Done(Ok(turn)));
            }
            Ok(None) => {}
            Err(e) => {
                let _ = sender.send(if models {
                    Event::Models(Err(e))
                } else {
                    Event::Done(Err(e))
                });
            }
        }
        ctx.request_repaint();
    });
    Job {
        receiver,
        cancel: Some(cancel),
        stopping: false,
    }
}

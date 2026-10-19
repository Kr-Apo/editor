//! Provider wire formats are distinct; all results cross the same typed boundary.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    OpenAI,
    Anthropic,
    DeepSeek,
    #[default]
    Ollama,
    LmStudio,
    NInfer,
}
impl Provider {
    pub const ALL: [Self; 6] = [
        Self::OpenAI,
        Self::Anthropic,
        Self::DeepSeek,
        Self::Ollama,
        Self::LmStudio,
        Self::NInfer,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::OpenAI => "OpenAI",
            Self::Anthropic => "Anthropic Claude",
            Self::DeepSeek => "DeepSeek",
            Self::Ollama => "Ollama",
            Self::LmStudio => "LM Studio",
            Self::NInfer => "NInfer",
        }
    }
    pub fn cloud(self) -> bool {
        matches!(self, Self::OpenAI | Self::Anthropic | Self::DeepSeek)
    }
    pub fn default_url(self) -> &'static str {
        match self {
            Self::OpenAI => "https://api.openai.com/v1",
            Self::Anthropic => "https://api.anthropic.com/v1",
            Self::DeepSeek => "https://api.deepseek.com",
            Self::Ollama => "http://127.0.0.1:11434",
            Self::LmStudio => "http://127.0.0.1:1234/v1",
            Self::NInfer => "http://127.0.0.1:8080/v1",
        }
    }
    pub fn chat_path(self) -> &'static str {
        match self {
            Self::OpenAI => "/responses",
            Self::Anthropic => "/messages",
            Self::Ollama => "/api/chat",
            _ => "/chat/completions",
        }
    }
    pub fn models_path(self) -> &'static str {
        if self == Self::Ollama {
            "/api/tags"
        } else {
            "/models"
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub provider: Provider,
    pub model: String,
    pub local_url: String,
    pub timeout_seconds: u64,
    pub max_tokens: u32,
    pub tool_budget: usize,
    pub context_kib: usize,
    pub manual_bytes: usize,
    pub image_requests: usize,
    pub image_quality: super::vision::Quality,
    pub capabilities: super::capabilities::Capabilities,
    pub editing: bool,
    pub tools_enabled: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            provider: Provider::Ollama,
            model: String::new(),
            local_url: Provider::Ollama.default_url().into(),
            timeout_seconds: 60,
            max_tokens: 2048,
            tool_budget: 6,
            context_kib: 256,
            manual_bytes: super::manual::MAX_BYTES,
            image_requests: 2,
            image_quality: super::vision::Quality::Balanced,
            capabilities: Default::default(),
            editing: false,
            tools_enabled: false,
        }
    }
}
impl Settings {
    pub fn base_url(&self) -> Result<String, String> {
        if self.provider.cloud() {
            return Ok(self.provider.default_url().into());
        }
        let u =
            reqwest::Url::parse(self.local_url.trim()).map_err(|_| "Invalid local server URL")?;
        let host = u.host_str().unwrap_or("").trim_matches(['[', ']']);
        let loopback = host == "localhost"
            || host
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback());
        if !loopback
            || !matches!(u.scheme(), "http" | "https")
            || !u.username().is_empty()
            || u.password().is_some()
            || u.query().is_some()
            || u.fragment().is_some()
        {
            return Err("Local servers must use a loopback HTTP(S) URL without credentials, query or fragment".into());
        }
        Ok(u.as_str().trim_end_matches('/').to_string())
    }
    pub fn validate(&self) -> Result<(), String> {
        self.base_url()?;
        if self.model.trim().is_empty() || self.model.len() > 256 {
            return Err("Enter a model identifier".into());
        }
        if !(1..=600).contains(&self.timeout_seconds)
            || !(128..=32768).contains(&self.max_tokens)
            || !(1..=16).contains(&self.tool_budget)
            || !(32..=2048).contains(&self.context_kib)
            || !(2048..=super::manual::MAX_BYTES).contains(&self.manual_bytes)
            || !(1..=17).contains(&self.image_requests)
        {
            return Err("Invalid timeout/output/tool-call limit".into());
        }
        self.validate_capabilities()
    }
    /// Convenience budgets, not a hardware or context-window certification.
    pub fn profile(&mut self, small: bool) {
        self.timeout_seconds = if small { 120 } else { 60 };
        self.max_tokens = if small { 1024 } else { 2048 };
        self.tool_budget = if small { 4 } else { 6 };
        self.context_kib = if small { 96 } else { 256 };
        self.manual_bytes = if small {
            3072
        } else {
            super::manual::MAX_BYTES
        };
        self.image_requests = 2;
        self.image_quality = if small {
            super::vision::Quality::Light
        } else {
            super::vision::Quality::Balanced
        };
    }
    pub fn credential_target(&self) -> Result<String, String> {
        Ok(format!(
            "Ankhimate.Community.AI/{:?}/{}",
            self.provider,
            self.base_url()?
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Call {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(skip)]
    pub image: Option<super::vision::ImageInput>,
    pub role: String,
    pub text: String,
    pub calls: Vec<Call>,
    pub call_id: Option<String>,
    #[serde(default)]
    pub openai_output: Vec<Value>,
    #[serde(default)]
    pub anthropic_output: Vec<Value>,
    #[serde(default)]
    pub reasoning_content: Option<String>,
}
impl Message {
    pub fn text(role: &str, text: impl Into<String>) -> Self {
        Self {
            image: None,
            role: role.into(),
            text: text.into(),
            calls: vec![],
            call_id: None,
            openai_output: vec![],
            anthropic_output: vec![],
            reasoning_content: None,
        }
    }
    pub fn result(id: &str, value: Value) -> Self {
        Self {
            image: None,
            role: "tool".into(),
            text: value.to_string(),
            calls: vec![],
            call_id: Some(id.into()),
            openai_output: vec![],
            anthropic_output: vec![],
            reasoning_content: None,
        }
    }
}

pub fn request(settings: &Settings, messages: &[Message], defs: &[Value]) -> Value {
    let instructions =
        super::manual::instructions_with_budget(messages, !defs.is_empty(), settings.manual_bytes);
    match settings.provider {
        Provider::OpenAI => {
            let mut input = vec![];
            for m in messages {
                if m.role == "tool" {
                    input.push(
                        json!({"type":"function_call_output","call_id":m.call_id,"output":m.text}),
                    );
                } else if !m.openai_output.is_empty() {
                    input.extend(m.openai_output.clone());
                } else {
                    if let Some(image) = &m.image {
                        input.push(json!({"role":m.role,"content":[{"type":"input_text","text":image.text(&m.text)},{"type":"input_image","image_url":image.data_url,"detail":"auto"}]}));
                    } else if !m.text.is_empty() {
                        input.push(json!({"role":m.role,"content":m.text}));
                    }
                    for c in &m.calls {
                        input.push(json!({"type":"function_call","call_id":c.id,"name":c.name,"arguments":c.arguments.to_string()}));
                    }
                }
            }
            let mut body = json!({"model":settings.model,"instructions":instructions,"input":input,"stream":true,"store":false,"include":["reasoning.encrypted_content"],"max_output_tokens":settings.max_tokens});
            if !defs.is_empty() {
                body["tools"]=json!(defs.iter().map(|d|json!({"type":"function","name":d["name"],"description":d["description"],"parameters":d["parameters"],"strict":false})).collect::<Vec<_>>());
            }
            body
        }
        Provider::Anthropic => {
            let mut wire: Vec<Value> = vec![];
            for m in messages {
                let (role, content) = if m.role == "tool" {
                    (
                        "user",
                        json!([{"type":"tool_result","tool_use_id":m.call_id,"content":m.text}]),
                    )
                } else if !m.anthropic_output.is_empty() {
                    (m.role.as_str(), json!(m.anthropic_output))
                } else {
                    let mut c = vec![];
                    if let Some(image) = &m.image {
                        c.push(json!({"type":"image","source":{"type":"base64","media_type":"image/png","data":image.base64()}}));
                        c.push(json!({"type":"text","text":image.text(&m.text)}));
                    } else if !m.text.is_empty() {
                        c.push(json!({"type":"text","text":m.text}));
                    }
                    for call in &m.calls {
                        c.push(json!({"type":"tool_use","id":call.id,"name":call.name,"input":call.arguments}));
                    }
                    (m.role.as_str(), json!(c))
                };
                if wire.last().is_some_and(|v| v["role"] == role) {
                    wire.last_mut().unwrap()["content"]
                        .as_array_mut()
                        .unwrap()
                        .extend(content.as_array().unwrap().clone());
                } else {
                    wire.push(json!({"role":role,"content":content}));
                }
            }
            let mut body = json!({"model":settings.model,"system":instructions,"messages":wire,"stream":true,"max_tokens":settings.max_tokens});
            if !defs.is_empty() {
                body["tools"]=json!(defs.iter().map(|d|json!({"name":d["name"],"description":d["description"],"input_schema":d["parameters"]})).collect::<Vec<_>>());
            }
            body
        }
        _ => {
            let native = settings.provider == Provider::Ollama;
            let mut wire = vec![json!({"role":"system","content":instructions})];
            let mut names = std::collections::HashMap::new();
            for m in messages {
                let mut v = json!({"role":m.role,"content":m.text});
                if let Some(image) = &m.image {
                    if native {
                        v["content"] = json!(image.text(&m.text));
                        v["images"] = json!([image.base64()]);
                    } else {
                        v["content"] = json!([{"type":"text","text":image.text(&m.text)},{"type":"image_url","image_url":{"url":image.data_url}}]);
                    }
                }
                if native && let Some(reasoning) = &m.reasoning_content {
                    v["thinking"] = json!(reasoning);
                }
                if matches!(settings.provider, Provider::DeepSeek | Provider::NInfer)
                    && let Some(reasoning) = &m.reasoning_content
                {
                    v["reasoning_content"] = json!(reasoning);
                }
                if let Some(id) = &m.call_id {
                    if native {
                        v["tool_name"] = json!(names.get(id));
                    } else {
                        v["tool_call_id"] = json!(id);
                    }
                }
                for c in &m.calls {
                    names.insert(c.id.clone(), c.name.clone());
                }
                if !m.calls.is_empty() {
                    v["tool_calls"]=json!(m.calls.iter().map(|c|json!({"id":c.id,"type":"function","function":{"name":c.name,"arguments":if native {c.arguments.clone()}else{json!(c.arguments.to_string())}}})).collect::<Vec<_>>());
                }
                wire.push(v);
            }
            let mut body = json!({"model":settings.model,"messages":wire,"stream":true});
            if native {
                body["options"] = json!({"num_predict":settings.max_tokens});
            } else {
                body["max_tokens"] = json!(settings.max_tokens);
            }
            if !defs.is_empty() {
                body["tools"] = json!(
                    defs.iter()
                        .map(|d| json!({"type":"function","function":d}))
                        .collect::<Vec<_>>()
                );
            }
            body
        }
    }
}

#[derive(Default)]
pub struct Decoder {
    pub text: String,
    calls: BTreeMap<u64, (String, String, String)>,
    pub done: bool,
    openai_output: Vec<Value>,
    openai_items: BTreeMap<u64, Value>,
    anthropic_output: BTreeMap<u64, Value>,
    reasoning: String,
}
impl Decoder {
    pub fn frame(&mut self, provider: Provider, data: &str) -> Result<(), String> {
        if data == "[DONE]" {
            self.done = true;
            return Ok(());
        }
        let v: Value = serde_json::from_str(data).map_err(|_| "Malformed provider stream")?;
        if v["type"] == "response.incomplete" {
            return Err(
                match v["response"]["incomplete_details"]["reason"].as_str() {
                    Some("max_output_tokens") => "Output token limit reached; no edits proposed",
                    Some("content_filter") => {
                        "Provider content filter interrupted response; no edits proposed"
                    }
                    _ => "Provider declared an incomplete response; no edits proposed",
                }
                .into(),
            );
        }
        if v.get("error").is_some()
            || matches!(
                v["type"].as_str(),
                Some("error" | "response.failed" | "response.incomplete")
            )
        {
            return Err("Provider returned a stream error or incomplete response".into());
        }
        match provider {
            Provider::OpenAI => match v["type"].as_str().unwrap_or("") {
                "response.output_text.delta" => {
                    self.text.push_str(v["delta"].as_str().unwrap_or(""))
                }
                "response.output_item.added" => {
                    let item = &v["item"];
                    if item["type"] == "function_call" {
                        self.calls.insert(
                            v["output_index"].as_u64().ok_or("Missing output index")?,
                            (
                                item["call_id"].as_str().unwrap_or("").into(),
                                item["name"].as_str().unwrap_or("").into(),
                                String::new(),
                            ),
                        );
                    }
                }
                "response.function_call_arguments.delta" => {
                    let c = self
                        .calls
                        .get_mut(&v["output_index"].as_u64().ok_or("Missing output index")?)
                        .ok_or("Tool delta without start")?;
                    c.2.push_str(v["delta"].as_str().unwrap_or(""));
                }
                "response.output_item.done" => {
                    self.openai_items.insert(
                        v["output_index"].as_u64().ok_or("Missing output index")?,
                        v["item"].clone(),
                    );
                }
                "response.completed" => {
                    self.done = true;
                    self.openai_output = v["response"]["output"]
                        .as_array()
                        .filter(|a| !a.is_empty())
                        .cloned()
                        .unwrap_or_else(|| self.openai_items.values().cloned().collect());
                }
                _ => {}
            },
            Provider::Anthropic => match v["type"].as_str().unwrap_or("") {
                "content_block_start" => {
                    let b = &v["content_block"];
                    self.anthropic_output
                        .insert(v["index"].as_u64().ok_or("Missing block index")?, b.clone());
                    if b["type"] == "tool_use" {
                        self.calls.insert(
                            v["index"].as_u64().ok_or("Missing block index")?,
                            (
                                b["id"].as_str().unwrap_or("").into(),
                                b["name"].as_str().unwrap_or("").into(),
                                String::new(),
                            ),
                        );
                    }
                }
                "content_block_delta" => {
                    let d = &v["delta"];
                    let b = self
                        .anthropic_output
                        .get_mut(&v["index"].as_u64().ok_or("Missing block index")?)
                        .ok_or("Missing content block")?;
                    let field = match d["type"].as_str() {
                        Some("text_delta") => Some("text"),
                        Some("thinking_delta") => Some("thinking"),
                        Some("signature_delta") => Some("signature"),
                        _ => None,
                    };
                    if let Some(f) = field {
                        let mut s = b[f].as_str().unwrap_or("").to_owned();
                        s.push_str(d[f].as_str().unwrap_or(""));
                        b[f] = json!(s);
                    }
                    if d["type"] == "text_delta" {
                        self.text.push_str(d["text"].as_str().unwrap_or(""));
                    }
                    if d["type"] == "input_json_delta" {
                        self.calls
                            .get_mut(&v["index"].as_u64().ok_or("Missing block index")?)
                            .ok_or("Tool delta without start")?
                            .2
                            .push_str(d["partial_json"].as_str().unwrap_or(""));
                    }
                }
                "message_delta" if v["delta"]["stop_reason"] == "max_tokens" => {
                    return Err("Output limit reached; no edits proposed".into());
                }
                "message_stop" => self.done = true,
                _ => {}
            },
            Provider::Ollama => {
                self.reasoning
                    .push_str(v["message"]["thinking"].as_str().unwrap_or(""));
                self.text
                    .push_str(v["message"]["content"].as_str().unwrap_or(""));
                if let Some(calls) = v["message"]["tool_calls"].as_array() {
                    for call in calls {
                        let n = self.calls.len();
                        self.calls.insert(
                            n as u64,
                            (
                                format!("local_{n}"),
                                call["function"]["name"].as_str().unwrap_or("").into(),
                                call["function"]["arguments"].to_string(),
                            ),
                        );
                    }
                }
                if v["done"].as_bool() == Some(true) {
                    if v["done_reason"] == "length" {
                        return Err("Output limit reached; no edits proposed".into());
                    }
                    self.done = true;
                }
            }
            _ => {
                if let Some(c) = v["choices"].as_array().and_then(|a| a.first()) {
                    self.reasoning
                        .push_str(c["delta"]["reasoning_content"].as_str().unwrap_or(""));
                    self.text
                        .push_str(c["delta"]["content"].as_str().unwrap_or(""));
                    if let Some(calls) = c["delta"]["tool_calls"].as_array() {
                        for call in calls {
                            let key = call["index"].as_u64().ok_or("Missing tool index")?;
                            let e = self.calls.entry(key).or_default();
                            if let Some(id) = call["id"].as_str() {
                                e.0 = id.into();
                            }
                            if let Some(name) = call["function"]["name"].as_str() {
                                e.1.push_str(name);
                            }
                            if let Some(arg) = call["function"]["arguments"].as_str() {
                                e.2.push_str(arg);
                            }
                        }
                    }
                    match c["finish_reason"].as_str() {
                        Some("length") => {
                            return Err("Output token limit reached; no edits proposed".into());
                        }
                        Some("content_filter") => {
                            return Err(
                                "Provider content filter interrupted response; no edits proposed"
                                    .into(),
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
        if self.text.len()
            + self.reasoning.len()
            + self.calls.values().map(|c| c.2.len()).sum::<usize>()
            > 256 * 1024
            || self.calls.len() > 16
        {
            return Err("Provider output exceeds safety limits".into());
        }
        Ok(())
    }
    pub fn finish(mut self) -> Result<Message, String> {
        if !self.done {
            return Err("Provider stream ended before completion".into());
        }
        let mut calls = vec![];
        let mut seen = std::collections::HashSet::new();
        for (_, (id, name, args)) in self.calls {
            if id.is_empty() || name.is_empty() || !seen.insert(id.clone()) {
                return Err("Invalid or duplicate tool identity".into());
            }
            let arguments: Value = serde_json::from_str(if args.is_empty() { "{}" } else { &args })
                .map_err(|_| "Tool arguments are not valid JSON")?;
            for block in self.anthropic_output.values_mut() {
                if block["type"] == "tool_use" && block["id"] == id {
                    block["input"] = arguments.clone();
                }
            }
            calls.push(Call {
                id,
                name,
                arguments,
            });
        }
        Ok(Message {
            image: None,
            role: "assistant".into(),
            text: self.text,
            calls,
            call_id: None,
            openai_output: self.openai_output,
            anthropic_output: self.anthropic_output.into_values().collect(),
            reasoning_content: (!self.reasoning.is_empty()).then_some(self.reasoning),
        })
    }
}

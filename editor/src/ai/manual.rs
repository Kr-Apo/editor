//! Deterministic, embedded guidance. No files, RAG, network or project strings.
use super::protocol::Message;
pub const VERSION: &str = "ankhimate-ai/2";
pub const MAX_BYTES: usize = 6144;
pub const MAIN: &str = include_str!("../../../docs/ai/ANKHIMATE-AI.md");
pub const TOOLS: &str = include_str!("../../../docs/ai/TOOLS.md");
pub const PROJECT: &str = include_str!("../../../docs/ai/PROJECT.md");
pub const ANIMATION: &str = include_str!("../../../docs/ai/ANIMATION.md");
pub const VISION: &str = include_str!("../../../docs/ai/VISION.md");
pub const RIGGING: &str = include_str!("../../../docs/ai/RIGGING.md");
pub const ASSEMBLY: &str = include_str!("../../../docs/ai/ASSEMBLY.md");
pub const WEIGHTS: &str = include_str!("../../../docs/ai/WEIGHTS.md");
pub const WORKFLOWS: &str = include_str!("../../../docs/ai/WORKFLOWS.md");
pub struct Guidance {
    pub text: String,
    pub sections: Vec<&'static str>,
}
pub fn select(prompt: &str, tools: bool) -> Guidance {
    select_with_budget(prompt, tools, MAX_BYTES)
}
pub fn select_with_budget(prompt: &str, tools: bool, budget: usize) -> Guidance {
    let mut guide = Guidance {
        text: format!("Manual version: {VERSION}\n{MAIN}"),
        sections: vec!["ANKHIMATE-AI"],
    };
    let mut add = |name, text: &str| {
        if guide.text.len() + text.len() + 2 <= budget.min(MAX_BYTES) {
            guide.sections.push(name);
            guide.text.push_str("\n\n");
            guide.text.push_str(text);
        }
    };
    if tools && budget >= MAX_BYTES {
        add("TOOLS", TOOLS);
    }
    let p = prompt.to_lowercase();
    let has = |words: &[&str]| words.iter().any(|word| p.contains(word));
    let weights = has(&[
        "mesh",
        "maillage",
        "weight",
        "poids",
        "liss",
        "smooth",
        "influence",
        "normalis",
    ]);
    if weights {
        add("WEIGHTS", WEIGHTS);
    }
    let assembly = has(&[
        "assembl",
        "attachment",
        "region",
        "slot",
        "accessoire",
        "accessory",
    ]);
    if assembly {
        add("ASSEMBLY", ASSEMBLY);
    }
    if has(&[
        "observe",
        "vision",
        "image",
        "viewport",
        "personnage",
        "capture",
        "voir",
    ]) {
        add("VISION", VISION);
    }
    if !assembly
        && has(&[
            "rig",
            "parent",
            "bone",
            "joint",
            "poids",
            "weight",
            "articul",
            "setup",
            "accessoire",
            "accessory",
        ])
    {
        add("RIGGING", RIGGING);
    }
    if has(&[
        "anim", "key", "clé", "clef", "respir", "breath", "attack", "attaque", "frappe", "walk",
        "marche", "idle",
    ]) {
        add("ANIMATION", ANIMATION);
        let title = if has(&["breath", "respir"]) {
            "Breathing"
        } else if has(&["attack", "attaque", "frappe"]) {
            "Attack"
        } else {
            "Walk or idle"
        };
        if let Some(section) = WORKFLOWS
            .split("\n## ")
            .skip(1)
            .find(|s| s.starts_with(title))
        {
            add(title, section);
        }
    }
    if has(&[
        "rig",
        "bone",
        "squelette",
        "mesh",
        "weight",
        "poids",
        "hiérarch",
        "hierarch",
    ]) {
        add("PROJECT", PROJECT);
    }
    guide
}
pub fn instructions_with_budget(messages: &[Message], tools: bool, budget: usize) -> String {
    let prompt = messages
        .iter()
        .rev()
        .find(|m| m.role == "user")
        .map(|m| m.text.as_str())
        .unwrap_or("");
    if budget >= MAX_BYTES {
        select(prompt, tools).text
    } else {
        select_with_budget(prompt, tools, budget).text
    }
}

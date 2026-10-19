//! User-declared capabilities belong to one exact destination, never a provider name.
use super::protocol::Settings;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Support {
    Supported,
    Unsupported,
    #[default]
    Unknown,
}
impl Support {
    pub const ALL: [Self; 3] = [Self::Unknown, Self::Supported, Self::Unsupported];
    pub fn label(self) -> &'static str {
        match self {
            Self::Supported => "Confirmed by user",
            Self::Unsupported => "Incompatible",
            Self::Unknown => "Unknown / untested",
        }
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Capabilities {
    pub destination: String,
    pub text: Support,
    pub images: Support,
    pub tools: Support,
    pub streaming: Support,
    pub reasoning: Support,
}
impl Settings {
    pub fn destination(&self) -> String {
        serde_json::json!([
            self.provider,
            self.base_url().unwrap_or_default(),
            self.model
        ])
        .to_string()
    }
    pub fn capabilities(&self) -> Capabilities {
        if self.capabilities.destination == self.destination() {
            self.capabilities.clone()
        } else {
            Capabilities::default()
        }
    }
    pub fn validate_capabilities(&self) -> Result<(), String> {
        let caps = self.capabilities();
        if caps.text == Support::Unsupported || caps.streaming == Support::Unsupported {
            return Err("Selected model is declared incompatible with text or streaming. Choose a compatible model; no request sent.".into());
        }
        if self.tools_enabled && caps.tools == Support::Unsupported {
            return Err("Selected model is declared incompatible with tools. Disable Model tools for conversation only.".into());
        }
        Ok(())
    }
    pub fn authorize_image(&self, image: &super::vision::ImageInput) -> Result<(), String> {
        match self.capabilities().images {
            Support::Unsupported=>Err("Selected model is declared incompatible with images. Remove the image to continue with text-only.".into()),
            Support::Unknown if !image.try_unknown=>Err("Vision capability unknown: explicitly authorize an untested image attempt, or remove the image for text-only.".into()),
            _=>Ok(()),
        }
    }
}

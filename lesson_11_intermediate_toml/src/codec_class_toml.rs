use serde::Deserialize;
use std::fs;
use thiserror::Error;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Codec {
    pub name: String,
    pub encoder: String,
    pub container: String,
    pub video: VideoSection,
    pub audio: AudioSection,
    pub custom: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VideoSection {
    pub format: String,
    pub quality: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AudioSection {
    pub codec: String,
    pub quality: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CodecList {
    pub codec: Vec <Codec>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CodecClass {
    Intermediate, Delivery, Broadcast, Archive,
    Streaming, WebOptimized, QuickPresets,
}

#[derive(Debug, Error)]
pub enum CodecLoadError {
    #[error("Failed to read file {0}")]
    ReadError(String),
    #[error("Failed to parse TOML {0}")]
    ParseError(String),
}

pub const CODEC_CLASSES: &[&str] = &[ "Intermediate", "Delivery", "Broadcast", "Archive",
                    "Streaming", "Web Optimized", "Quick Presets"];

pub type CodecRegistry = HashMap<String, Vec<Codec>>;

pub fn load_all_from_file () -> Result<CodecRegistry, CodecLoadError> {
    let mut registry: CodecRegistry = HashMap::new();
    for class in CODEC_CLASSES {
        let filename = format!("assets/toml/{}.toml", class.to_lowercase().replace(' ', "_"));
        let codecs = load_from_file(&filename)?;
        registry.insert(class.to_string(), codecs);
    }
    Ok(registry)
}

pub fn load_from_file (path: &str) -> Result<Vec<Codec>, CodecLoadError> {
    let data = fs::read_to_string(path)
                .map_err(|e| CodecLoadError::ReadError(e.to_string()))?;

    let root: CodecList = toml::from_str::<CodecList>(&data)
                .map_err(|e| CodecLoadError::ParseError(e.to_string()))?;

    Ok(root.codec)
}


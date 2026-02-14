use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use wasmparser::{Parser, Payload};

use crate::metadata::Metadata;
use crate::runtime::Runtime;
use crate::sdk::MAGREXT_METADATA_SECTION;

/// modの実体
#[derive(Debug, Clone)]
pub struct Mod {
    /// modのファイルパス
    path: PathBuf,
    /// modのメタデータ
    metadata: Metadata,
    /// modのバイトコード
    bytes: Vec<u8>,
}

impl Mod {
    /// 指定されたパスからmodを読み込む
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let bytes = fs::read(&path)
            .with_context(|| format!("failed to read wasm file: {}", path.display()))?;
        let metadata = extract_metadata(&bytes)?;
        Ok(Self {
            path,
            metadata,
            bytes,
        })
    }

    /// modのメタデータを取得する
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// modの名前を取得する
    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    /// modのバージョンを取得する
    pub fn version(&self) -> &str {
        &self.metadata.version
    }

    /// modの作者を取得する
    pub fn author(&self) -> &str {
        &self.metadata.author
    }
    
    pub fn license(&self) -> Option<&str> {
        self.metadata.license.as_deref()
    }

    /// modのファイルパスを取得する
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// modのバイトコードを取得する
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// mod内の関数を呼び出す
    pub fn call(&self, func: &str) -> Result<()> {
        let runtime = Runtime::new()?;
        runtime.call_bytes(&self.bytes, func)
    }
}

/// wasmバイトコードからmodのメタデータを抽出する
fn extract_metadata(bytes: &[u8]) -> Result<Metadata> {
    let mut found: Option<Vec<u8>> = None;
    for payload in Parser::new(0).parse_all(bytes) {
        match payload.context("failed to parse wasm payload")? {
            Payload::CustomSection(section) => {
                if section.name() == MAGREXT_METADATA_SECTION {
                    found = Some(section.data().to_vec());
                    break;
                }
            }
            _ => {}
        }
    }

    let bytes = found.ok_or_else(|| {
        anyhow::anyhow!("missing custom section: {}", MAGREXT_METADATA_SECTION)
    })?;
    let metadata = Metadata::from_json_bytes(&bytes)?;

    if metadata.name.is_empty() || metadata.version.is_empty() || metadata.author.is_empty() {
        bail!("metadata is missing required fields");
    }

    Ok(metadata)
}


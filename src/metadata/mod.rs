use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// modのメタデータを表す構造体
#[derive(Serialize, Deserialize, Default,)]
pub struct Metadata {
    /// modの名前
    pub name: String,
    /// modのバージョン
    pub version: String,
    /// modの作者
    pub author: String,
    /// 最低で必要なホスト側のバージョン
    pub min_host_version: Option<String>,
    /// modの説明
    pub description: Option<String>,
    /// modのライセンス
    pub license: Option<String>,
}

impl Metadata {
    /// JSON形式のバイト列からMetadataを生成する
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self> {
        let metadata: Metadata = serde_json::from_slice(bytes)
            .context("failed to parse magrext metadata json")?;
        Ok(metadata)
    }
}

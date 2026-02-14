use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::mods::Mod;

/// modのコレクションを表す構造体
#[derive(Debug, Default)]
pub struct Nest {
    /// modのルートディレクトリ
    root: PathBuf,
    /// 収集されたmodのリスト
    mods: Vec<Mod>,
}

impl Nest {
    /// 指定されたパスからNestを生成する
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            root: path.into(),
            mods: Vec::new(),
        }
    }
    
    /// modを収集する
    pub fn collect(&mut self) -> Result<()> {
        self.mods.clear();
        let root = self.root.clone();
        self.collect_from(&root)
    }

    /// 収集されたmodのイテレータを返す
    pub fn mods(&self) -> impl Iterator<Item = &Mod> {
        self.mods.iter()
    }

    /// 指定されたディレクトリからmodを再帰的に収集する
    fn collect_from(&mut self, path: &Path) -> Result<()> {
        let entries = fs::read_dir(path)
            .with_context(|| format!("failed to read directory: {}", path.display()))?;
        for entry in entries {
            let entry = entry.context("failed to read directory entry")?;
            let path = entry.path();
            if path.is_dir() {
                self.collect_from(&path)?;
                continue;
            }

            let is_wasm = path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("wasm"))
                .unwrap_or(false);
            if !is_wasm {
                continue;
            }

            let mods = Mod::load(&path)
                .with_context(|| format!("failed to load mod: {}", path.display()))?;
            self.mods.push(mods);
        }

        Ok(())
    }
}

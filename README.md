# magrext
magrext（Magpie Rust Extension)はRustでWASMベースのmodを簡単に作成したり、modを開発できる環境をホストアプリ側から提供するためのライブラリです。

## 特徴
* **自動検知**: 複雑な設定なしで、フォルダ内の有効な Mod をスキャン。
* **WASMサンドボックス上での実行**: Modがホストアプリに悪影響を与えることなく、安全に機能を提供。
* **メタデータ解析**: カスタムセクションを利用して、Mod名、バージョン、作者情報をロード前に取得。
* **型安全な対話**: ホストとゲスト（Mod）間のやり取りを直感的に定義可能。

### アプリ本体（ホスト）の実装
```rust
use magrext::Nest;

fn main() -> anyhow::Result<()> {
    // Modを保管しているディレクトリ
    let mut nest = Nest::new("./mods");

    // ディレクトリからMod一覧を取得
    nest.collect()?;
    
    // Modを起動
    for mods in nest.mods() {
        println!("Found mod: {} v{}", mods.name(), mods.version());
        mods.call("init")?;
    }

    Ok(())
}

```

## Modの作り方
`magrext-sdk`（開発中）を使用すると、以下のように簡単にModを作成できます。
```rust
#[magrext::shiny_mod]
pub fn init() {
    println!("Hello from the shiny mod!");
}
```

## ロードマップ
* [ ] WASM 内部のカスタムセクションによるシグネチャ検知
* [ ] ホットリロード機能（ファイル追加時に即座に検知）
* [ ] Mod 間の依存関係解決
* [ ] 権限管理（ファイルアクセスやネットワークの制限）

## 貢献
`magrext` は、Mod文化をRustエコシステムに持ち込むためのプロジェクトです。
バグ報告、機能提案、プルリクエストは大歓迎です！
#[macro_export]
macro_rules! magrext_metadata {
    ($json_bytes:expr) => {
        #[cfg(target_arch = "wasm32")]
        #[link_section = concat!(".custom_section.", $crate::sdk::MAGREXT_METADATA_SECTION)]
        #[used]
        static MAGREXT_METADATA: [u8; $json_bytes.len()] = *$json_bytes;
    };
}

/// modのメタデータを格納するカスタムセクションの名前
pub const MAGREXT_METADATA_SECTION: &str = "magrext.meta";

pub use crate::magrext_metadata;
pub use magrext_macros::shiny_mod;

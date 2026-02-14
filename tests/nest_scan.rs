use std::fs;

use magrext::nest::Nest;
use magrext::sdk::MAGREXT_METADATA_SECTION;
use tempfile::tempdir;

fn encode_u32(mut value: u32, out: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

fn wasm_with_custom_section(name: &str, data: &[u8]) -> Vec<u8> {
    let mut section = Vec::new();
    encode_u32(name.len() as u32, &mut section);
    section.extend_from_slice(name.as_bytes());
    section.extend_from_slice(data);

    let mut bytes = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
    bytes.push(0x00);

    let mut size = Vec::new();
    encode_u32(section.len() as u32, &mut size);
    bytes.extend_from_slice(&size);
    bytes.extend_from_slice(&section);

    bytes
}

#[test]
fn nest_collects_mods() {
    let dir = tempdir().unwrap();
    let json = br#"{"name":"demo","version":"1.0.0","author":"me"}"#;
    let wasm = wasm_with_custom_section(MAGREXT_METADATA_SECTION, json);
    let path = dir.path().join("demo.wasm");
    fs::write(&path, wasm).unwrap();

    let mut nest = Nest::new(dir.path());
    nest.collect().unwrap();

    let mods: Vec<_> = nest.mods().collect();
    assert_eq!(mods.len(), 1);
    assert_eq!(mods[0].name(), "demo");
}

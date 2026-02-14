use anyhow::Result;
use wasmtime::{Engine, Instance, Module, Store};

/// modの実行環境を表す構造体
#[derive(Debug)]
pub struct Runtime {
    /// WebAssemblyの実行エンジン
    engine: Engine,
}

impl Runtime {
    /// Runtimeを生成する
    pub fn new() -> Result<Self> {
        Ok(Self {
            engine: Engine::default(),
        })
    }

    /// WebAssemblyのバイトコードを実行する
    pub fn call_bytes(&self, wasm_bytes: &[u8], func: &str) -> Result<()> {
        let module = Module::from_binary(&self.engine, wasm_bytes)?;
        let mut store = Store::new(&self.engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;
        let function = instance.get_typed_func::<(), ()>(&mut store, func)?;
        function.call(&mut store, ())?;
        Ok(())
    }
}

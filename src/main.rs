use std::error::Error;

use wasm::CardanoBlockParam;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = include_bytes!("../hello-cardano.wasm");

    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let component = Component::from_binary(&engine, bytes)?;

    let (wit_hermes, _) = wasm::Hermes::instantiate(&mut store, &component, &linker).unwrap();

    wit_hermes
        .call_on_new_cardano(
            &mut store,
            CardanoBlockParam {
                slot: 123,
                hash: &[0; 32],
                raw_bytes: &[1; 32],
            },
        )
        .unwrap();

    wit_hermes
        .call_on_new_ethereum(
            &mut store,
            "{\"jsonrpc\":\"2.0\",\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1}",
        )
        .unwrap();

    Ok(())
}

mod wasm {
    wasmtime::component::bindgen!("hermes");

    impl host::Host for Hermes {
        fn tweet(&mut self, msg: String) -> wasmtime::Result<()> {
            println!("tweeting: {msg}");
            Ok(())
        }

        fn publish(&mut self, msg: String) -> wasmtime::Result<()> {
            println!("publishing: {msg}");
            Ok(())
        }

        fn blake_hash(&mut self, bytes: Vec<u8>) -> wasmtime::Result<Vec<u8>> {
            Ok(vec![1, 2, 3, 4, 5])
        }
    }
}

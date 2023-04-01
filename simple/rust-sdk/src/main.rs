use extism::{Context, Plugin, Function};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Result {
    count: i32,
    input: String,
}

fn main() {
    // initialize plugin
    let wasm = include_bytes!("../../simple.wasm");
    let context = Context::new();
    let functions: &[Function] = &[];
    let mut plugin = Plugin::new(&context, wasm, functions, true).unwrap();

    // call plugin and parse json
    let data = plugin.call("count_vowels", "this is a text from rust!").unwrap();
    let result: Result = serde_json::from_slice(data).unwrap();
    println!("{:?}", result);
}

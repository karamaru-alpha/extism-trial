use extism::{Context, Plugin, Function, Val, CurrentPlugin, UserData, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    vowel_count: i32,
    input: String,
}

fn main() {
    // initialize plugin
    let wasm = include_bytes!("../../simple.wasm");
    let context = Context::new();
    let f = Function::new(
        "host_function",
        [],
        [],
        None,
        host_function,
    );
    let functions: &[Function] = &[f];
    let mut plugin = Plugin::new(&context, wasm, functions, true).unwrap();

    // call plugin and parse json
    let data = plugin.call("count_vowels", "This text from rust to plugin!").unwrap();
    let result: Response = serde_json::from_slice(data).unwrap();
    println!("{:?}", result);
}


fn host_function(
    _: &mut CurrentPlugin,
    _: &[Val],
    _: &mut [Val],
    _: UserData,
) -> Result<(), Error> {
    println!("Hello from host function!");
    Ok(())
}

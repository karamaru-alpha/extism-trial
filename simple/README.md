Simple
==

This is an example of code that calls a wasm plugin (vowel counting) created in go from rust.

Requires extism, rust, tinugo install. [doc](https://extism.org/docs/install)

```shell
## create wasm plugin using go-pdk and tinygo
$ make build

## call wasm plugin using rust-sdk
$ make run
-> Result { count: 6, input: "this is a text from rust!" }
```

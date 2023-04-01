
This is a repository to try [exitsm](https://extism.org).

- Create a plugin in tinygo that counts vowels in passed text and call it from Rust
- Call host function from plugin.

```shell
## create wasm plugin using go-pdk and tinygo
$ make build

## call wasm plugin using rust-sdk
$ make run
Call host function!
Response { vowel_count: 7, input: "This text from rust to plugin!" }
```

Requires extism, rust, tinygo install. [doc](https://extism.org/docs/install)


This is a repository to try [Exitsm](https://extism.org).

- Create a plugin in TinyGo that counts vowels in passed text and call it from Rust
- Call host function from plugin

```shell
## create wasm plugin using TinyGo.
$ make build

## call wasm plugin from Rust.
$ make run
Call host function!
Response { vowel_count: 7, input: "This text from rust to plugin!" }
```

Requires extism, rust, tinygo install.

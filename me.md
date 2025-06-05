

```shell
cargo build --target wasm32-unknown-unknown --release
wasm-tools component new ./target/wasm32-unknown-unknown/release/onnx_basic_test.wasm -o ./onnx_basic_test_component.wasm
```



copy onnx_basic_test_component.wasm to root of [graphtime](https://github.com/MendyBerger/onnx-basic-test/blob/main/me.md)

run in root of graphtime
```shell
cargo run ./onnx_basic_test_component.wasm
```




`wasm-tools print ./onnx_basic_test_component.wasm  > out.wat`




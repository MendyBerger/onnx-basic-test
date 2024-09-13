

```shell
cargo build --target wasm32-unknown-unknown --release
wasm-tools component new ./target/wasm32-unknown-unknown/release/onnx_basic_test.wasm -o ./onnx_basic_test_component.wasm
```



copy onnx_basic_test_component.wasm to root of wgpu

run in root of wgpu
```shell
cargo +1.77 run --bin wgpu-examples-wasi --features="wasi-runtime" hello_compute
```




`wasm-tools print ./onnx_basic_test_component.wasm  > out.wat`




use std::collections::HashMap;

use wonnx::{
    utils::{attribute, graph, initializer, model, node, tensor, OutputTensor},
    SessionError, WonnxError,
};


pub async fn run() -> Result<(), WonnxError> {
    log::info!("Hello, world!");
    log::info!("IN");

    let result = execute_gpu().await?;
    log::info!("Result: {:?}", result);
    let result = result.into_iter().next().unwrap().1;

    log::info!("Result: {:?}", result);

    assert_eq!(
        result,
        OutputTensor::F32(vec![54., 63., 72., 99., 108., 117., 144., 153., 162.])
    );
    Ok(())
}

// Hardware management
async fn execute_gpu() -> Result<HashMap<String, OutputTensor>, SessionError> {
    // USER INPUT
    let n = 5;
    let c = 1;
    let mut input_data = HashMap::new();

    let data: Vec<f32> = (0..25).map(|x| x as f32).collect();
    input_data.insert("X".to_string(), data.as_slice().into());

    // ONNX INPUTS
    let shape = vec![1, c, n as i64, n as i64];
    let kernel_n = 3;
    let m = 1;
    let data_w: Vec<f32> = (0..m * c * kernel_n * kernel_n).map(|_| 1.0f32).collect();
    let model = model(graph(
        vec![tensor("X", &shape)],
        vec![tensor("Y", &[1, m, 3, 3])],
        vec![],
        vec![initializer("W", data_w, vec![m, c, 3, 3])],
        vec![node(
            vec!["X", "W"],
            vec!["Y"],
            "conv",
            "Conv",
            vec![attribute("kernel_shape", vec![3, 3])],
        )],
    ));

    // LOGIC

    let session = wonnx::Session::from_model(model)
        .await
        .expect("Session did not create");

    session.run(&input_data).await
}

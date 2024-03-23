use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in JSON format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    a: i32,
    b: i32,
    c: i32,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into JSON. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    sum: i32,
    product: i32,
    average: f64,
}

/// This is the main body for the function.
/// Write your code inside it.
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract integers from the request
    let a = event.payload.a;
    let b = event.payload.b;
    let c = event.payload.c;

    // Perform some data processing
    let sum = a + b + c;
    let product = a * b * c;
    let average = (sum as f64) / 3.0;

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id.clone(),
        sum,
        product,
        average,
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

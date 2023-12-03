use axum::{extract::Path, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn calculate(params: Path<(u64, u64)>) -> String {
    let nums = params.0;
    let (num1, num2) = nums;

    let result = (num1 ^ num2).pow(3);

    format!("{}", result)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/:num1/:num2", get(calculate))
        .route("/", get(hello_world));

    Ok(router.into())
}

use cr_backend::dispatch_event;
use lambda_runtime::{service_fn, Error as LambdaError};

// Where all the magic starts
#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = service_fn(dispatch_event);
    lambda_runtime::run(func).await?;
    Ok(())
}

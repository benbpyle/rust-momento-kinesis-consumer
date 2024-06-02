mod models;

use aws_lambda_events::kinesis::{KinesisEvent, KinesisEventRecord};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::info;

async fn function_handler(event: LambdaEvent<KinesisEvent>) -> Result<(), Error> {
    for e in event.payload.records {
        info!("(Event)={}", e);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
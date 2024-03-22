use lambda_runtime::{handler_fn, Context, Error};
use serde_derive::{Deserialize, Serialize};
use log::{info, error};
use simple_logger::SimpleLogger;

#[derive(Deserialize, Clone)]
pub struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
    original_name: String,
}

#[derive(Serialize, Clone)]
pub struct CustomOutput {
    message: String,
}

async fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput, Error> {
    if e.first_name.is_empty() {
        error!("Missing first name");
        return Err(Error::from("Missing first name"));
    }

    let message = format!("Hello, {}! the reverse of your name is {}!", e.original_name, e.first_name);
    info!("{}", message);

    Ok(CustomOutput {
        message,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();

    if std::env::var("AWS_EXECUTION_ENV").is_ok() {
        let handler = handler_fn(my_handler);
        lambda_runtime::run(handler).await?;
    } else {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            return Err(Error::from("Missing first name"));
        }

        let reversed_name = args[1].chars().rev().collect::<String>();

        let event = CustomEvent {
            first_name: reversed_name,
            original_name: args[1].clone(),
        };

        let ctx = lambda_runtime::Context::default();

        let result = my_handler(event, ctx).await;

        match result {
            Ok(output) => println!("{}", output.message),
            Err(error) => eprintln!("Error: {}", error),
        }
    }

    Ok(())
}
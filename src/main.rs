use clap::Parser;
use http_getty_lib::cli::{Cli, Commands};
use http_getty_lib::core::{Body, Requester};
use http_getty_lib::error::HttpGettyError;
use http_getty_lib::write_file;
use serde_json::Value;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), HttpGettyError> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Get { url, output } => {
            let req = Requester::new(url.clone(), None, None, "get".to_owned())?;
            let response = req.request().await?;
            if let Body::Binary(b) = response {
                if let Some(a) = output {
                    write_file(b, a)?;
                    println!("Done writing the file.");
                    return Ok(());
                } else {
                    return Err(HttpGettyError::NoFileForBinaryOutput);
                }
            }

            if let Some(p) = output {
                write_file(response.to_string().as_bytes().to_vec(), p)?;
            } else {
                if let Body::Json(v) = response {
                    let json: Value = serde_json::from_str(&v).unwrap();
                    println!("{}\n{}", "JSON OUTPUT".white().bold(), v.bold().green());

                } else {
                    println!("{}\n{}", "OUTPUT".white().bold(), response.to_string())

                }
            }
        }
    }
    Ok(())
}

use clap::{Parser, Subcommand, Args};


#[derive(Parser)]
#[clap(name = "http-getty")]
#[clap(author = "Haider Ali")]
#[clap(version = "0.0.1")]
#[clap(about = "A command line application for making http requests")]

pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Used to send get requests to http servers.
    /// - For binary outputs you must use the `-o` or `--output` flag to write the bytes into a file
    
    /// Example: `http-getty get "https://haider-ali.xyz/assets/face.png" -o foo.png`
    /// Example: `http-getty get "https://google.com"`
    Get {
        #[clap(value_parser)]
        /// Url from where you want to get the data.
        url: String,
        #[clap(long, short)]
        /// Filename or file path to save the output, this flag is needed for binary outputs.
        output: Option<String>
    }
}

#[derive(Args)]
pub struct Get {
    #[clap(value_parser)]
    url: String,
    #[clap(long, short)]
    output: String,
}


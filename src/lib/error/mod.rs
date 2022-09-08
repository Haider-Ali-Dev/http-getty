

#[derive(Debug, thiserror::Error)]
pub enum HttpGettyError {
    #[error("http-getty is not able to parse the url provided.")]
    UrlParseError(#[from] url::ParseError),
    #[error("Output is in binary but there is no `--output` flag to write the stream")]
    NoFileForBinaryOutput,
    #[error("Invalid method provided to http-getty.")]
    InvalidMethod,
    #[error("Error while sending http request.")]
    RequestError(#[from] reqwest::Error),
    #[error("This response type is not supported by http-getty, create an issue in the repo to make it supported.")]
    UnSupportedResponse,
    #[error("Cannot convert the following body to raw.")]
    CannotConvertToRaw,
    #[error("There is no body provided for a post request")]
    NoBodyForPost,
    #[error("Cannot create file.")]
    FileError(#[from] std::io::Error)
}
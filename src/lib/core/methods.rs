use http::method::Method;
pub enum SupportedMethods {
    Post,
    Get,
    Put,
    // UnSupported Ones
    Other
}


impl From<Method> for SupportedMethods {
    fn from(method: Method) -> Self {
        match method {
            Method::POST => Self::Post,
            Method::GET => Self::Get,
            Method::PUT => Self::Put,
            // This will later raise an error.
            _ => Self::Other
        }
    }
}
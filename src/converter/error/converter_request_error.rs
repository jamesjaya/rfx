use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ConverterRequestError {
  FailedToSendRequest,
  NoResponseBody,
  FailedToParseResponse,
  InvalidResponse,
}

impl fmt::Display for ConverterRequestError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let text = match *self {
      ConverterRequestError::FailedToSendRequest => "Failed to send request",
      ConverterRequestError::NoResponseBody => "Response does not contain body",
      ConverterRequestError::FailedToParseResponse => "Unexpected response body",
      ConverterRequestError::InvalidResponse => "Wrong response format",
    };

    return write!(f, "{}", text);
  }
}

impl error::Error for ConverterRequestError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

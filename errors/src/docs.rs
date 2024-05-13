use std::error::Error;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
  2
}

#[derive(Debug)]
enum DocumentServiceError {
  RateLimitExceeded,
  IoError(io::Error),
}

impl Error for DocumentServiceError {
  // description() is optional since Rust 1.27.
  fn description(&self) -> &str {
    use DocumentServiceError::*;
    match *self {
      RateLimitExceeded => "rate limit exceeded",
      IoError(_) => "I/O error"
    }
  }
}

impl fmt::Display for DocumentServiceError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use DocumentServiceError::*;
    match *self {
      RateLimitExceeded => write!(f, "You have exceeded the allowed number of documents created per minute"),
      IoError(ref io) => write!(f, "I/O error: {}", io)
    }
  }
}

impl From<io::Error> for DocumentServiceError {
  fn from(o: io::Error) -> Self {
    DocumentServiceError::IoError(o)
  }
}

fn create_document(file_name: &str) -> Result<File, DocumentServiceError> {
  if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
    return Err(DocumentServiceError::RateLimitExceeded);
  }

  let file = OpenOptions::new()
    .write(true)
    .create_new(true)
    .open(file_name)?;

  Ok(file)
}

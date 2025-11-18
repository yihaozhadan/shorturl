use url::Url;

#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    Empty,
    InvalidFormat,
}

pub fn validate_url(input: &str) -> Result<(), ValidationError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ValidationError::Empty);
    }

    match Url::parse(trimmed) {
        Ok(parsed) => match parsed.scheme() {
            "http" | "https" => Ok(()),
            _ => Err(ValidationError::InvalidFormat),
        },
        Err(_) => Err(ValidationError::InvalidFormat),
    }
}

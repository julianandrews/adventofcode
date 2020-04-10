#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AOCError {
    message: String,
}

impl AOCError {
    pub fn new(message: &str) -> AOCError {
        AOCError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for AOCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occured: {}", self.message)
    }
}

impl std::error::Error for AOCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

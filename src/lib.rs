pub trait Config: Sized {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>>;
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>>;
    fn validate(&self) -> Result<(), Vec<String>> { Ok(()) }
}

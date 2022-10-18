#[derive(Debug)]
pub struct Message {
    pub sender: String, // User type?
    pub content: String,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.sender, self.content)
    }
}

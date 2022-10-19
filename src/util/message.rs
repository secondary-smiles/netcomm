#[derive(Debug)]
pub struct Message {
    pub sender: String, // User type?
    pub content: String,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.content.replace('\n', "\r\n");
        write!(f, "\x1b[1m{}:\x1b[0m {}", self.sender, message)
    }
}

impl Message {
    pub fn new(sender: &String, content: &String) -> Self {
        Self {
            sender: sender.to_string(),
            content: content.to_string(),
        }
    }
}
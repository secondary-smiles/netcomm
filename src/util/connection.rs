#[derive(Clone, Debug)]
pub struct Connection {
    pub domain: Option<String>,
    pub port: Option<u16>,
}
use crate::config::Config;
use failure::Error;

pub struct Server {}

impl Server {
    pub fn new(config: &Config) -> Result<Self, Error> {
        Ok(Server {})
    }

    pub fn start(self) -> i32 {
        println!("start server...");
        0
    }
}

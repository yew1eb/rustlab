use crate::config::Config;
use failure::Error;

use actix::prelude::*;
use actix::SystemRunner;
use actix_web::{get, web, App, HttpServer, Responder};


pub struct Server {
    runner: SystemRunner,
}

impl Server {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let runner = actix::System::new("apiserver");

        HttpServer::new(move || app()).bind(config.server.port)?.start();

        Ok(Server { runner })
    }

    pub fn start(self) {
        println!("start server...");
        self.runner.run();
    }
}

fn app() {}

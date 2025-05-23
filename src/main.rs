#![allow(unused)]

use anyhow::Result;
use env_logger_wrapper::{LevelFilter, new_builder};

#[tokio::main]
async fn main() -> Result<()> {
	dotenv::dotenv().ok(); // load .env file
	new_builder(LevelFilter::Trace).init(); // init logging

	println!("Hello, world!");

	Ok(())
}

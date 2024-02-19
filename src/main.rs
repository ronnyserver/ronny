use std::error::Error;

use crate::cmd::entry;

pub mod cmd;
pub mod http;
pub mod io;
pub mod modules;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    entry::entry().await
}

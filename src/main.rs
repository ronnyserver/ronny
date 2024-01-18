use crate::cmd::entry;

pub mod cmd;
pub mod http;
pub mod io;
pub mod modules;

#[tokio::main]
async fn main() {
    entry::entry().await;
}

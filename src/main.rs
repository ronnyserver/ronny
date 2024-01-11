use crate::cmd::entry;

pub mod cmd;
pub mod io;

#[tokio::main]
async fn main() {
    entry::entry().await;
}

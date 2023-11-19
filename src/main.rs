use crates_io_api::AsyncClient;
use rustina::{
    handler,
    utils::{github::GitHub, groups::Groups, resources::Resources},
};
use std::error::Error;
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting Rustina Assistant...");

    let bot = Bot::from_env();

    let groups = Groups::new();
    let github = GitHub::new();
    let crates_client = AsyncClient::new(
        "Rustina Assistant (rust@maid.uz)",
        std::time::Duration::from_millis(100),
    )
    .unwrap();
    let resources = Resources::new();

    Dispatcher::builder(bot, handler())
        .dependencies(dptree::deps![crates_client, github, groups, resources])
        // If no handler succeeded to handle an update, this closure will be called
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        // If the dispatcher fails for some reason, execute this handler
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

use crate::functions;
use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", parse_with = "split")]
#[command(description = "These are the commands that I can understand:")]
pub enum Command {
    /// List existing commands
    Help,

    /// Starting point of the bot
    Start,

    /// Rules of our chat
    Rules,

    /// About the bot
    About,

    /// Available groups
    Group,

    /// Latest version
    Latest,

    /// Specific version
    Version,

    /// Report offtopic
    Warn,

    /// Useful resources
    Useful,

    /// Roadmap for newbies,
    Roadmap,

    /// Check for chatid
    Check,
}

pub fn handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry()
        // Inline Queries
        .branch(Update::filter_inline_query().endpoint(functions::inline))
        // Callbacks
        .branch(Update::filter_callback_query().endpoint(functions::callback))
        // Commands
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(functions::commands),
        )
        .branch(Update::filter_message().endpoint(functions::triggers))
}

pub fn dispatch(
    bot: &Bot,
    deps: DependencyMap,
) -> Dispatcher<Bot, Box<dyn std::error::Error + Send + Sync>, teloxide::dispatching::DefaultKey> {
    Dispatcher::builder(bot.clone(), handler())
        .dependencies(deps) // dptree::deps![...]
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
}

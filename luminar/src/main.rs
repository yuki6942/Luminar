mod commands;
mod events;
mod utils;

use crate::commands::{
    general::{about, help, userinfo},
    owner::{commands, register, shutdown},
};
use crate::utils::{
    command::{post_command, pre_command},
    luminar::LuminarData,
};

use crate::events::{error::on_error, event_handler::event_handler};

use dotenv::dotenv;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        commands: vec![help(), register(), commands(), about(), userinfo(), shutdown()],
        event_handler: |ctx, event, framework, user_data| {
            Box::pin(event_handler(ctx, event, framework, user_data))
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| Box::pin(pre_command(ctx)),
        post_command: |ctx| Box::pin(post_command(ctx)),

        // Options specific to prefix commands, i.e. commands invoked via chat messages
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(String::from("~")),

            mention_as_prefix: false,
            // An edit tracker needs to be supplied here to make edit tracking in commands work
            edit_tracker: Some(poise::EditTracker::for_timespan(
                std::time::Duration::from_secs(3600 * 3),
            )),
            ..Default::default()
        },

        ..Default::default()
    };
    dotenv().ok();
    poise::Framework::builder()
        .token(std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment"))
        .intents(serenity::GatewayIntents::all())
        .options(options)
        .setup(|_ctx, _data_about_bot, _framework| {
            Box::pin(async move {
                Ok(LuminarData {
                    command_counter: std::sync::Mutex::new(std::collections::HashMap::new()),
                })
            })
        })
        .run()
        .await
        .expect("Client error");
}

use crate::utils::luminar::{LuminarData, LuminarError};
use text_colorizer::*;

pub async fn on_error(error: poise::FrameworkError<'_, LuminarData, LuminarError>) {
    match error {
        poise::FrameworkError::Command { error, ctx } => {
            eprintln!(
                "'{}:' Command '{}' returned error {:?}",
                "Error:".red().bold(),
                ctx.command().name,
                error
            );
        }
        poise::FrameworkError::EventHandler { error, event, .. } => {
            eprintln!(
                "'{}:' EventHandler returned error during {:?} event: {:?}",
                "Error:".red().bold(),
                event.name(),
                error
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                eprintln!("{} while handling error: {}", "Error:".red().bold(), e)
            }
        }
    }
}

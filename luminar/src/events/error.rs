use crate::utils::luminar::{LuminarData, LuminarError};

pub async fn on_error(error: poise::FrameworkError<'_, LuminarData, LuminarError>) {
    match error {
        poise::FrameworkError::Command { error, ctx } => {
            println!(
                "Command '{}' returned error {:?}",
                ctx.command().name,
                error
            );
        }
        poise::FrameworkError::EventHandler { error, event, .. } => {
            println!(
                "EventHandler returned error during {:?} event: {:?}",
                event.name(),
                error
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

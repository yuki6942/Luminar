use crate::utils::checks::owner_check;
use crate::utils::luminar::{LuminarContext, LuminarResult};
use std::fmt::Write as _;

#[poise::command(prefix_command, hide_in_help, check = "owner_check")]
/// Shows how often each command was used
pub async fn commands(ctx: LuminarContext<'_>) -> LuminarResult {
    let mut contents = "Commands used:\n".to_string();

    for (k, v) in &*ctx.data().command_counter.lock().unwrap() {
        writeln!(contents, "- {name}: {amount}", name = k, amount = v)?;
    }

    ctx.say(contents).await?;

    Ok(())
}

/// Registers slash commands in this guild or globally
#[poise::command(prefix_command, hide_in_help, check = "owner_check")]
pub async fn register(ctx: LuminarContext<'_>) -> LuminarResult {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}

#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn shutdown(ctx: LuminarContext<'_>) -> LuminarResult {
    ctx.say("Shutting down").await?;
    ctx.framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}

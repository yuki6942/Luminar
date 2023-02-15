use crate::utils::luminar::{LuminarContext, LuminarError};
use crate::utils::checks::{owner_check};
use std::fmt::Write as _;

#[poise::command(prefix_command, slash_command, category = "General", check="owner_check")]
/// Shows how often each command was used
pub async fn commands(ctx: LuminarContext<'_>) -> Result<(), LuminarError> {
    let mut contents = "Commands used:\n".to_string();

    for (k, v) in &*ctx.data().command_counter.lock().unwrap() {
        writeln!(contents, "- {name}: {amount}", name = k, amount = v)?;
    }

    ctx.say(contents).await?;

    Ok(())
}
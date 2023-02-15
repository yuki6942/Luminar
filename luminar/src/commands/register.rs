use crate::utils::{
    luminar::{LuminarContext, LuminarError},
    checks::owner_check
};

/// Registers slash commands in this guild or globally
#[poise::command(prefix_command, hide_in_help, check="owner_check")]
pub async fn register(ctx: LuminarContext<'_>) -> Result<(), LuminarError> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}

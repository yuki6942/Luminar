use crate::utils::luminar::{LuminarContext, LuminarError};

// Show a help menu
#[poise::command(prefix_command, slash_command)]
pub async fn help(
    ctx: LuminarContext<'_>,
    #[description = "Command to display specific information about"] command: Option<String>,
) -> Result<(), LuminarError> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Hello! こんにちは！Hola! Bonjour! 您好! 안녕하세요~

If you want more information about a specific command, just pass the command as argument.",
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
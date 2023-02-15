use crate::utils::luminar::{LuminarContext, LuminarResult};
use poise::serenity_prelude::{self as serenit};
use serenity::utils::Colour;

#[poise::command(prefix_command, slash_command, category = "General")]
/// Shows some information about Luminar
pub async fn about(ctx: LuminarContext<'_>) -> LuminarResult {
    let version: &str = env!("CARGO_PKG_VERSION");
    let authors: &str = env!("CARGO_PKG_AUTHORS");

    ctx.send(|b| {
        b.embed(|b| {
            b.title("About Luminar")
                .fields(vec![
                    ("__Version__", version, true),
                    ("__Library__", "Serenity, poise", true),
                    ("__Language__", "Rust", true),
                    ("__Authors__", authors, true),
                ])
                .colour(Colour::FADED_PURPLE)
        })
        .components(|b| {
            b.create_action_row(|b| {
                b.create_button(|b| {
                    b.label("Support")
                        .url("https://discord.gg/EnEDSYWvUm")
                        .style(serenit::ButtonStyle::Link)
                })
                .create_button(|b| {
                    b.label("GitHub")
                        .url("https://github.com/yuki6942/Luminar")
                        .style(serenit::ButtonStyle::Link)
                })
            })
        })
    })
    .await?;

    Ok(())
}

// Show a help menu
#[poise::command(prefix_command, slash_command, category = "General")]
/// Shows the help menu
pub async fn help(
    ctx: LuminarContext<'_>,
    #[description = "Command to display specific information about"] command: Option<String>,
) -> LuminarResult {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Hello! こんにちは！Hola! Bonjour! 您好! 안녕하세요~
If you want more information about a specific command, just pass the command as argument.",
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn userinfo(
    ctx: LuminarContext<'_>,
    #[description = "The user to get the information from"] user: Option<serenit::User>,
) -> LuminarResult {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    ctx.send(|e| {
        e.embed(|e| {
            e.title(format!("Information about {}", u.name))
                .colour(Colour::FADED_PURPLE)
                .fields(vec![
                    ("Username", &u.name, true),
                    ("ID", &u.id.to_string(), true),
                    ("Bot", &u.bot.to_string(), true),
                ])
                .field(
                    "Created at",
                    format!("<t:{}:R>", u.created_at().timestamp()),
                    true,
                )
                .thumbnail(u.avatar_url().unwrap_or_else(|| u.default_avatar_url()))
        })
    })
    .await?;
    Ok(())
}

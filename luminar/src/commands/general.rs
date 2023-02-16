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
                    ("__Version__", version, false),
                    ("__Library__", "Serenity", false),
                    ("__Command Framework__", "Poise", false),
                    ("__Language__", "Rust", false),
                    ("__Authors__", authors, false),
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
    let bot_str = if u.bot { "Yes" } else { "No" }.to_owned();
    let created_at = format!("<t:{}:R>", &u.created_at().timestamp()).to_owned();

    let mut author = serenit::CreateEmbedAuthor::default();
    author.name(&u.name);
    author.icon_url(u.avatar_url().unwrap_or_else(|| u.default_avatar_url()));

    let global_avatar_text = format!("[Link]({})", &u.avatar_url().unwrap_or_else(|| u.default_avatar_url()));

    ctx.send(|e| {
        e.embed(|e| {
            e.set_author(author)
            .title("Account information")
            .colour(Colour::FADED_PURPLE)
            .fields(vec![
                ("__Basics__", "", false),
                ("Full name:", &u.tag(), false),
                ("ID", &u.id.to_string(), false),
                ("Bot?", &bot_str, false),
                ("Created at", &created_at, false),
                ("Global avatar", &global_avatar_text, false),
                
            ])
            .thumbnail(u.avatar_url().unwrap_or_else(|| u.default_avatar_url()))

        })
    })
    .await?;
    Ok(())
}

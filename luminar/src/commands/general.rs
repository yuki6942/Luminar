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

#[poise::command(prefix_command, slash_command, track_edits, category = "General")]
/// Shows the help menu
pub async fn help(
    ctx: LuminarContext<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> LuminarResult {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "\
To get information about a specific command, use `help <command>`.",
            show_context_menu_commands: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/* TODO: Make this into the new help command with an select menu, get automatically the categories and commands and filter if,
    It has hide_in_help or a owner_only check
    Also check if the user who invoked the command is the user that uses the interaction on the select menu


// poise slash and prefix command named commands
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn show(ctx: LuminarContext<'_>) -> LuminarResult {
    // Send a embed which has a introduction to the bot and list all command categories
    ctx.send(|b| {
        b.embed(|b| {
            b.title("Luminar - Help")
                .fields(vec![
                    ("__Categories__", "General", false),
                    ("__Prefix__", "`~` or `/`", false),
                    ("__Invite__", "[Click here](https://discord.com/api/oauth2/authorize?client_id=1075052879443931208&permissions=8&scope=bot%20applications.commands)", false),
                    ("__Support__", "[Click here](https://discord.gg/EnEDSYWvUm)", false),
                    ("__GitHub__", "[Click here](https://github.com/yuki6942/Luminar)", false),
                ])
                .colour(Colour::FADED_PURPLE)
        }) 
    })
    .await?;

    Ok(())
}

 */

#[poise::command(prefix_command, slash_command, category = "General")]
/// Shows some information a specifc user
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

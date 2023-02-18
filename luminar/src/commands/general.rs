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
/// Shows some information an user
pub async fn userinfo(
    ctx: LuminarContext<'_>,
    #[description = "The user to get the information from"] user: Option<serenit::User>,
) -> LuminarResult {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let bot_str = if u.bot { "Yes" } else { "No" }.to_owned();
    let mention = format!("<@{}>", &u.id.to_string()).to_owned();
    let time_since_create = format!("<t:{}:R>", &u.created_at().timestamp()).to_owned();
    let created_at = format!("<t:{}:F>", &u.created_at().timestamp()).to_owned();
    let mut author = serenit::CreateEmbedAuthor::default();
    author.name(&u.name);
    author.icon_url(u.avatar_url().unwrap_or_else(|| u.default_avatar_url()));
    let global_avatar_text = format!(
        "[Link]({})",
        &u.avatar_url().unwrap_or_else(|| u.default_avatar_url())
    );
    let guild_owner = if ctx.guild().unwrap().owner_id == u.id {
        "Yes"
    } else {
        "No"
    }
    .to_owned();
    let guild_name = format!("__Within {}__", ctx.guild().unwrap().name.to_owned());

    let nickname = if ctx
        .guild()
        .unwrap()
        .member(&ctx, u.id)
        .await?
        .nick
        .is_some()
    {
        ctx.guild().unwrap().member(&ctx, u.id).await?.nick.unwrap()
    } else {
        "No nickname set".to_owned()
    };

    let boost_status: String = if ctx
        .guild()
        .unwrap()
        .member(&ctx, u.id)
        .await?
        .premium_since
        .is_some()
    {
        format!(
            "Boosting since <t:{}:F>",
            &ctx.guild()
                .unwrap()
                .member(&ctx, u.id)
                .await?
                .premium_since
                .unwrap()
                .timestamp()
                .to_owned()
        )
    } else {
        "Not boosting the server.".to_owned()
    };

    let roles = ctx
        .guild()
        .unwrap()
        .member(&ctx, u.id)
        .await?
        .roles
        .iter()
        .map(|r| format!("<@&{}>", r))
        .collect::<Vec<String>>()
        .join(", ")
        .to_owned();

    /* TODO: Make this work lol (Get every bade and show the emoji)
        let badges: String = if u.public_flags.is_some() {
            format!("{:?}", &u.public_flags.unwrap_or_default().to_owned())
        } else {
            format!("{} has no public badges.", &u.name)
        };
    */

    let join_date = format!(
        "<t:{}:F>",
        &ctx.guild()
            .unwrap()
            .member(&ctx, u.id)
            .await?
            .joined_at
            .unwrap()
            .timestamp()
            .to_owned()
    );
    let time_since_join = format!(
        "<t:{}:R>",
        &ctx.guild()
            .unwrap()
            .member(&ctx, u.id)
            .await?
            .joined_at
            .unwrap()
            .timestamp()
            .to_owned()
    );

    ctx.send(|e| {
        e.embed(|e| {
            e.set_author(author)
                .title("Account information")
                .colour(Colour::FADED_PURPLE)
                .fields(vec![
                    ("__Basics__", "", false),
                    ("Full name:", &u.tag(), false),
                    ("Mention:", &mention, false),
                    ("ID:", &u.id.to_string(), false),
                    ("Creation date:", &created_at, false),
                    ("Time since creation date:", &time_since_create, false),
                    ("Bot?", &bot_str, false),
                    ("Global avatar:", &global_avatar_text, false),
                    (&guild_name, "", false),
                    ("Nickname:", &nickname, false),
                    ("Join date:", &join_date, false),
                    ("Time since join date:", &time_since_join, false),
                    ("Boost status:", &boost_status, false),
                    ("Roles:", &roles, false),
                    ("Server owner?", &guild_owner, false),
                    ("__Badges__", ""/*&badges*/, false),
                ])
                .thumbnail(u.avatar_url().unwrap_or_else(|| u.default_avatar_url()))
        })
    })
    .await?;

    Ok(())
}

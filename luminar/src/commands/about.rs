use crate::utils::luminar::{LuminarContext, LuminarError};
use poise::serenity_prelude::{self as serenit};
use serenity::utils::Colour;

#[poise::command(prefix_command, slash_command, category="General")]
/// Shows some information about Luminar
pub async fn about(ctx: LuminarContext<'_>) -> Result<(), LuminarError> {

    let version = env!("CARGO_PKG_VERSION");

    ctx.send(|b| {
        b.embed(|b| b.title(
            "About Luminar"
        ).fields(vec![
            ("__Version__", version, true),
        ]).colour(Colour::BLITZ_BLUE))
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
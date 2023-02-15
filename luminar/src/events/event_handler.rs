use crate::utils::luminar::{LuminarData ,LuminarError};

use poise::serenity_prelude as serenity;


pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, LuminarData, LuminarError>,
    _user_data: &LuminarData,
) -> Result<(), LuminarError> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name)
        }
        _ => {}
    }

    Ok(())
}

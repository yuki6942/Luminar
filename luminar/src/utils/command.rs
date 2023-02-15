use crate::utils::{
    luminar::{LuminarContext},
};

pub async fn pre_command(ctx: LuminarContext<'_>) {
    println!(
        "Got command '{}' by user '{}'",
        ctx.command().name,
        ctx.author().name
    );

    let mut command_counter = ctx.data().command_counter.lock().unwrap();
    let entry = command_counter
        .entry(ctx.command().name.to_string())
        .or_insert(0);
    *entry += 1;
}

pub async fn post_command(ctx: LuminarContext<'_>) {
    println!("Processed command '{}'", ctx.command().name);
}

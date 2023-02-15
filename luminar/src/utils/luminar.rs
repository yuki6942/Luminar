use std::{
    sync::Mutex,
    collections::HashMap,
    error::Error
};
use poise::Context;

pub struct LuminarData {
    pub command_counter: Mutex<HashMap<String, u64>>,
}

pub type LuminarError = Box<dyn Error + Send + Sync>;

pub type LuminarContext<'a> = Context<'a, LuminarData, LuminarError>;
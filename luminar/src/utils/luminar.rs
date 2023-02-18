use poise::Context;
use std::{collections::HashMap, error::Error, sync::Mutex};

pub struct LuminarData {
    pub command_counter: Mutex<HashMap<String, u64>>,

}

pub type LuminarError = Box<dyn Error + Send + Sync>;

pub type LuminarContext<'a> = Context<'a, LuminarData, LuminarError>;

pub type LuminarResult = Result<(), LuminarError>;

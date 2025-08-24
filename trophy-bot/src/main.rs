mod macros;
mod types;
mod traits;
mod local_implementation;
mod errors;
mod discord_implementations;

use poise::serenity_prelude as serenity;
use local_implementation::local_context;

use crate::traits::SendMessage;

struct Data {}

// define error and context
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let new_context = local_context::LocalContext::new();
    custom_send!(context: new_context, ephemeral: true, contents: "HELLO!".to_string());

}

use std::env;

use serenity::{
    async_trait,
    model::prelude::Ready,
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a discord token in the environment");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client :(");

    if let Err(e) = client.start().await {
        println!("CLient error: {:?}", e);
    }
}

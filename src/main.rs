use std::env;

use serenity::{
    async_trait,
    model::{
        prelude::{ChannelId, Ready},
        voice::VoiceState,
    },
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name)
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        match old {
            Some(o) => {
                if o.mute != new.mute && new.mute {
                    new.member
                        .unwrap()
                        .move_to_voice_channel(&ctx.http, ChannelId(123))
                        .await
                        .expect("Something went wrong moving the user to muted channel");
                }
            }
            None => {
                eprintln!("Something went wrong getting the user who changed the voice state!");
            }
        }
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

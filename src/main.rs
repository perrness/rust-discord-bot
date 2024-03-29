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

struct Handler {
    muted_chat_id: u64,
    general_chat_id: u64,
    server_owner_id: u64,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name)
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        match old {
            Some(o) => {
                if o.mute != new.mute && new.mute {
                    let user = new.member.unwrap();

                    if user.user.id == self.server_owner_id {
                        return;
                    }

                    user.move_to_voice_channel(&ctx.http, ChannelId(self.muted_chat_id))
                        .await
                        .expect("Something went wrong moving the user to muted channel");
                    let channel_id = ChannelId(self.general_chat_id);
                    channel_id
                        .send_message(&ctx.http, |m| {
                            m.content(format!("{} muted themselves!", user.user.name))
                        })
                        .await
                        .expect("Could not send message about user muting themself");
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
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a discord token in the environment");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;

    let handler = Handler {
        muted_chat_id: env::var("MUTED_CHAT_ID")
            .expect("Expected MUTED_CHAT_ID in environment")
            .parse::<u64>()
            .unwrap(),
        general_chat_id: env::var("GENERAL_CHAT_ID")
            .expect("Expected MUTED_CHAT_ID in environment")
            .parse::<u64>()
            .unwrap(),
        server_owner_id: env::var("SERVER_OWNER_ID")
            .expect("Expected MUTED_CHAT_ID in environment")
            .parse::<u64>()
            .unwrap(),
    };

    let mut client = Client::builder(token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client :(");

    if let Err(e) = client.start().await {
        println!("CLient error: {:?}", e);
    }
}

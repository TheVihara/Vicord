use std::env;
use serenity::Client;
use serenity::framework::StandardFramework;
use serenity::prelude::GatewayIntents;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let framework = StandardFramework::new().configure(|c| c.prefix("!"));

    // Login with a bot token from the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token.");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("Error creating client.");

    // Start listening for events by starting a single shard.
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

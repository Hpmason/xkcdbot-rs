
use std::env;

#[cfg(feature = "discord")]
use serenity::{
    Client,
    framework::standard::StandardFramework
};

mod handler;
use handler::*;
mod embeds;

#[tokio::main]
async fn main() {
    #[cfg(feature = "discord")] {
        // Configure the client with your Discord bot token in the environment.
        let token = env::var("XKCD_BOT_TOK")
            .expect("Expected a token in the environment");

        let framework = StandardFramework::new()
            .configure(|c| c.prefix("!"))
            // The `#[group]` (and similarly, `#[command]`) macro generates static instances
            // containing any options you gave it. For instance, the group `name` and its `commands`.
            // Their identifiers, names you can use to refer to these instances in code, are an
            // all-uppercased version of the `name` with a `_GROUP` suffix appended at the end.
            .group(&GENERAL_GROUP);

        // Create a new instance of the Client, logging in as a bot. This will
        // automatically prepend your bot token with "Bot ", which is a requirement
        // by Discord for bot users.
        let mut client = Client::builder(&token)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Err creating client");

        // Finally, start a single shard, and start listening to events.
        //
        // Shards will automatically attempt to reconnect, and will perform
        // exponential backoff until it reconnects.
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}
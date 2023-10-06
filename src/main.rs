use std::sync::Arc;

use anyhow::Context;
use serenity::{
    prelude::*,
    framework::StandardFramework,
    model::prelude::ChannelId
};
use shuttle_secrets::SecretStore;
use discobot::{Bot, REDDIT_GROUP, SubredditsStore, ShardManagerContainer};
// use roux::Subreddit;


#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Get the channel id set in `Secrets.toml`
    let channel = secret_store
        .get("CHANNEL_ID")
        .context("'CHANNEL_ID' was not found")?;

    let channel_id = channel
        .parse::<u64>()
        .context("'CHANNEL_ID' should be valiable number")?;
    // Commands configuration
    let framework = StandardFramework::new()
        .configure(|c| c
            .allowed_channels(vec![ChannelId(channel_id)].into_iter().collect())
            .prefix("!"))
        .group(&REDDIT_GROUP);
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .framework(framework)
        .type_map_insert::<SubredditsStore>(Vec::new())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }        

    Ok(client.into())
}

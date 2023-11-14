use std::sync::Arc;
use tokio::sync::Mutex;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::{Context, TypeMapKey};
use serenity::model::channel::Message;
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::{command, group};


pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct SubredditsStore;

impl TypeMapKey for SubredditsStore {
    type Value = Vec<String>;
}

#[group]
#[commands(watch, remove)]
pub struct Reddit;
// TODO: Add a "list" command for getting all watched subreddits?
// TODO: Namechecking extraction for readability/something else
// TODO: Write tests
#[command]
#[description = "Add subreddit to watchlist"]
pub async fn watch(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let subreddit_name = args.parse::<String>().expect("Should always be a string");

    if subreddit_name.len() > 21 {
        msg.reply(&ctx.http, "Name cannot be longer than 21 characters!").await?;
        return Ok(());
    }

    if subreddit_name.chars().next() == Some('_') {
        msg.reply(&ctx.http, "Name cannot begin with an underscore!").await?;
        return Ok(());
    };

    if subreddit_name.chars().any(|x|!x.is_alphanumeric() && x != '_') {
        msg.reply(&ctx.http, "Only letters, numbers and underscores are allowed for subreddit name.").await?;
        return Ok(());
    };

    let mut data = ctx.data.write().await;
    let subreddits = data.get_mut::<SubredditsStore>().expect("Expected Subreddits in TypeMap.");

    if subreddits.contains(&subreddit_name) {
        msg.reply(&ctx.http, "This subreddit is already in the watchlist!").await?;
        return Ok(());
    } else {
        subreddits.push(subreddit_name.to_owned());
        msg.reply(&ctx.http, format!("Added \"{}\" to watchlist.", subreddit_name)).await?;
        return Ok(());
    }
}

#[command]
#[description = "Remove subreddit from watchlist"]
pub async fn remove(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let subreddit_name = args.parse::<String>().unwrap();

    if subreddit_name.len() > 21 {
        msg.reply(&ctx.http, "Name cannot be longer than 21 characters!").await?;
        return Ok(());
    }

    if subreddit_name.chars().next() == Some('_') {
        msg.reply(&ctx.http, "Name cannot begin with an underscore!").await?;
        return Ok(());
    };

    if subreddit_name.chars().any(|x|!x.is_alphanumeric() && x != '_') {
        msg.reply(&ctx.http, "Only letters, numbers and underscores are allowed for subreddit name.").await?;
        return Ok(());
    };

    let mut data = ctx.data.write().await;
    let subreddits = data.get_mut::<SubredditsStore>().expect("Expected Subreddits in TypeMap.");

    if !subreddits.contains(&subreddit_name) {
        msg.reply(&ctx.http, "This subreddit is not in the watchlist!").await?;
        return Ok(());
    } else {
        subreddits.retain(|subreddit| subreddit != &subreddit_name);
        msg.reply(&ctx.http, format!("Removed \"{}\" from watchlist.", subreddit_name)).await?;
        return Ok(());
    }

}
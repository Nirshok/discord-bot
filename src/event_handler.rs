use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info};

pub struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("https://twitter.com") {
                let user = msg.author.name.as_str();
                let message = msg.content.replace("https://twitter.com", "https://vxtwitter.com");  
                let bot_response = format!("From: **{}**\n\n{}", user, message);

                if msg.referenced_message.is_some() {
                    if let Err(e) = msg.reply(&ctx, &bot_response).await {
                        error!("Error replying to a message: {:?}", e);
                    }
                } else {
                    if let Err(e) = msg.channel_id.say(&ctx.http, bot_response).await {
                        error!("Error sending message: {:?}", e);
                    }
                }
                
                    
                if let Err(e) = msg.delete(ctx.http).await {
                    error!("Error deleting original message: {:?}", e);
                }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

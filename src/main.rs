use std::env;
// use std::fs::File;

mod memepoint;

use crate::memepoint::{calc_meme_points, genboard, updateboard};
use serenity::all::{ChannelId, GuildMemberUpdateEvent, Member};
use serenity::async_trait;
use serenity::builder::{CreateEmbed, CreateMessage, EditMessage};
use serenity::model::channel::Message;
use serenity::prelude::*;
use tokio::time::{Duration, sleep};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!test" && !msg.author.bot {
            let embed = CreateEmbed::new()
                .title("Totally Serious Leaderboard")
                .description("MEME POINT")
                .fields(genboard(msg.guild_id.expect("awawaw")));
            let builder = CreateMessage::new().embed(embed);
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content.contains("Luna")
            && msg.content.contains("remove")
            && msg.content.contains("role")
            && !msg.author.bot
        {
            let builder = CreateMessage::new().content("NOOOOOOOOOO, I am INCREDIBLY OFFENDED that you did that qwq, I have put le funny meme assemble HQ president role back");
            sleep(Duration::new(2, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content.contains("?") && msg.content.contains("HOW") && !msg.author.bot {
            let builder = CreateMessage::new().content("skill issue");
            sleep(Duration::new(2, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content.contains("<:What:768586399732727808>") && !msg.author.bot {
            let builder = CreateMessage::new().content("<:What:768586399732727808>");
            sleep(Duration::new(2, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content.contains("entris")
            && msg.content.contains("oth side")
            && !msg.author.bot
        {
            let builder = CreateMessage::new().content("<:ReisenIDontEvenKnow:678322617538838529>");
            sleep(Duration::new(2, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content.contains("roissant")
            && msg.content.contains("rashant")
            && !msg.author.bot
        {
            let builder = CreateMessage::new().content("im a centrist lol");
            sleep(Duration::new(2, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content.contains("roissant") && !msg.author.bot {
            let builder = CreateMessage::new().content("no its prashant");
            sleep(Duration::new(2, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content.contains("rashant") && !msg.author.bot {
            let builder = CreateMessage::new().content("no its croissant");
            sleep(Duration::new(2, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content.contains("<@1468954832764276856>")
            && msg.content.contains("gay")
            && !msg.author.bot
        {
            let builder = CreateMessage::new()
                .content("IMNOTGAYIMABABAIMPLANTIMBOTIMAROIMACEIMAPLIMANATRACTIONALABABA");
            sleep(Duration::new(2, 0)).await;
            let msgs = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msgs {
                println!("Error sending message: {why:?}");
            }
            let builder = CreateMessage::new().content("<:ABABA:1005758975435870218>");
            sleep(Duration::new(1, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        } else if (msg.content.contains("bot") || msg.content.contains("<@1468954832764276856>"))
            && msg.content.contains("cute")
            && !msg.author.bot
        {
            let builder = CreateMessage::new()
                .content("NOIMNOTCUTEIMABABAIMPLANTIMBOTYOUCUTEYOUSILLYIMSILLY");
            sleep(Duration::new(2, 0)).await;
            let msgs = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msgs {
                println!("Error sending message: {why:?}");
            }
            let builder = CreateMessage::new().content("<:ABABA:1005758975435870218>");
            sleep(Duration::new(1, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content.contains("https://tenor.com/view/girls-last-tour-shoujo-shuumatsu-ryokou-squish-stretch-book-gif-20324471") {
            let builder = CreateMessage::new().content("https://cdn.discordapp.com/attachments/824449350238732388/1469666036147290327/flip.gif");
            sleep(Duration::new(2, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
        if (msg.content.contains("ay bot")
            || msg.content.contains("ute bot")
            || msg.content.contains("<@1468954832764276856>"))
            && msg.content.contains("shower")
            && !msg.author.bot
        {
            let builder = CreateMessage::new().content("https://tenor.com/view/touhou-murasa-dodge-go-shower-shower-gif-14832011749876304043");
            sleep(Duration::new(3, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content.contains("factory") && msg.content.contains("when") && !msg.author.bot {
            let builder = CreateMessage::new().content("https://media.discordapp.net/attachments/659119924840759307/1195819457696772187/meme.gif");
            sleep(Duration::new(4, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
        if (msg.content.contains("ay bot")
            || msg.content.contains("ute bot")
            || msg.content.contains("<@1468954832764276856>"))
            && msg.content.contains("rate")
            && !msg.author.bot
        {
            let builder = CreateMessage::new()
                .content("obviously im literally the best bot ever made, trust");
            sleep(Duration::new(4, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content.contains("will you date me?") && msg.author.id.get() == 1458023696940929137 {
            sleep(Duration::new(1, 0)).await;
            let msgs = msg
                .reply_ping(&ctx.http, "Yes uwu, but it has to be an open relationship")
                .await;
            if let Err(why) = msgs {
                println!("Error sending message: {why:?}");
            }
            let builder = CreateMessage::new().content("along with vert, rose, ray and luminance");
            sleep(Duration::new(1, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content.contains("no me") && msg.author.id.get() == 1458023696940929137 {
            sleep(Duration::new(1, 0)).await;
            let msgs = msg.reply_ping(&ctx.http, "no both").await;
            if let Err(why) = msgs {
                println!("Error sending message: {why:?}");
            }
            let builder = CreateMessage::new().content("<:ABABA:1005758975435870218>");
            sleep(Duration::new(1, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content.contains("only if you open source")
            && msg.author.id.get() == 1458023696940929137
        {
            sleep(Duration::new(1, 0)).await;
            let msgs = msg
                .reply_ping(&ctx.http, "AWAWA, now you can see my bad code at https://github.com/Aster-sama/SillyBot/ qwq")
                .await;
            if let Err(why) = msgs {
                println!("Error sending message: {why:?}");
            }
            let builder = CreateMessage::new().content("<:ABABA:1005758975435870218>");
            sleep(Duration::new(1, 0)).await;
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn guild_member_update(
        &self,
        ctx: Context,
        _old: Option<Member>,
        _new: Option<Member>,
        event: GuildMemberUpdateEvent,
    ) {
        println!("ohwowupdate!");
        println!("{:?}", event.roles);
        if event.user.id.get() != 195219971754819584 {
            let score = calc_meme_points(&ctx, event.guild_id, event.roles, event.user.id)
                .await
                .expect("awawa");
            let _ = updateboard(event.guild_id, event.user.id, score);
            let name = "board/".to_owned() + &event.guild_id.get().to_string() + ".id";
            println!("{:?}", name);
            let mut rdr = csv::Reader::from_path(name).expect("no message id file");
            for result in rdr.records() {
                let record = result.expect("no channelid in file");
                let embed = CreateEmbed::new()
                    .title("Totally Serious Leaderboard")
                    .description("More points = You are a better being (/s)")
                    .fields(genboard(event.guild_id));
                let builder = EditMessage::new()
                    .embed(embed)
                    .content("# MEME ROLE LEADERBOARD");
                let msg = ChannelId::new(record[0].parse().expect("id file not a number"))
                    .edit_message(
                        &ctx.http,
                        record[1].parse::<u64>().expect("id file not a number"),
                        builder,
                    )
                    .await;
                if let Err(why) = msg {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

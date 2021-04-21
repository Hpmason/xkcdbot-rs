use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message, 
        gateway::Ready,
    },
    framework::standard::{
        macros::{command, group},
        Args,
        CommandResult,
    }
};

use xkcd_utils::{comic::Comic, error::XKCDError, explain::ExplainXKCD, xkcd::XKCD};

use crate::embeds::Embeddable;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "pong!").await?;

    Ok(())
}

#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "pong!").await?;

    Ok(())
}

async fn get_comic(mut args: Args) -> Result<Comic, XKCDError> {
    let home = Comic::get_home().await?;
    // Get home page if no page specified
    if args.is_empty() {
        return Ok(home);
    }
    // If number provided as arg, search to page id
    let id= args.single::<i32>();
    if let Ok(id) = id {
        if id < 0 {
            return Err(XKCDError::NegativeNumber(id));
        }
        let id = id as u32;
        if id > home.num || id < 1 {
            return Err(XKCDError::OutOfRange(id));
        }
        return Comic::get_page(id as u32).await;
    }
    else if let Err(e) = id {
        println!("{}", e.to_string());
    }
    println!("Id not entered, searching now");
    args.restore();
    let search_params = args.rest();
    return Comic::search(&search_params).await;
}

/*
 *  TODO:
 *      Need to do error handling (wrong input)
 */
#[command]
async fn comic(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("Comic command received");
    let result = get_comic(args).await;
    
    match result {
        Ok(comic) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.set_embed(comic.as_embed());
                m
            }).await?;
        },
        Err(e) => {
            println!("{:?}", e);
            let resp = match_error(e);
            msg.channel_id.say(&ctx.http, resp).await?;
        }
    }
   
    Ok(())
}

fn match_error(e: XKCDError) -> String {
    match e {
        XKCDError::SearchError(s) => format!("Error finding requested page. Seach terms: \"{}\"", s), 
        XKCDError::ParseError(s) => format!("Error parsing page data. Error: {}", s),
        XKCDError::GetError(e) => format!("Error requesting page. Error: {}", e.to_string()),
        XKCDError::OutOfRange(i) => format!("Request made was out of range ({})", i),
        XKCDError::NegativeNumber(i) => format!("Request made with a negative number ({})", i),
        XKCDError::ParseIntError(e) => format!("Error while parsing int. {}", e.to_string()),
        XKCDError::ArgParseError(e) => format!("Error while parsing args. {}", e.to_string()),
    }
}

#[command]
async fn random(ctx: &Context, msg: &Message) -> CommandResult {

    let result = Comic::get_random().await;
    match result {
        Ok(comic) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.set_embed(comic.as_embed());
                m
            }).await?;
        },
        Err(e) => {
            println!("{:?}", e);
            let resp = match_error(e);
            msg.channel_id.say(&ctx.http, resp).await?;
        }
    }
    
    Ok(())
}
async fn get_explain(mut args: Args) -> Result<ExplainXKCD, XKCDError>{
    if args.is_empty() {
        return ExplainXKCD::get_home().await;
    }
    else {
        let id = args.single::<u32>()?;
        return ExplainXKCD::get_page(id).await;
    }
}

#[command]
async fn explain(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    println!("Explain command received");
    let result = get_explain(args).await;
    
    match result {
        Ok(explain) => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.set_embed(explain.as_embed());
                m
            }).await?;
        },
        Err(e) => {
            println!("{:?}", e);
            let status = msg.channel_id.say(&ctx.http, e.to_string()).await;
            if let Err(e) = status {
                println!("Error:{}", e.to_string());
            }
        }
    }
   
    Ok(())
}

#[group]
#[commands(ping, comic, explain, random, test)]
struct General;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

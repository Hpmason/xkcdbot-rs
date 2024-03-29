use serenity::builder::CreateEmbed;

use xkcd_utils::{comic::Comic, explain::ExplainXKCD, xkcd_regex::MarkdownConverter};

pub const AUTHOR_NAME: &str = "XKCD Bot";
pub const AUTHOR_URL: &str = "https://github.com/Hpmason";
pub const ICON_URL: &str = "https://xkcd.com/s/0b7742.png";

pub trait Embeddable {
    fn as_embed(&self) -> CreateEmbed;
}

impl Embeddable for Comic {
    fn as_embed(&self) -> CreateEmbed {
        let mut e = CreateEmbed::default();
        e.title(format!("XKCD - {}: {}", self.num, self.title));
        
        // e.field("Page ID", &self.num, true);
        // e.field("Permalink", &format!("http://xkcd.com/{}", self.num), true);
        e.url(&format!("http://xkcd.com/{}", self.num));
        if self.transcript.is_empty() {
            e.description(format!("__**Transcript**__:\n{}", 
                MarkdownConverter::from_xkcd(&self.transcript).discord()
            ));
        }
        e.field("Explain xkcd", format!("https://www.explainxkcd.com/wiki/index.php/{}", &self.num), false);
        e.image(&self.img);
        
        e.author(|a| {
            a.name(AUTHOR_NAME);
            a.url(AUTHOR_URL);
            a.icon_url(ICON_URL);
            a
        });
        e.footer(|f| {
            f.text(&self.alt);
            f
        });
        e
    }
}


impl Embeddable for ExplainXKCD {
    fn as_embed(&self) -> CreateEmbed {
        let mut e = CreateEmbed::default();
        e.title(format!("Explain - {}: {}", self.num, &self.title));
        let markdown = MarkdownConverter::from_explain(&self.explanation).discord();
        if markdown.len() > 2048 {
            println!("Explanation is longer than 2048 chars (len={}), spliting into fields", markdown.len());
            
            let mut i = 0;
            let mut s = String::new();
            for lin in markdown.split('\n') {
                if lin.len() + s.len() + 2 < 1024 {
                    s += "\n";
                    s += lin;
                }
                else {
                    e.field(format!("Line {}", i), &s, false);
                    i += 1;
                    s.clear();
                }
            }
            if !s.trim().is_empty() {
                e.field(format!("Line {}", i), s, false);
            }
        }
        else {
            e.description(markdown);
        }
        e.url(&self.permalink);
        e.author(|a| {
            a.name(AUTHOR_NAME);
            a.url(AUTHOR_URL);
            a.icon_url(ICON_URL);
            a
        });
        e.footer(|f| {
            f.text("Generated by XKCD Bot");
            f
        });
        e
    }
}
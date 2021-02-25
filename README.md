# xkcdbot-rs
Discord bot for xkcd.com comics
## Usage
xkcdbot-rs supports the following commands:
- !explain [comic_num]
  - Gets the explain xkcd explaination from explainxkcd.com
- !comic [comic_num | search terms]
  - Searchs for the comic requested
- !random
  - Pulls a random xkcd comic
## Setup
xkcdbot-rs requires 2 environmental variables tokens/keys.
- `XKCD_BOT_TOK`
- `GOOGLE_SEARCH_KEY`

#### Setting environment variables
##### Windows Powershell
```
$Env:TOKEN_NAME="TOKEN_KEY"
```
##### Linux/Unix/Mac
```
export TOKEN_NAME="TOKEN_KEY"
```
#### XKCD_BOT_TOK
You can get this token from https://discord.com/developers/applications.
Steps are:
1. Create a discord app
2. In your new app, click the `Bot` tab on the left hand side
3. Click `Add Bot`
4. Copy the token provided
5. Add this as an environment variable under the name `XKCD_BOT_TOK`
6. Under `OAuth2` create an OAuth url with the scope of `bot` and the permission to send messages
    - In order to add the bot to your disord server, open this link and discord will walk you through adding the bot

#### GOOGLE_SEARCH_KEY
You can get a google API key by following: https://cloud.google.com/docs/authentication/api-keys

To give it the proper permissions, enable this api: https://console.cloud.google.com/apis/library/customsearch.googleapis.com

And then make sure to restrict the API key to only be able to use the custom search API from the Credentials dashboard https://console.cloud.google.com/apis/credentials

### Instalation
You can run the bot 3 ways:
- Install via `cargo install --git https://github.com/Hpmason/xkcdbot-rs` and then run with `xkcdbot-rs` command
- Download release executable
- Build from source
  - git clone this repo
  - run `cargo run` or build then run with `cargo build`

# Contributing
 I'm always open to pull requests and suggestions

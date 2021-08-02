# xkcdbot-rs
Discord bot for xkcd.com comics

Docker image can be found on my [GitHub Container Registry](https://github.com/Hpmason/mcbot-rs/pkgs/container/xkcdbot-rs)
```
docker pull ghcr.io/hpmason/mcbot-rs:latest
```
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
If you don't know how to get these keys, look below
### Running
If you want to build the image manually, you can do so by running the following commands:
```
docker build -t xkcdbot .
```
Run the bot image
```
docker run --rm --env-file .env --name xkcdbot xkcdbot
```

## Obtaining keys

#### XKCD_BOT_TOK
This token is required in order for the program to communicate with discord via a bot.

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
This token is required to search for xkcd comics from key words. The token is not used by this repo, but by the xkcd-utils repo (see https://github.com/Hpmason/xkcd-utils/blob/master/src/search.rs).

You can get a google API key by following: https://cloud.google.com/docs/authentication/api-keys

To give it the proper permissions, enable this api: https://console.cloud.google.com/apis/library/customsearch.googleapis.com

And then make sure to restrict the API key to only be able to use the custom search API from the Credentials dashboard https://console.cloud.google.com/apis/credentials

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

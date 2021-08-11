# private-channel-bot

This is a minimalistic chatbot for Anarchy Online that provides a private channel to relay with.

## Setup

Create a `.env` file that looks like this:

```
CHARNAME=Mycharacter
USERNAME=Myusername
PASSWORD=mypassword
RUST_LOG=info
```

Then, on all bots you'd like to join the private channel, send `!register` in a tell to the bot.

Whenever your bots log on, they will now be invited to the private channel.

Similarly, `!unregister` can be used to disable this again.

## Public instance

We run a bot called `Privchannel` that provides a public instance of this bot.

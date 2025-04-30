# rssbot

This is a sample implementation of an RSS bot that forwards messages to [Taggr](https://taggr.link).

If you want to run your own bot on any RSS feed, follow the steps below.

## Setup

### Step 1

Install the latest `dfx` tool following the official [intructions](https://internetcomputer.org/docs/building-apps/getting-started/install).

### Step 2

Install the latest `cargo` tool following the official [instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Step 3

Checkout the `rssbot` code: 

    git clone git@github.com:TaggrNetwork/rssbot.git

Open the file `src/rssbot/src/rss.rs` and change variables

- `REALM` to the realm name for bot's messages,
- `RSS_FEED` to the host name of your RSS feed.

If necessary, change the XML parsing inside the function `parse_items` and use the `test_parsing` unit test to test your changes.

### Step 4

Deploy the canister to the Internet Computer and enable canister logs:

    make deploy
    make enable_canister_logs

You should see a message like this:

    Installing code for canister rssbot, with canister ID iqq3j-byaaa-aaaak-qug4a-cai

Copy the canister ID displayed in this message.

### Step 5

1. Create a new user on Taggr.
2. Open the [settings page](https://taggr.link/#/settings).
3. Locate the field `Controller principal` and enter into this field the cansiter ID copied from the previous step.

Now, the bot will fetch the RSS feed, prse the items and schedule them as messages.
The bot will post one message scheduled message to Taggr every half an hour.
If you want to change the posting frequency, change the variable `POSTING_INTERVAL_MINS` in the file `src/rssbot/src/lib.rs`.

## Operations

To check the internal state of the bot, run `make stats`.

To check the logs of the bot, run `make logs`.

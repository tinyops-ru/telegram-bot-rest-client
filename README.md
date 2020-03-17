# Telegram Bot Rest Client

Client for [Telegram Bot Rest service](https://github.com/tinyops-ru/telegram-bot-rest).

## Usage

1. Create configuration:
    ```shell script
    cp telegram-bot-rest-client.conf-dist telegram-bot-rest-client.conf
    ```

2. Edit config file.

3. Send message:

```shell script
telegram-bot-rest-client "your message here"
```

## How to build from sources

1. Install [Rust](https://www.rust-lang.org/learn/get-started)

2. Run:

```shell script
cargo build --release
```
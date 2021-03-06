# Telegram Bot Rest Client

Client for [Telegram Bot Rest service](https://github.com/tinyops-ru/telegram-bot-rest).

## Usage

1. Create configuration:
    ```shell script
    cp tbrc.conf-dist tbrc.conf
    ```

2. Edit `tbrc.conf` file.

3. Send message:

    ```shell script
    tbrc "your message here"
    ```
   
### Specify working directory

Example:

```shell script
tbrc --work-dir=/home/user "my message"
```

## How to build from sources

### Build on CentOS 6/7+

```shell script
cargo build --release
```

### Build on Windows

Comment openssl dependency from `Cargo.toml`.

```shell script
cargo build --release
```
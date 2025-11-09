# Nova Gateway Telegram Bot

A Teloxide-based Telegram bot that forwards chat messages to the Nova Gateway `/ai` endpoint and returns the AI's response.

## Requirements
- Rust 1.70+
- A Telegram bot token (`TELEGRAM_BOT_TOKEN`)
- A Nova Gateway API key (`NOVA_API_KEY`)

## Configuration
Set the following environment variables (consider using a `.env` file):

| Variable | Required | Description |
| --- | --- | --- |
| `TELEGRAM_BOT_TOKEN` | Yes | Telegram bot token from @BotFather |
| `NOVA_API_KEY` | Yes | Nova Gateway API key (`nova_...`) |
| `NOVA_BASE_URL` | No | Override base URL (default `https://gateway.inferenco.com`) |
| `NOVA_MODEL` | No | Model to use (`gpt-5`, `gpt-5-mini`, etc.; default `gpt-5-mini`) |
| `NOVA_VERBOSITY` | No | Response verbosity (`Low`, `Medium`, `High`; default `Medium`) |
| `NOVA_MAX_TOKENS` | No | Maximum response tokens (default `1024`) |
| `NOVA_REASONING` | No | Enable reasoning (`true`/`false`; default `false`) |
| `NOVA_REASONING_EFFORT` | No | Optional reasoning effort hint (e.g., `Medium`) |
| `NOVA_TIMEOUT_SECONDS` | No | HTTP timeout in seconds (default `60`) |

## Running
```bash
cargo run
```

The bot registers the `/start`, `/help`, and `/reset` commands. Use `/reset` to clear the Nova conversation history for the current chat.

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

## Sample `.env`
Copy `.env.sample` to `.env` (e.g., `cp .env.sample .env`) and replace the placeholder values with your own Telegram bot token and Nova Gateway API key. Leave optional settings commented out unless you need to override the defaults.

## Running
```bash
cargo run
```

The bot registers the `/start`, `/help`, and `/reset` commands. Use `/reset` to clear the Nova conversation history for the current chat.

## Docker
Build the image from the project root:

```bash
docker build -t nova-gateway-telegram-bot .
```

Run the container with the same environment variables you configured locally:

```bash
docker run --env-file .env --restart unless-stopped nova-gateway-telegram-bot
```

Or let Docker Compose handle the build/run loop after you copy `.env.sample`:

```bash
docker compose up --build
```

## Cloud Run
Deploy to Google Cloud Run with the Dockerfile in this repo:

1. Build and push the image with Cloud Build:
   ```bash
   gcloud builds submit --tag gcr.io/PROJECT_ID/nova-gateway-telegram-bot
   ```
2. Deploy the image to Cloud Run:
   - **Console**: Navigate to Cloud Run → *Create Service*, select the pushed image, set *Min/Max instances* to `1`, and add the required environment variables (`TELEGRAM_BOT_TOKEN`, `NOVA_API_KEY`, etc.) under the *Variables & Secrets* tab.
   - **CLI (optional)**:
     ```bash
     gcloud run deploy nova-gateway-telegram-bot \
       --image gcr.io/PROJECT_ID/nova-gateway-telegram-bot \
       --region REGION \
       --set-env-vars TELEGRAM_BOT_TOKEN=xxx,NOVA_API_KEY=xxx \
       --set-env-vars NOVA_BASE_URL=https://gateway.inferenco.com \
       --allow-unauthenticated=false \
       --max-instances=1
     ```
3. Optionally manage secrets via `--set-secrets` or the console’s Secret Manager integration instead of inline values.

The `.dockerignore` file keeps `.env` (and other local-only artifacts) out of the build context, so Cloud Build never needs your Telegram or Nova credentials during image creation.

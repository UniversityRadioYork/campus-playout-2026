# Campus Playout 2026

Yet another Campus Playout implementation. Will it succeed? Who knows!

![](docs/screenshot.png)

## About

### Stack

- [Liquidsoap](https://www.liquidsoap.info) (actually does the audio stuff)
- [Rust](https://rust-lang.org) + [axum](https://docs.rs/axum/latest/axum/) (backend)
- [sqlite](https://sqlite.org) (database)
- [maud](https://maud.lambda.xyz) (HTML templating)
- [htmx](https://htmx.org) (frontend interactivity)
- [Bootstrap](https://getbootstrap.com) (UI styling)

## Development

### You will need

- A [last.fm](https://last.fm) API key
- A MyRadio API key
- A rust toolchain
- Liquidsoap
- The sqlx-cli (`cargo install sqlx-cli`)

### Configuration

Create a `.env` file with the following content:

```
DATABASE_URL=sqlite:database.db

INSTANCE_NAME="Test Venue"

LAST_FM_API_KEY=

MYRADIO_API_BASE=https://ury.org.uk/api/v2
MYRADIO_API_KEY=

DEFAULT_PLAYLIST_ID=pop-
PLAYLIST_FILE=./test/playlist-gen.txt

API_TOKEN=changeme

```

Loading the `.env` file is up to you.

### Database

Create the database with `sqlx database create`.

### Running

Run the Rust control server with `cargo run`.

Start the liquidsoap script with `liquidsoap scripts/playout.liq`.

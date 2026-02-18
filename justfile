default:
    just --list

[group("control server")]
serve:
    cargo run

[group("database")]
database-setup:
    cargo sqlx database create
    just database-migrate

[group("database")]
database-migrate:
    cargo sqlx migrate run

[group("streamer")]
streamer:
    liquidsoap scripts/playout.liq

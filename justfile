alias r := run
alias w := watch

# default debugging run
run:
    cargo run --target wasm32-unknown-unknown

# debugging watch
watch:
    cargo watch -cx "run --target wasm32-unknown-unknown"

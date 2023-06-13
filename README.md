This repository demonstrates an issue with `tokio`:

When a task is run within a `LocalSet`, if any struct inside it uses `tokio::spawn` in its `Drop` implementation then a panic occurs.

Example output:

```bash
# works:
❯ cargo run --quiet -- select
cancelling runtime after 1 second...
runtime exited

# works:
❯ cargo run --quiet -- spawn
cancelling runtime after 1 second...
runtime exited

# PANICS:
❯ cargo run --quiet -- spawn_local
cancelling runtime after 1 second...
thread 'main' panicked at 'there is no reactor running, must be called from the context of a Tokio 1.x runtime', src/main.rs:61:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
runtime exited
```
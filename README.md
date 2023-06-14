When running an application which is trying to be completely single-threaded (by using `tokio`'s `current_thread` runtime, and then running everything within a `LocalSet` to allow for `!Send` types) there is some unexpected behaviour when the futures are dropped:

When `tokio::task::spawn` is used in a struct's `Drop` implementation - it panics with `there is no reactor running, must be called from the context of a Tokio 1.x runtime` if the parent task was spawned with `tokio::task::spawn_local`:


| task started with          | spawn used in `Drop`       | panic                                 | expected |
| -------------------------- | -------------------------- | ------------------------------------- | -------- |
| `tokio::task::spawn`       | `tokio::task::spawn`       | No                                    | Yes ✅   |
| `tokio::task::spawn`       | `tokio::task::spawn_local` | Yes: if not running within `LocalSet` | Yes ✅   |
| `tokio::task::spawn_local` | `tokio::task::spawn`       | Yes: `no reactor running`             | No ❌    |
| `tokio::task::spawn_local` | `tokio::task::spawn_local` | No                                    | Yes ✅   |

This repository runs each of the combinations in the above table:

```bash
$ cargo run --quiet
SpawnMethod::Select + DropMethod::Spawn

SpawnMethod::Select + DropMethod::SpawnLocal

SpawnMethod::Spawn + DropMethod::Spawn

SpawnMethod::Spawn + DropMethod::SpawnLocal
thread 'main' panicked at '`spawn_local` called from outside of a `task::LocalSet`', src/main.rs:88:39
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

SpawnMethod::SpawnLocal + DropMethod::Spawn
thread 'main' panicked at 'there is no reactor running, must be called from the context of a Tokio 1.x runtime', src/main.rs:87:34

SpawnMethod::SpawnLocal + DropMethod::SpawnLocal
```
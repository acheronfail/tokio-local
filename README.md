When running an application which is trying to be completely single-threaded (by using `tokio`'s `current_thread` runtime, and then running everything within a `LocalSet` to allow for `!Send` types) there is some unexpected behaviour when the futures are dropped:

When `tokio::task::spawn` is used in a struct's `Drop` implementation - it panics with `there is no reactor running, must be called from the context of a Tokio 1.x runtime` if the parent task was spawned with `tokio::task::spawn_local`:


| task started with          | spawn used in `Drop`       | panic                                 | expected |
| -------------------------- | -------------------------- | ------------------------------------- | -------- |
| `tokio::task::spawn`       | `tokio::task::spawn`       | No                                    | Yes ✅   |
| `tokio::task::spawn`       | `tokio::task::spawn_local` | Yes: if not running within `LocalSet` | Yes ✅   |
| `tokio::task::spawn_local` | `tokio::task::spawn`       | Yes: `no reactor running`             | No ❌    |
| `tokio::task::spawn_local` | `tokio::task::spawn_local` | No                                    | Yes ✅   |

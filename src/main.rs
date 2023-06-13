use std::{env, time::Duration};

use tokio::{
    runtime::Builder,
    task::{self, LocalSet},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

fn main() {
    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    LocalSet::new().block_on(&runtime, async_main());

    println!("runtime exited");
}

async fn async_main() {
    // cancellation token - used to stop the runtime
    let cancel = CancellationToken::new();

    // after a timeout, cancel the runtime
    task::spawn_local({
        let cancel = cancel.clone();
        async move {
            println!("cancelling runtime after 1 second...");
            sleep(Duration::from_secs(1)).await;
            cancel.cancel();
        }
    });

    match env::args().skip(1).next().unwrap().as_str() {
        // NOTE: this one panics on drop
        "spawn_local" => {
            task::spawn_local(spawn_task());
            cancel.cancelled().await;
        }
        // NOTE: this one *does not panic* on drop
        "spawn" => {
            task::spawn(spawn_task());
            cancel.cancelled().await;
        }
        // NOTE: this one *does not panic* on drop
        "select" => {
            tokio::select! {
                _ = spawn_task() => {},
                _ = cancel.cancelled() => {},
            };
        }
        _ => {
            println!("Pass either 'select', 'spawn' or 'spawn_local'");
            std::process::exit(1);
        }
    };
}

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        task::spawn(async {
            println!("dropping");
            sleep(Duration::from_millis(100)).await;
            println!("dropped");
        });
    }
}

async fn spawn_task() {
    let _foo = Foo;

    futures::future::pending::<()>().await;
}

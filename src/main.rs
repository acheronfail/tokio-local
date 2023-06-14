use std::time::Duration;

use tokio::{
    runtime::Builder,
    task::{self, LocalSet},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Copy, Clone)]
enum SpawnMethod {
    Spawn,
    SpawnLocal,
    Select,
}

#[derive(Debug, Copy, Clone)]
enum DropMethod {
    Spawn,
    SpawnLocal,
}

fn main() {
    for spawn_method in &[
        SpawnMethod::Select,
        SpawnMethod::Spawn,
        SpawnMethod::SpawnLocal,
    ] {
        for drop_method in &[DropMethod::Spawn, DropMethod::SpawnLocal] {
            println!(
                "SpawnMethod::{:?} + DropMethod::{:?}",
                spawn_method, drop_method
            );

            {
                let runtime = Builder::new_current_thread().enable_all().build().unwrap();
                LocalSet::new().block_on(&runtime, async_main(*spawn_method, *drop_method));

                // NOTE: this prevents any panics from occurring:
                // runtime.block_on(async {
                //     LocalSet::new()
                //         .run_until(async_main(*spawn_method, *drop_method))
                //         .await;
                // });

                // NOTE: interestingly, this doesn't though:
                // runtime
                //     .block_on(LocalSet::new().run_until(async_main(*spawn_method, *drop_method)));
            }

            println!();
        }
    }
}

async fn async_main(spawn_method: SpawnMethod, drop_method: DropMethod) {
    // cancellation token - used to stop the runtime
    let cancel = CancellationToken::new();

    // after a timeout, cancel the runtime
    task::spawn_local({
        let cancel = cancel.clone();
        async move {
            sleep(Duration::from_millis(10)).await;
            cancel.cancel();
        }
    });

    let fut = spawn_task(drop_method);
    match spawn_method {
        // NOTE: this one panics on drop
        SpawnMethod::SpawnLocal => {
            task::spawn_local(fut);
            cancel.cancelled().await;
        }
        // NOTE: this one *does not panic* on drop
        SpawnMethod::Spawn => {
            task::spawn(fut);
            cancel.cancelled().await;
        }
        // NOTE: this one *does not panic* on drop
        SpawnMethod::Select => {
            tokio::select! {
                _ = fut => {},
                _ = cancel.cancelled() => {},
            };
        }
    };
}

struct Foo(DropMethod);

impl Drop for Foo {
    fn drop(&mut self) {
        // NOTE: this future is never run anyway
        let drop_future = async { unreachable!() };
        match self.0 {
            DropMethod::Spawn => task::spawn(drop_future),
            DropMethod::SpawnLocal => task::spawn_local(drop_future),
        };
    }
}

async fn spawn_task(drop_method: DropMethod) {
    let _foo = Foo(drop_method);
    futures::future::pending::<()>().await;
}

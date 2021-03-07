use std::time::{Duration, Instant};

use toykio_runtime::{Delay, Toykio};
use warp::Filter;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let mut rt = Toykio::new();

        rt.spawn(async {
            println!("Spawned");
            let when = Instant::now() + Duration::from_millis(1500);
            let future = Delay { when };
            println!("Wait 1.5sec...");
            let out = future.await;
            println!("Done");
            assert_eq!(out, "done");
        });

        rt.run();
    });

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

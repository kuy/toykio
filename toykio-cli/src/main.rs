use std::time::{Duration, Instant};

use toykio_runtime::{Delay, Toykio};

fn main() {
    let mut runtime = Toykio::new();

    runtime.spawn(async {
        println!("Spawned");
        let when = Instant::now() + Duration::from_millis(1500);
        let future = Delay { when };
        println!("Wait 1.5sec...");
        let out = future.await;
        println!("Done");
        assert_eq!(out, "done");
    });

    runtime.run();
}

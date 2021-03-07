use std::time::{Duration, Instant};

use toykio_runtime::{Delay, Toykio};

fn main() {
    let mut runtime = Toykio::new();

    runtime.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;
        assert_eq!(out, "done");
    });

    runtime.run();
}

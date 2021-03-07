use std::convert::Infallible;
use std::time::{Duration, Instant};

use tokio::sync::{mpsc, oneshot};
use toykio_runtime::{Delay, Req, Toykio};
use warp::Filter;

pub async fn on_request(tx: mpsc::Sender<Req>) -> Result<impl warp::Reply, Infallible> {
    let (res_tx, res_rx) = oneshot::channel();
    let _ = tx.send(Req { resp: res_tx }).await;
    println!("HOST: sent");
    match res_rx.await {
        Ok(count) => {
            println!("count = {}", count);
            Ok(format!("count = {}", count))
        }
        Err(_) => {
            println!("ERROR: failed to receive response");
            Ok("ERROR: failed to receive response".into())
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Req>(32);

    tokio::spawn(async move {
        let mut rt = Toykio::new();

        rt.spawn(async {
            println!("Spawned");
            let when = Instant::now() + Duration::from_millis(100);
            let future = Delay { when };
            println!("Wait 100ms...");
            let out = future.await;
            println!("Done");
            assert_eq!(out, "done");
        });

        rt.run(rx);
    });

    let handler = warp::path!("toykio")
        .and(warp::any().map(move || tx.clone()))
        .and_then(on_request);

    warp::serve(handler).run(([127, 0, 0, 1], 3030)).await;
}

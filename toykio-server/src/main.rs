use std::convert::Infallible;
use std::time::{Duration, Instant};

use tokio::sync::{mpsc, oneshot};
use toykio_runtime::{Delay, Toykio};
use warp::Filter;

type Responder<T> = oneshot::Sender<T>;

pub struct Req {
    pub resp: Responder<usize>,
}

pub async fn on_request(tx: mpsc::Sender<Req>) -> Result<impl warp::Reply, Infallible> {
    let (res_tx, res_rx) = oneshot::channel();
    let _ = tx.send(Req { resp: res_tx }).await;
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
    let (tx, mut rx) = mpsc::channel::<Req>(32);

    tokio::spawn(async move {
        let mut rt = Toykio::new();

        tokio::spawn(async move {
            while let Some(req) = rx.recv().await {
                let _ = req.resp.send(10);
            }
        });

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

    let handler = warp::path!("toykio")
        .and(warp::any().map(move || tx.clone()))
        .and_then(on_request);

    warp::serve(handler).run(([127, 0, 0, 1], 3030)).await;
}

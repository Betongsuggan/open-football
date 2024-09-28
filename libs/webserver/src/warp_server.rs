use super::*;
use tokio::{signal, sync::oneshot};
use warp::Filter;

#[derive(Default)]
pub struct WarpWebserver {
    //filter: dyn Filter<(), Rejection>,
}

impl WarpWebserver {
    pub fn new() -> WarpWebserver {
        WarpWebserver {
            //filter: warp::path::end(),
        }
    }
}

impl Webserver for WarpWebserver {
    fn listen(&self) -> Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        let hello = warp::path::end().map(|| "Hello, Warp!");

        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        let graceful_shutdown = async {
            signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
            shutdown_tx
                .send(())
                .expect("Failed to send shutdown signal");
        };

        let (addr, server_future) =
            warp::serve(hello).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async {
                shutdown_rx.await.ok();
            });

        println!("Server running on {}", addr);

        Box::pin(async {
            tokio::select! {
                _ = server_future => (),
                _ = graceful_shutdown => {
                    println!("Server shutting down...");
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

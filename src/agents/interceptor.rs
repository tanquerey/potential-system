use tokio::sync::{broadcast};
use tokio::task;

pub fn spawn_interceptor(mut rx: broadcast::Receiver<String>) {
    task::spawn(async move {
        while let Ok(data) = rx.recv().await {
            println!("Interceptor launched due to {}", data);
        }
    });
}

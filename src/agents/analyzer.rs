use tokio::sync::{broadcast};
use tokio::task;

pub fn spawn_analyzer(mut rx: broadcast::Receiver<String>) {
    task::spawn(async move {
        while let Ok(data) = rx.recv().await {
            println!("Analyzer logging input: {}", data);
        }
    });
}

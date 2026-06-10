use tokio::sync::broadcast;

use crate::fusion::FusedMessage;

pub struct Analyzer {
    pub cycle_count: u32,
    pub ignored_count: u32,
}

impl Analyzer {
    pub fn spawn(mut rx: broadcast::Receiver<FusedMessage>) {
        tokio::spawn(async move {
            let mut analyzer = Analyzer { cycle_count: 0, ignored_count: 0 };

            while let Ok(msg) = rx.recv().await {
                analyzer.cycle_count += 1;

                match msg {
                    FusedMessage::Combined(data) => {
                        println!("Analyzer logging combined input: {}", data);
                    }
                    _ => {
                        analyzer.ignored_count += 1;
                    }
                }

                if analyzer.cycle_count % 10 == 0 {
                    println!(
                        "Analyzer report: {} cycles, {} ignored messages",
                        analyzer.cycle_count, analyzer.ignored_count
                    );
                }
            }
        });
    }
}

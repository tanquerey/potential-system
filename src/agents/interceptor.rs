use tokio::sync::broadcast;
use tokio::task;
use crate::fusion::FusedMessage;
use std::time::{SystemTime, Duration};

pub struct Interceptor {
    pub radar_events: Vec<SystemTime>,
    pub window_size: usize,
}

impl Interceptor {
    pub fn spawn(mut rx: broadcast::Receiver<FusedMessage>) {
        tokio::spawn(async move {
            let mut interceptor = Interceptor {
                radar_events: Vec::new(),
                window_size: 5,
            };

            while let Ok(msg) = rx.recv().await {
                match msg {
                    FusedMessage::Radar(data) => {
                        interceptor.radar_events.push(SystemTime::now());

                        // keep only last N events
                        if interceptor.radar_events.len() > interceptor.window_size {
                            interceptor.radar_events.remove(0);
                        }

                        println!("Interceptor noted radar activity: {}", data);
                    }
                    FusedMessage::Camera(data) => {
                        // count radar events in last 10 seconds
                        let now = SystemTime::now();
                        let recent_hits = interceptor.radar_events
                            .iter()
                            .filter(|&&t| now.duration_since(t).unwrap() < Duration::from_secs(10))
                            .count();

                        if recent_hits >= 2 {
                            println!("Interceptor LAUNCH: camera saw {}, radar active recently!", data);
                        } else {
                            println!("Interceptor ignored camera: {}", data);
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}

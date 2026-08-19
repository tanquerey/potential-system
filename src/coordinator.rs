use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::agents::interceptor::ClaimedTargets;
use crate::agents::{Agent, AgentType, Interceptor, MissionEntry};
use crate::event::MissionEvent::{self, Intercept};
use crate::event::SourcedEvent;
use tokio::sync::mpsc;
use tokio::time::{Instant, Interval, interval};
use tokio_util::sync::CancellationToken;

pub struct Coordinator {
    agents: Vec<AgentType>,
}

impl Coordinator {
    pub fn new() -> Self {
        Coordinator { agents: Vec::new() }
    }

    pub fn add_agent(&mut self, agent: AgentType) {
        self.agents.push(agent);
    }

    pub async fn run(
        &mut self,
        mut receiver: mpsc::Receiver<SourcedEvent>,
        shutdown: CancellationToken,
    ) {
        let targets: ClaimedTargets = Arc::new(Mutex::new(HashSet::new()));

        let mut interceptors = vec![
            Interceptor {
                id: 3,
                claimed: targets.clone(),
            },
            Interceptor {
                id: 4,
                claimed: targets.clone(),
            },
        ];
        let mut watchdog_tick: Interval = interval(Duration::from_secs(5));
        let stale_after = Duration::from_secs(10);

        loop {
            tokio::select! {
                maybe_event = receiver.recv() => {
                    match maybe_event {
                        Some(sourced) => self.handle_event(sourced, &mut interceptors),
                        None => break, // all senders dropped, channel closed — shut down
                    }
                }
                _ = shutdown.cancelled() => {
                    println!("Graceful shutdown of drone");
                    break;
                }
                _ = watchdog_tick.tick() => {
                    self.check_watchdog(stale_after);
                }
            }
        }
    }

    fn handle_event(&mut self, sourced: SourcedEvent, interceptors: &mut Vec<Interceptor>) {
        for agent in &mut self.agents {
            if agent.id() == sourced.drone_id {
                match agent.act(&sourced.event) {
                    Ok(entry) => Self::log_entry(&entry, interceptors),
                    Err(e) => eprintln!("Agent {} failed to act: {:?}", agent.id(), e),
                }
            }
        }
    }

    fn log_entry(entry: &MissionEntry, interceptors: &mut Vec<Interceptor>) {
        match &entry.event {
            MissionEvent::Alert(tracking) => match tracking.clone().intercept() {
                Some(intercepting) => {
                    for interceptor in interceptors {
                        if interceptor.try_claim(intercepting.target_id) {
                            match interceptor.act(&Intercept(intercepting.clone())) {
                                Ok(_) => break, // claimed and acted — stop trying other interceptors
                                Err(e) => eprintln!("Interceptor failed: {:?}", e),
                            }
                        } else {
                            eprintln!("Target already acquired: {:?}", intercepting);
                            break;
                        }
                    }
                }
                None => {
                    println!(
                        "ALERT from Agent {}: target {:?} not confident enough to intercept",
                        entry.agent_id, tracking
                    );
                }
            },
            MissionEvent::Radar(msg) => println!("Radar from Agent {}: {:?}", entry.agent_id, msg),
            MissionEvent::Camera(msg) => {
                println!("Camera from Agent {}: {:?}", entry.agent_id, msg)
            }
            MissionEvent::Command(msg) => {
                println!("Command executed by Agent {}: {}", entry.agent_id, msg)
            }
            MissionEvent::Idle(msg) => {
                println!("Agent {} is Idle with msg:{}", entry.agent_id, msg)
            }
            MissionEvent::Intercept(intercepting) => {
                println!("Agent {} intercepted {:?}", entry.agent_id, intercepting)
            }
        }
    }

    fn check_watchdog(&self, stale_after: Duration) {
        let now = Instant::now();
        for agent in &self.agents {
            if let Some(seen_at) = agent.last_seen() {
                if now.duration_since(seen_at) > stale_after {
                    eprintln!(
                        "Agent {} appears lost — last seen {:?} ago",
                        agent.id(),
                        now.duration_since(seen_at)
                    );
                }
            }
        }
    }
}

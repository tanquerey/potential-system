use std::time::Duration;

use crate::agents::{Agent, AgentType, Interceptor, MissionEntry};
use crate::event::MissionEvent::{self, Intercept};
use crate::event::SourcedEvent;
use tokio::sync::mpsc;
use tokio::time::{Instant, Interval, interval};

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

    pub async fn run(&mut self, mut receiver: mpsc::Receiver<SourcedEvent>) {
        let mut interceptor = Interceptor { id: 3 };
        let mut watchdog_tick: Interval = interval(Duration::from_secs(5));
        let stale_after = Duration::from_secs(10);

        loop {
            tokio::select! {
                maybe_event = receiver.recv() => {
                    match maybe_event {
                        Some(sourced) => self.handle_event(sourced, &mut interceptor),
                        None => break, // all senders dropped, channel closed — shut down
                    }
                }
                _ = watchdog_tick.tick() => {
                    self.check_watchdog(stale_after);
                }
            }
        }
    }

    fn handle_event(&mut self, sourced: SourcedEvent, interceptor: &mut Interceptor) {
        for agent in &mut self.agents {
            if agent.id() == sourced.drone_id {
                match agent.act(&sourced.event) {
                    Ok(entry) => Self::log_entry(&entry, interceptor),
                    Err(e) => eprintln!("Agent {} failed to act: {:?}", agent.id(), e),
                }
            }
        }
    }

    fn log_entry(entry: &MissionEntry, interceptor: &mut Interceptor) {
        match &entry.event {
            MissionEvent::Alert(tracking) => match tracking.clone().intercept() {
                Some(intercepting) => {
                    if let Err(e) = interceptor.act(&Intercept(intercepting)) {
                        eprintln!("Interceptor failed: {:?}", e);
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

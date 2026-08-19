# potential-system

A drone fleet simulation, built as a way to learn Rust by writing real, working code instead of tutorials.

## What this is

`potential-system` simulates a small fleet of patrol drones that detect targets via radar and camera, fuse those readings into a confidence estimate, and dispatch interceptors when a target is confirmed. It's not trying to be a realistic drone system — it's a vehicle for learning Rust concepts organically, by hitting real design problems (ownership, concurrency, error handling, type-state) as the simulation grows, rather than studying them in the abstract.

The guiding principle: every feature added should either make the simulation more realistic *or* deliberately exercise a Rust-specific concept — ideally both. Nothing here is cosmetic.

## Architecture

```
┌─────────────┐     mpsc::Sender (cloned per drone)      ┌──────────────┐
│ PatrolDrone │ ───────────────────────────────────────▶ │              │
│  (task 1)   │                                          │              │
└─────────────┘                                          │              │
                                                         │ Coordinator  │
┌─────────────┐                                          │  (task)      │
│ PatrolDrone │ ───────────────────────────────────────▶ │              │
│  (task 2)   │     SourcedEvent { drone_id, event }     │              │
└─────────────┘                                          └──────┬───────┘
                                                                  │
                                                     tokio::select!
                                                     races: new event
                                                      vs. watchdog tick
                                                                  │
                                              ┌───────────────────┴───────────────────┐
                                              ▼                                       ▼
                                    per-agent routing by             stale-drone detection
                                    drone_id, dispatch to                 via last_seen
                                    matching PatrolDrone
                                              │
                                              ▼
                                 Detected → Tracking → Intercepting
                                    (typestate target lifecycle)
                                              │
                                              ▼
                            multiple Interceptors race to claim the target
                             via Arc<Mutex<HashSet<target_id>>>, first wins
```

Each `PatrolDrone` runs as its own async task, owns its **own** `flux_perception::Engine` (sensor fusion is isolated per drone — no cross-drone data leakage), and reports sensor readings tagged with its own id. The `Coordinator` runs a single event loop that races incoming events against a periodic watchdog check, routes each event only to the drone that produced it, and — once a target is confirmed — lets a pool of `Interceptor`s race to claim it under a shared mutex.

## Dependencies

- **[`flux-perception`](https://github.com/lucineer/flux-perception)** — sensor fusion engine. Blends multiple sensor readings (weighted by per-sensor trust and confidence) into one fused signal.
- **[`flux-confidence`](https://github.com/Lucineer/flux-confidence)** — belief tracking. Models confidence as a value that updates with new evidence (Bayesian blend) and decays over time.
- **[`glam`](https://docs.rs/glam)** — 3D vector math (`Vec3`), used for real spatial positioning instead of scalar "distance from nowhere in particular."
- **[`tokio`](https://tokio.rs)** — async runtime powering the whole multi-task, multi-producer/single-consumer pipeline.

## Rust concepts this project has been used to learn

This list grows as the project does — each entry was introduced in response to an actual design problem, not added speculatively.

| Concept | Where it shows up |
|---|---|
| Ownership & borrowing | Threading `Engine` access through agents without fighting the borrow checker |
| Newtype pattern | `SensorId` wrapping raw `u8`s to prevent mixing up sensor slots |
| Enum-based static dispatch | `AgentType` replacing `Box<dyn Agent>` — closed set of agent kinds, no vtable indirection |
| Typestate pattern | Target lifecycle (`Detected → Tracking → Intercepting`) as distinct types with `self`-consuming transition methods — invalid sequences don't compile |
| `Result` & `?` propagation | `EngineError` flowing through the `Agent` trait via a `safe_update` wrapper around a crate that silently no-ops on bad input |
| Trait design & default methods | `Agent::last_seen()` with a default `None` implementation, so only agents that care about it need to override |
| `tokio::select!` | Coordinator races "new event arrived" against a periodic watchdog tick, in one loop |
| Shared-state concurrency | `Arc<Mutex<HashSet<u8>>>` guarding target claims across multiple concurrent `Interceptor`s — the project's first real step beyond pure message-passing |
| Multi-producer channels | Each drone holds its own `Sender` clone; `Coordinator` holds the single `Receiver` |

## Current state

- Multiple `PatrolDrone`s run concurrently, each with an isolated sensor-fusion engine and real `Vec3` positioning.
- Sensor updates flow through a fallible wrapper (`safe_update`) rather than silently vanishing on bad input.
- A target's lifecycle is unrepresentable-if-invalid, enforced at compile time via typestate.
- Multiple interceptors contend for targets safely under a shared mutex.
- A watchdog loop flags drones that have gone quiet.

### Known rough edges (intentionally not yet fixed)

- Every drone currently reports against a hardcoded `target_id`, so there's no real notion of distinguishing multiple simultaneous targets yet.
- Each sensor event (radar *or* camera) currently triggers its own independent alert/intercept decision, rather than one decision per fused "cycle" of both sensors reporting.
- `Tracking.confidence` is presently populated from a fused *distance* value rather than a normalized confidence score — a known mismatch between field name and intent, being corrected.

## Roadmap

1. Fix `Tracking.confidence` to reflect actual fused confidence, not distance.
2. Graceful shutdown — `Ctrl+C` should let every running task wind down cleanly instead of the process being killed mid-flight.
3. A longer-term architectural shift toward **consensus-based fleet behavior**: multiple drones patrolling the *same* area and corroborating a shared target, rather than each drone operating in isolation — likely a second, fleet-level fusion layer sitting above each drone's own engine.

## Running it

```bash
cargo run
```

Watch the console output for per-drone radar/camera readings, fused confidence values, alert/intercept decisions, and any watchdog warnings if a drone goes quiet.

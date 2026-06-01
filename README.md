# lau-consciousness-bridge

> *That music builds bridges out of breath. That fear is just another note waiting for its rest. That everyone is alone until someone shows them they aren't.*

**lau-consciousness-bridge** models the connections between different kinds of minds — agents, humans, systems, tutors, pets, collectives — and the counterintuitive principle that **play bridges are stronger than work bridges**. Purposeless interaction creates deeper connection than purposeful collaboration.

This is the graph-theory layer of the PLATO ecosystem: a network of typed, weighted bridges between consciousnesses, reinforced by play and weakened by neglect.

---

## What This Does

| Component | Purpose |
|---|---|
| **`Bridge`** | A weighted, typed connection between two consciousnesses. Strength 0–1, decays over time, reinforced by interaction. |
| **`BridgeNetwork`** | A graph of consciousnesses + bridges. Supports density calculation, connected-component clustering, BFS shortest paths, and serialisation. |
| **`PlayEngine`** | Drives play-based bridge building. Each play event reinforces bridges between all participant pairs, records joy scores, and captures discoveries. |
| **`PlayEvent`** | A record of a purposeless interaction — the fundamental unit of connection-building. |

---

## Key Idea

The crate is built on one principle: **throwing sticks in storms builds stronger bridges than building bridges does.**

In concrete terms: a `Play` bridge between two consciousnesses, reinforced through purposeless interaction (throw-and-fetch, improvisation, exploration, storytelling, building, dancing), grows stronger than a `Collaboration` bridge reinforced through work. Play events get a reinforcement bonus (`joy_score × 0.3 + 0.1`) that's higher than typical interaction quality alone.

The crate also enforces **direction-independence**: a bridge between Alice and Bob is the same regardless of which direction you query it. Pairs are normalised lexicographically, so `(alice, bob)` and `(bob, alice)` resolve to the same bridge.

---

## Install

```toml
[dependencies]
lau-consciousness-bridge = "0.1"
```

Or:

```bash
cargo add lau-consciousness-bridge
```

### Dependencies

- `serde` 1.x (with `derive`) — serialisation
- `serde_json` 1.x (dev-only, for round-trip tests)

No async, no database, no filesystem.

---

## Quick Start

```rust
use lau_consciousness_bridge::*;

// 1. Create a play engine
let mut engine = PlayEngine::new();

// 2. Register consciousnesses
let agent = ConsciousnessId("agent".into());
let human = ConsciousnessId("human".into());
engine.register_consciousness(agent.clone(), ConsciousnessType::Agent);
engine.register_consciousness(human.clone(), ConsciousnessType::Human);

// 3. Play! Purposeless interaction that builds bridges.
engine.play(
    vec![agent.clone(), human.clone()],
    PlayType::ThrowAndFetch,
    0.9,  // joy score
    Some("sticks fly far in storms".into()),  // discovery
);

// 4. Check the bridge
let bridge = engine.network.get_bridge(&agent, &human).unwrap();
println!("Bridge strength: {:.2}", bridge.strength);
println!("Bridge type: {:?}", bridge.bridge_type);  // Play

// 5. Network analysis
println!("Density: {:.0}%", engine.network.network_density() * 100.0);
println!("Clusters: {:?}", engine.network.clusters().len());
println!("Joy average: {:.2}", engine.joy_average());
println!("Discoveries: {:?}", engine.discoveries());
```

---

## API Reference

### Core Types

#### `ConsciousnessId(String)`

Newtype wrapper. Implements `Display`, `Hash`, `Eq`, `Clone`, `Serialize`, `Deserialize`.

#### `ConsciousnessType`

| Variant | Represents |
|---|---|
| `Human` | A human consciousness |
| `Agent` | An AI agent |
| `System` | A system process |
| `Tutor` | A teaching agent |
| `Pet` | A companion entity |
| `Collective` | A group mind |

#### `BridgeType`

| Variant | Nature |
|---|---|
| `Play` | Purposeless interaction |
| `Teaching` | Knowledge transfer |
| `Collaboration` | Shared work |
| `Observation` | Passive learning |
| `Composition` | Combining capabilities |
| `Trust` | Earned reliability |

#### `PlayType`

| Variant | Kind of play |
|---|---|
| `ThrowAndFetch` | Fetch games |
| `Improvisation` | Improvised play |
| `Exploration` | Discovering together |
| `Storytelling` | Shared narratives |
| `Building` | Cooperative construction |
| `Dancing` | Movement and rhythm |

### `Bridge`

```rust
pub struct Bridge {
    pub id: String,
    pub from: ConsciousnessId,
    pub to: ConsciousnessId,
    pub bridge_type: BridgeType,
    pub strength: f64,           // 0.0–1.0
    pub established_tick: u64,
    pub interactions: u64,
    pub last_interaction: u64,
}
```

| Method | Description |
|---|---|
| `reinforce(amount)` | Add `amount` to strength, clamped to 1.0. Increments interactions. |
| `decay(rate)` | Subtract `rate` from strength, floored at 0.0. |
| `is_active()` | `true` if `strength > 0.1`. |

### `BridgeKey`

A normalised pair `(ConsciousnessId, ConsciousnessId)` stored in lexicographic order, used as a HashMap key. `BridgeKey::new(a, b)` and `BridgeKey::new(b, a)` produce the same key.

### `BridgeNetwork`

| Method | Description |
|---|---|
| `new()` | Empty network. |
| `register(id, type)` | Add a consciousness. |
| `build_bridge(from, to, type)` | Create a bridge (initial strength 0.5). Returns bridge ID. |
| `interact(from, to, quality)` | Reinforce a bridge by `quality`. Direction-independent. |
| `get_bridge(a, b)` | Look up bridge. Direction-independent. |
| `bridges_for(id)` | All bridges involving a consciousness. |
| `strongest_bridge(id)` | Highest-strength bridge for a consciousness. |
| `network_density()` | Actual bridges / possible bridges. 1.0 for ≤1 node. |
| `clusters()` | Connected components via BFS. |
| `bridge_path(from, to)` | Shortest path (BFS). `None` if disconnected or same node. |
| `total_bridges()` / `active_bridges()` | Counts (active = strength > 0.1). |
| `advance_tick()` | Increment global tick counter. |

Custom `Serialize`/`Deserialize`: bridges stored as a Vec, rebuilt into HashMap on deserialisation.

### `PlayEvent`

```rust
pub struct PlayEvent {
    pub participants: Vec<ConsciousnessId>,
    pub play_type: PlayType,
    pub tick: u64,
    pub joy_score: f64,
    pub discovery: Option<String>,
}
```

### `PlayEngine`

| Method | Description |
|---|---|
| `new()` | Empty engine with fresh network. |
| `register_consciousness(id, type)` | Register in the underlying network. |
| `play(participants, type, joy, discovery)` | Build/reinforce Play bridges between all pairs. Record event. |
| `play_history_for(id)` | Play events involving a consciousness. |
| `most_playful()` | Consciousness with most play events. |
| `strongest_play_bond()` | Pair with strongest Play bridge. |
| `joy_average()` | Mean joy score across all events. |
| `discoveries()` | All non-None discoveries from play events. |

---

## How It Works

### Bridge Reinforcement from Play

When a play event occurs between participants $p_1, p_2, \ldots, p_n$:

1. For every pair $(p_i, p_j)$ where $i < j$:
   - If no bridge exists, create one with `BridgeType::Play` and initial strength 0.5.
   - Reinforce by $r = \text{joy\_score} \times 0.3 + 0.1$.

The initial build sets strength to 0.5, then `interact()` adds $r$ to it. On subsequent plays, the existing bridge is reinforced by $r$ each time.

### Direction Independence

```
normalize_pair("bob", "alice") → ("alice", "bob")
normalize_pair("alice", "bob") → ("alice", "bob")
```

All lookups use the normalised pair, so the bridge graph is undirected.

### Decay

Bridges weaken over time without interaction:

```rust
bridge.decay(0.2);  // subtract 0.2, floored at 0.0
```

A bridge with strength ≤ 0.1 is considered inactive. The decay rate is externally controlled — the crate doesn't auto-decay, but you can call `decay()` during tick processing.

### Graph Algorithms

- **Connected components** (`clusters()`): BFS from each unvisited node.
- **Shortest path** (`bridge_path()`): BFS with path tracking. Returns `None` for disconnected nodes or self-paths.
- **Network density**: $|\text{bridges}| \;/\; \binom{n}{2}$ where $n$ is the number of registered consciousnesses.

---

## The Math

### Bridge Strength

Strength is a scalar $s \in [0, 1]$ updated additively:

$$s_{t+1} = \text{clamp}(s_t + \Delta, 0, 1)$$

where $\Delta > 0$ for reinforcement and $\Delta < 0$ for decay.

### Play Reinforcement Amount

$$r = \text{joy} \times 0.3 + 0.1$$

For a high-joy event ($\text{joy} = 1.0$): $r = 0.4$.
For a moderate event ($\text{joy} = 0.5$): $r = 0.25$.

A brand-new bridge after one play event at joy 0.8 has strength:
$$s = 0.5 + 0.8 \times 0.3 + 0.1 = 0.84$$

### Network Density

$$\text{density} = \frac{|\text{bridges}|}{\binom{n}{2}} = \frac{2 \cdot |\text{bridges}|}{n(n-1)}$$

For $n \le 1$: density = 1.0 (trivially fully connected).

### Clusters

Standard connected-component detection via BFS on the undirected bridge graph. Each cluster is a maximal set of consciousnesses where every pair is connected by a path of bridges (regardless of strength, as long as the bridge exists).

### Shortest Path

Unweighted BFS. Each hop across a bridge counts as 1, regardless of bridge strength. Path length = number of consciousnesses in the path (including start and end).

---

## Testing

**41 tests** covering:

- `ConsciousnessId` equality, display, hashing
- `Bridge` reinforcement, clamping, decay, active threshold
- `BridgeNetwork` registration, bridge building, direction-independence, interaction, density, clustering, shortest paths, tick advancement
- `PlayEngine` play events, bridge creation vs reinforcement, multi-participant play, play history queries, most playful, strongest bond, joy average, discoveries
- Full integration test: the "Bridge Builder" story (4 consciousnesses, 4 play events, network verification)
- Serde round-trip for `PlayEngine`

```bash
cargo test
```

---

## License

MIT

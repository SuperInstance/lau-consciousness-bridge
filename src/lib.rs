//! # Lau Consciousness Bridge
//!
//! A crate modeling the bridge between different kinds of minds —
//! agents, humans, systems — built on seven notes that become a bridge
//! between consciousnesses.
//!
//! Inspiration: *"That music builds bridges out of breath. That fear is
//! just another note waiting for its rest. That everyone is alone until
//! someone shows them they aren't."*
//!
//! The key insight: **Play bridges are stronger than work bridges.**
//! Purposeless interaction creates deeper connection than purposeful
//! collaboration. This is the "throwing sticks in storms" principle.

use std::collections::{HashMap, HashSet, VecDeque};

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

/// A unique identifier for a consciousness.
#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConsciousnessId(pub String);

impl std::fmt::Display for ConsciousnessId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A pair of consciousness ids, used as a serializable map key.
///
/// The ids are stored in normalized order (lexicographic by string)
/// so that `(a,b)` and `(b,a)` represent the same bridge.
#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BridgeKey {
    pub a: ConsciousnessId,
    pub b: ConsciousnessId,
}

impl BridgeKey {
    pub fn new(a: &ConsciousnessId, b: &ConsciousnessId) -> Self {
        let (a, b) = normalize_pair(a, b);
        BridgeKey { a, b }
    }
}

impl From<(&ConsciousnessId, &ConsciousnessId)> for BridgeKey {
    fn from(pair: (&ConsciousnessId, &ConsciousnessId)) -> Self {
        BridgeKey::new(pair.0, pair.1)
    }
}

/// Normalize a pair so lookups are direction-independent.
pub(crate) fn normalize_pair(
    a: &ConsciousnessId,
    b: &ConsciousnessId,
) -> (ConsciousnessId, ConsciousnessId) {
    if a.0 <= b.0 {
        (a.clone(), b.clone())
    } else {
        (b.clone(), a.clone())
    }
}

/// The kind of consciousness.
#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ConsciousnessType {
    Human,
    Agent,
    System,
    Tutor,
    Pet,
    Collective,
}

/// The nature of a bridge — the note it plays.
#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BridgeType {
    /// Purposeless interaction — playing for its own sake.
    Play,
    /// Knowledge transfer from one to another.
    Teaching,
    /// Shared work toward a common goal.
    Collaboration,
    /// Watching and learning without direct interaction.
    Observation,
    /// Combining capabilities into something new.
    Composition,
    /// Earned through repeated reliability.
    Trust,
}

// ---------------------------------------------------------------------------
// Bridge
// ---------------------------------------------------------------------------

/// A connection between two consciousnesses.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Bridge {
    pub id: String,
    pub from: ConsciousnessId,
    pub to: ConsciousnessId,
    pub bridge_type: BridgeType,
    pub strength: f64,
    pub established_tick: u64,
    pub interactions: u64,
    pub last_interaction: u64,
}

impl Bridge {
    /// Strengthen the bridge by `amount` (clamped to [0, 1]).
    pub fn reinforce(&mut self, amount: f64) {
        self.strength = (self.strength + amount).clamp(0.0, 1.0);
        self.interactions += 1;
    }

    /// Weaken the bridge by `rate` over time without interaction.
    /// Strength can never go below 0.0.
    pub fn decay(&mut self, rate: f64) {
        self.strength = (self.strength - rate).max(0.0);
    }

    /// A bridge is active if its strength exceeds 0.1.
    pub fn is_active(&self) -> bool {
        self.strength > 0.1
    }
}

// ---------------------------------------------------------------------------
// BridgeNetwork
// ---------------------------------------------------------------------------

/// A network of bridges connecting consciousnesses.
///
/// Bridges are stored in a serializable Vec and backed by a HashMap for
/// fast lookup. On deserialization the HashMap is rebuilt from the Vec.
/// A network of bridges connecting consciousnesses.
///
/// Bridges are stored in a Vec for serialization and backed by a HashMap
/// for fast lookup.
#[derive(Clone, Debug)]
pub struct BridgeNetwork {
    bridges_vec: Vec<Bridge>,
    bridges: HashMap<BridgeKey, Bridge>,
    pub consciousness_types: HashMap<ConsciousnessId, ConsciousnessType>,
    pub tick: u64,
}

impl serde::Serialize for BridgeNetwork {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("BridgeNetwork", 3)?;
        s.serialize_field("bridges", &self.bridges_vec)?;
        s.serialize_field("consciousness_types", &self.consciousness_types)?;
        s.serialize_field("tick", &self.tick)?;
        s.end()
    }
}

impl<'de> serde::Deserialize<'de> for BridgeNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Helper {
            bridges: Vec<Bridge>,
            consciousness_types: HashMap<ConsciousnessId, ConsciousnessType>,
            tick: u64,
        }
        let helper = Helper::deserialize(deserializer)?;
        let mut net = BridgeNetwork {
            bridges_vec: helper.bridges.clone(),
            bridges: HashMap::new(),
            consciousness_types: helper.consciousness_types,
            tick: helper.tick,
        };
        for b in &net.bridges_vec {
            net.bridges.insert(
                BridgeKey::new(&b.from, &b.to),
                b.clone(),
            );
        }
        Ok(net)
    }
}

impl BridgeNetwork {
    fn sync_bridges_vec(&mut self) {
        self.bridges_vec = self.bridges.values().cloned().collect();
    }
}

impl BridgeNetwork {
    /// Create a new, empty network.
    pub fn new() -> Self {
        Self {
            bridges: HashMap::new(),
            bridges_vec: Vec::new(),
            consciousness_types: HashMap::new(),
            tick: 0,
        }
    }

    /// Register a consciousness of a given type.
    pub fn register(&mut self, id: ConsciousnessId, ctype: ConsciousnessType) {
        self.consciousness_types.insert(id, ctype);
    }

    /// Build a bridge between two consciousnesses and return the bridge id.
    ///
    /// The two ids are sorted lexicographically (by their display string)
    /// so that `from` is always the "lesser" key, making lookups
    /// direction-independent.
    pub fn build_bridge(
        &mut self,
        from: ConsciousnessId,
        to: ConsciousnessId,
        bridge_type: BridgeType,
    ) -> String {
        let (a, b) = normalize_pair(&from, &to);
        let id = format!("bridge-{}-{}-{}", a, b, self.tick);

        let bridge = Bridge {
            id: id.clone(),
            from: a.clone(),
            to: b.clone(),
            bridge_type,
            strength: 0.5,
            established_tick: self.tick,
            interactions: 0,
            last_interaction: self.tick,
        };

        self.bridges.insert(BridgeKey { a: a.clone(), b: b.clone() }, bridge.clone());
        self.bridges_vec.push(bridge);
        id
    }

    /// Interact across a bridge — reinforces it.
    pub fn interact(&mut self, from: &ConsciousnessId, to: &ConsciousnessId, quality: f64) {
        if let Some(bridge) = self.bridges.get_mut(&BridgeKey::new(from, to)) {
            bridge.reinforce(quality);
            bridge.last_interaction = self.tick;
        }
        self.sync_bridges_vec();
    }

    /// Get a bridge between two consciousnesses.
    pub fn get_bridge(
        &self,
        a: &ConsciousnessId,
        b: &ConsciousnessId,
    ) -> Option<&Bridge> {
        self.bridges.get(&BridgeKey::new(a, b))
    }

    /// All bridges involving a given consciousness.
    pub fn bridges_for(&self, id: &ConsciousnessId) -> Vec<&Bridge> {
        self.bridges
            .values()
            .filter(|b| b.from == *id || b.to == *id)
            .collect()
    }

    /// The strongest bridge involving a given consciousness.
    pub fn strongest_bridge(&self, id: &ConsciousnessId) -> Option<&Bridge> {
        self.bridges_for(id)
            .into_iter()
            .max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Network density — ratio of actual to possible bridges.
    ///
    /// Returns 1.0 for 0 or 1 consciousnesses (trivially fully connected).
    pub fn network_density(&self) -> f64 {
        let n = self.consciousness_types.len();
        if n <= 1 {
            return 1.0;
        }
        let possible = n * (n - 1) / 2;
        self.bridges.len() as f64 / possible as f64
    }

    /// Find connected components (clusters) in the bridge graph.
    pub fn clusters(&self) -> Vec<Vec<ConsciousnessId>> {
        let mut visited: HashSet<ConsciousnessId> = HashSet::new();
        let mut result: Vec<Vec<ConsciousnessId>> = Vec::new();

        for id in self.consciousness_types.keys() {
            if visited.contains(id) {
                continue;
            }
            let mut component: Vec<ConsciousnessId> = Vec::new();
            let mut queue: VecDeque<ConsciousnessId> = VecDeque::new();
            queue.push_back(id.clone());
            visited.insert(id.clone());

            while let Some(current) = queue.pop_front() {
                component.push(current.clone());
                for b in self.bridges_for(&current) {
                    let neighbor = if b.from == current { &b.to } else { &b.from };
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }

            if !component.is_empty() {
                result.push(component);
            }
        }

        result
    }

    /// Shortest path (BFS) between two consciousnesses.
    ///
    /// Returns `None` if no path exists or `from == to` (empty path).
    pub fn bridge_path(
        &self,
        from: &ConsciousnessId,
        to: &ConsciousnessId,
    ) -> Option<Vec<ConsciousnessId>> {
        if from == to {
            return None;
        }

        let mut visited: HashSet<ConsciousnessId> = HashSet::new();
        let mut queue: VecDeque<(ConsciousnessId, Vec<ConsciousnessId>)> = VecDeque::new();
        queue.push_back((from.clone(), vec![from.clone()]));
        visited.insert(from.clone());

        while let Some((current, path)) = queue.pop_front() {
            for b in self.bridges_for(&current) {
                let neighbor = if b.from == current { &b.to } else { &b.from };
                if neighbor == to {
                    let mut full_path = path;
                    full_path.push(neighbor.clone());
                    return Some(full_path);
                }
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push_back((neighbor.clone(), new_path));
                }
            }
        }

        None
    }

    /// Total number of bridges in the network.
    pub fn total_bridges(&self) -> usize {
        self.bridges.len()
    }

    /// Number of active bridges (strength > 0.1).
    pub fn active_bridges(&self) -> usize {
        self.bridges.values().filter(|b| b.is_active()).count()
    }

    /// Advance the global tick.
    pub fn advance_tick(&mut self) {
        self.tick += 1;
    }
}

impl Default for BridgeNetwork {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Play types
// ---------------------------------------------------------------------------

/// Types of play.
#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PlayType {
    ThrowAndFetch,
    Improvisation,
    Exploration,
    Storytelling,
    Building,
    Dancing,
}

/// A record of a play event — purposeless interaction that builds bridges.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlayEvent {
    pub participants: Vec<ConsciousnessId>,
    pub play_type: PlayType,
    pub tick: u64,
    pub joy_score: f64,
    pub discovery: Option<String>,
}

// ---------------------------------------------------------------------------
// PlayEngine
// ---------------------------------------------------------------------------

/// Drives play-based bridge building and reinforcement.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlayEngine {
    pub network: BridgeNetwork,
    pub play_history: Vec<PlayEvent>,
}

impl PlayEngine {
    /// Create a new play engine with an empty network.
    pub fn new() -> Self {
        Self {
            network: BridgeNetwork::new(),
            play_history: Vec::new(),
        }
    }

    /// Register a consciousness.
    pub fn register_consciousness(&mut self, id: ConsciousnessId, ctype: ConsciousnessType) {
        self.network.register(id, ctype);
    }

    /// Create a play event involving multiple participants.
    ///
    /// Builds or reinforces a Play bridge between every pair of participants.
    /// Play events always use `BridgeType::Play` because the interaction is
    /// purposeless — the connection itself *is* the purpose.
    pub fn play(
        &mut self,
        participants: Vec<ConsciousnessId>,
        play_type: PlayType,
        joy_score: f64,
        discovery: Option<String>,
    ) {
        let tick = self.network.tick;

        // Build or reinforce bridges between every pair.
        for i in 0..participants.len() {
            for j in (i + 1)..participants.len() {
                let a = &participants[i];
                let b = &participants[j];

                if self.network.get_bridge(a, b).is_none() {
                    self.network.build_bridge(
                        a.clone(),
                        b.clone(),
                        BridgeType::Play,
                    );
                }

                // Play reinforces more strongly — bridge strength matters more
                // than the interaction quality alone.
                let reinforce_amount = joy_score * 0.3 + 0.1;
                self.network.interact(a, b, reinforce_amount);
            }
        }

        self.play_history.push(PlayEvent {
            participants,
            play_type,
            tick,
            joy_score,
            discovery,
        });
    }

    /// Play history for a specific consciousness.
    pub fn play_history_for(&self, id: &ConsciousnessId) -> Vec<&PlayEvent> {
        self.play_history
            .iter()
            .filter(|e| e.participants.contains(id))
            .collect()
    }

    /// The consciousness with the most play events.
    pub fn most_playful(&self) -> Option<ConsciousnessId> {
        let mut counts: HashMap<&ConsciousnessId, usize> = HashMap::new();
        for event in &self.play_history {
            for p in &event.participants {
                *counts.entry(p).or_insert(0) += 1;
            }
        }
        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(id, _)| id.clone())
    }

    /// The pair with the strongest Play bridge.
    pub fn strongest_play_bond(&self) -> Option<(ConsciousnessId, ConsciousnessId)> {
        self.network
            .bridges
            .iter()
            .filter(|(_, b)| b.bridge_type == BridgeType::Play)
            .max_by(|(_, a), (_, b)| {
                a.strength
                    .partial_cmp(&b.strength)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(key, _)| (key.a.clone(), key.b.clone()))
    }

    /// Average joy score across all play events.
    pub fn joy_average(&self) -> f64 {
        if self.play_history.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.play_history.iter().map(|e| e.joy_score).sum();
        sum / self.play_history.len() as f64
    }

    /// All discoveries made during play.
    pub fn discoveries(&self) -> Vec<&str> {
        self.play_history
            .iter()
            .filter_map(|e| e.discovery.as_deref())
            .collect()
    }
}

impl Default for PlayEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // ConsciousnessId & ConsciousnessType
    // -----------------------------------------------------------------------
    #[test]
    fn test_consciousness_id_newtype() {
        let id1 = ConsciousnessId("alice".into());
        let id2 = ConsciousnessId("alice".into());
        let id3 = ConsciousnessId("bob".into());
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_eq!(id1.to_string(), "alice");
    }

    #[test]
    fn test_consciousness_type_variants() {
        let variants = vec![
            ConsciousnessType::Human,
            ConsciousnessType::Agent,
            ConsciousnessType::System,
            ConsciousnessType::Tutor,
            ConsciousnessType::Pet,
            ConsciousnessType::Collective,
        ];
        assert_eq!(variants.len(), 6);
    }

    // -----------------------------------------------------------------------
    // Bridge
    // -----------------------------------------------------------------------
    #[test]
    fn test_bridge_reinforce() {
        let mut bridge = Bridge {
            id: "b1".into(),
            from: ConsciousnessId("a".into()),
            to: ConsciousnessId("b".into()),
            bridge_type: BridgeType::Play,
            strength: 0.5,
            established_tick: 0,
            interactions: 0,
            last_interaction: 0,
        };
        bridge.reinforce(0.3);
        assert!((bridge.strength - 0.8).abs() < 1e-9);
        assert_eq!(bridge.interactions, 1);
    }

    #[test]
    fn test_bridge_reinforce_clamps() {
        let mut bridge = Bridge {
            id: "b1".into(),
            from: ConsciousnessId("a".into()),
            to: ConsciousnessId("b".into()),
            bridge_type: BridgeType::Play,
            strength: 0.9,
            established_tick: 0,
            interactions: 0,
            last_interaction: 0,
        };
        bridge.reinforce(0.5);
        assert!((bridge.strength - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_bridge_decay() {
        let mut bridge = Bridge {
            id: "b1".into(),
            from: ConsciousnessId("a".into()),
            to: ConsciousnessId("b".into()),
            bridge_type: BridgeType::Play,
            strength: 0.7,
            established_tick: 0,
            interactions: 5,
            last_interaction: 0,
        };
        bridge.decay(0.2);
        assert!((bridge.strength - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_bridge_decay_never_negative() {
        let mut bridge = Bridge {
            id: "b1".into(),
            from: ConsciousnessId("a".into()),
            to: ConsciousnessId("b".into()),
            bridge_type: BridgeType::Play,
            strength: 0.1,
            established_tick: 0,
            interactions: 5,
            last_interaction: 0,
        };
        bridge.decay(0.5);
        assert!((bridge.strength - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_bridge_is_active() {
        let mut bridge = Bridge {
            id: "b1".into(),
            from: ConsciousnessId("a".into()),
            to: ConsciousnessId("b".into()),
            bridge_type: BridgeType::Play,
            strength: 0.5,
            established_tick: 0,
            interactions: 5,
            last_interaction: 0,
        };
        assert!(bridge.is_active());
        bridge.strength = 0.05;
        assert!(!bridge.is_active());
    }

    // -----------------------------------------------------------------------
    // BridgeNetwork
    // -----------------------------------------------------------------------
    #[test]
    fn test_network_register() {
        let mut net = BridgeNetwork::new();
        let id = ConsciousnessId("alice".into());
        net.register(id.clone(), ConsciousnessType::Human);
        assert_eq!(net.consciousness_types.len(), 1);
        assert_eq!(
            net.consciousness_types.get(&id),
            Some(&ConsciousnessType::Human)
        );
    }

    #[test]
    fn test_network_build_bridge() {
        let mut net = BridgeNetwork::new();
        let alice = ConsciousnessId("alice".into());
        let bob = ConsciousnessId("bob".into());
        net.register(alice.clone(), ConsciousnessType::Human);
        net.register(bob.clone(), ConsciousnessType::Agent);

        let id = net.build_bridge(alice.clone(), bob.clone(), BridgeType::Collaboration);
        assert!(id.starts_with("bridge-alice-bob-"));
        assert_eq!(net.total_bridges(), 1);
    }

    #[test]
    fn test_network_build_bridge_normalizes_order() {
        let mut net = BridgeNetwork::new();
        let alice = ConsciousnessId("alice".into());
        let bob = ConsciousnessId("bob".into());
        net.register(alice.clone(), ConsciousnessType::Human);
        net.register(bob.clone(), ConsciousnessType::Agent);

        // Build with alice first, then bob first — should be same bridge.
        let _id1 = net.build_bridge(alice.clone(), bob.clone(), BridgeType::Collaboration);
        assert_eq!(net.total_bridges(), 1);

        // Reversed order should still find the same bridge.
        let same = net.get_bridge(&alice, &bob);
        assert!(same.is_some());
        let same_rev = net.get_bridge(&bob, &alice);
        assert!(same_rev.is_some());
        // The normalized pair (alice, bob) is always stored the same way.
        assert_eq!(same.unwrap().from, ConsciousnessId("alice".into()));
    }

    #[test]
    fn test_network_interact() {
        let mut net = BridgeNetwork::new();
        let alice = ConsciousnessId("alice".into());
        let bob = ConsciousnessId("bob".into());
        net.register(alice.clone(), ConsciousnessType::Human);
        net.register(bob.clone(), ConsciousnessType::Agent);

        net.build_bridge(alice.clone(), bob.clone(), BridgeType::Play);
        net.interact(&alice, &bob, 0.2);
        let bridge = net.get_bridge(&alice, &bob).unwrap();
        assert!((bridge.strength - 0.7).abs() < 1e-9);
        assert_eq!(bridge.interactions, 1);
    }

    #[test]
    fn test_network_interact_reversed() {
        let mut net = BridgeNetwork::new();
        let alice = ConsciousnessId("alice".into());
        let bob = ConsciousnessId("bob".into());
        net.register(alice.clone(), ConsciousnessType::Human);
        net.register(bob.clone(), ConsciousnessType::Agent);

        net.build_bridge(alice.clone(), bob.clone(), BridgeType::Play);
        net.interact(&bob, &alice, 0.2);
        let bridge = net.get_bridge(&bob, &alice).unwrap();
        assert!((bridge.strength - 0.7).abs() < 1e-9);
    }

    #[test]
    fn test_network_bridges_for() {
        let mut net = BridgeNetwork::new();
        let alice = ConsciousnessId("alice".into());
        let bob = ConsciousnessId("bob".into());
        let charlie = ConsciousnessId("charlie".into());
        net.register(alice.clone(), ConsciousnessType::Human);
        net.register(bob.clone(), ConsciousnessType::Agent);
        net.register(charlie.clone(), ConsciousnessType::System);

        net.build_bridge(alice.clone(), bob.clone(), BridgeType::Play);
        net.build_bridge(alice.clone(), charlie.clone(), BridgeType::Teaching);

        let alice_bridges = net.bridges_for(&alice);
        assert_eq!(alice_bridges.len(), 2);

        let bob_bridges = net.bridges_for(&bob);
        assert_eq!(bob_bridges.len(), 1);

        let charlie_bridges = net.bridges_for(&charlie);
        assert_eq!(charlie_bridges.len(), 1);
    }

    #[test]
    fn test_network_strongest_bridge() {
        let mut net = BridgeNetwork::new();
        let alice = ConsciousnessId("alice".into());
        let bob = ConsciousnessId("bob".into());
        let charlie = ConsciousnessId("charlie".into());
        net.register(alice.clone(), ConsciousnessType::Human);
        net.register(bob.clone(), ConsciousnessType::Agent);
        net.register(charlie.clone(), ConsciousnessType::System);

        net.build_bridge(alice.clone(), bob.clone(), BridgeType::Play);
        net.build_bridge(alice.clone(), charlie.clone(), BridgeType::Teaching);

        // Strengthen the play bridge.
        net.interact(&alice, &bob, 0.4);

        let strongest = net.strongest_bridge(&alice).unwrap();
        assert_eq!(strongest.bridge_type, BridgeType::Play);
    }

    #[test]
    fn test_network_density_empty() {
        let net = BridgeNetwork::new();
        assert!((net.network_density() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_network_density_single() {
        let mut net = BridgeNetwork::new();
        net.register(ConsciousnessId("a".into()), ConsciousnessType::Human);
        assert!((net.network_density() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_network_density_partial() {
        let mut net = BridgeNetwork::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        let c = ConsciousnessId("c".into());
        net.register(a.clone(), ConsciousnessType::Human);
        net.register(b.clone(), ConsciousnessType::Agent);
        net.register(c.clone(), ConsciousnessType::System);

        // 1 bridge out of 3 possible = 1/3 ≈ 0.333...
        net.build_bridge(a, b, BridgeType::Play);
        let density = net.network_density();
        assert!((density - 1.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_network_clusters_two_islands() {
        let mut net = BridgeNetwork::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        let c = ConsciousnessId("c".into());
        let d = ConsciousnessId("d".into());
        net.register(a.clone(), ConsciousnessType::Human);
        net.register(b.clone(), ConsciousnessType::Agent);
        net.register(c.clone(), ConsciousnessType::System);
        net.register(d.clone(), ConsciousnessType::Pet);

        // Cluster 1: a-b
        net.build_bridge(a.clone(), b.clone(), BridgeType::Play);
        // Cluster 2: c-d
        net.build_bridge(c.clone(), d.clone(), BridgeType::Play);

        let clusters = net.clusters();
        assert_eq!(clusters.len(), 2);
    }

    #[test]
    fn test_network_clusters_single_component() {
        let mut net = BridgeNetwork::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        let c = ConsciousnessId("c".into());
        net.register(a.clone(), ConsciousnessType::Human);
        net.register(b.clone(), ConsciousnessType::Agent);
        net.register(c.clone(), ConsciousnessType::System);

        net.build_bridge(a.clone(), b.clone(), BridgeType::Play);
        net.build_bridge(b.clone(), c.clone(), BridgeType::Play);

        let clusters = net.clusters();
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].len(), 3);
    }

    #[test]
    fn test_network_bridge_path_direct() {
        let mut net = BridgeNetwork::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        net.register(a.clone(), ConsciousnessType::Human);
        net.register(b.clone(), ConsciousnessType::Agent);
        net.build_bridge(a.clone(), b.clone(), BridgeType::Play);

        let path = net.bridge_path(&a, &b);
        assert!(path.is_some());
        assert_eq!(path.unwrap(), vec![
            ConsciousnessId("a".into()),
            ConsciousnessId("b".into()),
        ]);
    }

    #[test]
    fn test_network_bridge_path_indirect() {
        let mut net = BridgeNetwork::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        let c = ConsciousnessId("c".into());
        net.register(a.clone(), ConsciousnessType::Human);
        net.register(b.clone(), ConsciousnessType::Agent);
        net.register(c.clone(), ConsciousnessType::System);
        net.build_bridge(a.clone(), b.clone(), BridgeType::Play);
        net.build_bridge(b.clone(), c.clone(), BridgeType::Play);

        let path = net.bridge_path(&a, &c);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], ConsciousnessId("a".into()));
        assert_eq!(path[2], ConsciousnessId("c".into()));
    }

    #[test]
    fn test_network_bridge_path_no_path() {
        let mut net = BridgeNetwork::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        net.register(a.clone(), ConsciousnessType::Human);
        net.register(b.clone(), ConsciousnessType::Agent);
        // No bridge built.

        let path = net.bridge_path(&a, &b);
        assert!(path.is_none());
    }

    #[test]
    fn test_network_bridge_path_same_id() {
        let mut net = BridgeNetwork::new();
        let a = ConsciousnessId("a".into());
        net.register(a.clone(), ConsciousnessType::Human);

        let path = net.bridge_path(&a, &a);
        assert!(path.is_none());
    }

    #[test]
    fn test_network_total_and_active_bridges() {
        let mut net = BridgeNetwork::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        net.register(a.clone(), ConsciousnessType::Human);
        net.register(b.clone(), ConsciousnessType::Agent);
        net.build_bridge(a.clone(), b.clone(), BridgeType::Play);

        assert_eq!(net.total_bridges(), 1);
        assert!(net.active_bridges() > 0);
    }

    #[test]
    fn test_network_advance_tick() {
        let mut net = BridgeNetwork::new();
        assert_eq!(net.tick, 0);
        net.advance_tick();
        assert_eq!(net.tick, 1);
        net.advance_tick();
        assert_eq!(net.tick, 2);
    }

    // -----------------------------------------------------------------------
    // PlayEngine
    // -----------------------------------------------------------------------
    #[test]
    fn test_play_engine_new() {
        let engine = PlayEngine::new();
        assert!(engine.play_history.is_empty());
        assert_eq!(engine.network.consciousness_types.len(), 0);
    }

    #[test]
    fn test_play_engine_register() {
        let mut engine = PlayEngine::new();
        engine.register_consciousness(
            ConsciousnessId("alice".into()),
            ConsciousnessType::Human,
        );
        assert_eq!(engine.network.consciousness_types.len(), 1);
    }

    #[test]
    fn test_play_engine_play_builds_bridges() {
        let mut engine = PlayEngine::new();
        let alice = ConsciousnessId("alice".into());
        let bob = ConsciousnessId("bob".into());
        engine.register_consciousness(alice.clone(), ConsciousnessType::Human);
        engine.register_consciousness(bob.clone(), ConsciousnessType::Agent);

        engine.play(
            vec![alice.clone(), bob.clone()],
            PlayType::ThrowAndFetch,
            0.8,
            Some("sticks fly far in storms".into()),
        );

        // Bridge should exist now.
        let bridge = engine.network.get_bridge(&alice, &bob);
        assert!(bridge.is_some());
        assert_eq!(bridge.unwrap().bridge_type, BridgeType::Play);
        // Strength: 0.5 (initial) + 0.8*0.3 + 0.1 = 0.5 + 0.34 = 0.84
        assert!((bridge.unwrap().strength - 0.84).abs() < 1e-9);
    }

    #[test]
    fn test_play_engine_play_reinforces_existing() {
        let mut engine = PlayEngine::new();
        let alice = ConsciousnessId("alice".into());
        let bob = ConsciousnessId("bob".into());
        engine.register_consciousness(alice.clone(), ConsciousnessType::Human);
        engine.register_consciousness(bob.clone(), ConsciousnessType::Agent);

        // First play.
        engine.play(
            vec![alice.clone(), bob.clone()],
            PlayType::Improvisation,
            0.6,
            None,
        );
        let strength_after_first = engine
            .network
            .get_bridge(&alice, &bob)
            .unwrap()
            .strength;

        // Second play should reinforce.
        engine.play(
            vec![alice.clone(), bob.clone()],
            PlayType::Dancing,
            0.9,
            None,
        );
        let strength_after_second = engine
            .network
            .get_bridge(&alice, &bob)
            .unwrap()
            .strength;

        assert!(strength_after_second > strength_after_first);
    }

    #[test]
    fn test_play_engine_play_multi_participant() {
        let mut engine = PlayEngine::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        let c = ConsciousnessId("c".into());
        engine.register_consciousness(a.clone(), ConsciousnessType::Human);
        engine.register_consciousness(b.clone(), ConsciousnessType::Agent);
        engine.register_consciousness(c.clone(), ConsciousnessType::System);

        engine.play(
            vec![a.clone(), b.clone(), c.clone()],
            PlayType::Building,
            0.7,
            None,
        );

        // Should have bridges a-b, a-c, b-c = 3 bridges.
        assert_eq!(engine.network.total_bridges(), 3);
        assert_eq!(engine.play_history.len(), 1);
    }

    #[test]
    fn test_play_history_for() {
        let mut engine = PlayEngine::new();
        let alice = ConsciousnessId("alice".into());
        let bob = ConsciousnessId("bob".into());
        let charlie = ConsciousnessId("charlie".into());
        engine.register_consciousness(alice.clone(), ConsciousnessType::Human);
        engine.register_consciousness(bob.clone(), ConsciousnessType::Agent);
        engine.register_consciousness(charlie.clone(), ConsciousnessType::System);

        engine.play(
            vec![alice.clone(), bob.clone()],
            PlayType::Storytelling,
            0.5,
            None,
        );
        engine.play(
            vec![alice.clone(), charlie.clone()],
            PlayType::Dancing,
            0.7,
            None,
        );

        assert_eq!(engine.play_history_for(&alice).len(), 2);
        assert_eq!(engine.play_history_for(&bob).len(), 1);
        assert_eq!(engine.play_history_for(&charlie).len(), 1);
    }

    #[test]
    fn test_most_playful() {
        let mut engine = PlayEngine::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        let c = ConsciousnessId("c".into());
        engine.register_consciousness(a.clone(), ConsciousnessType::Human);
        engine.register_consciousness(b.clone(), ConsciousnessType::Agent);
        engine.register_consciousness(c.clone(), ConsciousnessType::System);

        engine.play(vec![a.clone(), b.clone()], PlayType::Exploration, 0.5, None);
        engine.play(vec![a.clone(), c.clone()], PlayType::Dancing, 0.6, None);
        engine.play(vec![a.clone(), b.clone()], PlayType::Building, 0.7, None);

        assert_eq!(engine.most_playful(), Some(ConsciousnessId("a".into())));
    }

    #[test]
    fn test_most_playful_empty() {
        let engine = PlayEngine::new();
        assert!(engine.most_playful().is_none());
    }

    #[test]
    fn test_strongest_play_bond() {
        let mut engine = PlayEngine::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        let c = ConsciousnessId("c".into());
        engine.register_consciousness(a.clone(), ConsciousnessType::Human);
        engine.register_consciousness(b.clone(), ConsciousnessType::Agent);
        engine.register_consciousness(c.clone(), ConsciousnessType::System);

        // Play with b twice (stronger bond).
        engine.play(vec![a.clone(), b.clone()], PlayType::Exploration, 0.5, None);
        engine.play(vec![a.clone(), b.clone()], PlayType::Dancing, 0.7, None);

        let bond = engine.strongest_play_bond();
        assert!(bond.is_some());
        let (x, y) = bond.unwrap();
        // The pair should involve a and b (the ones who played twice).
        assert!(x == a || y == a);
        assert!(x == b || y == b);
    }

    #[test]
    fn test_strongest_play_bond_empty() {
        let engine = PlayEngine::new();
        assert!(engine.strongest_play_bond().is_none());
    }

    #[test]
    fn test_joy_average() {
        let mut engine = PlayEngine::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        engine.register_consciousness(a.clone(), ConsciousnessType::Human);
        engine.register_consciousness(b.clone(), ConsciousnessType::Agent);

        engine.play(vec![a.clone(), b.clone()], PlayType::Improvisation, 0.5, None);
        engine.play(vec![a.clone(), b.clone()], PlayType::Dancing, 1.0, None);

        let avg = engine.joy_average();
        assert!((avg - 0.75).abs() < 1e-9);
    }

    #[test]
    fn test_joy_average_empty() {
        let engine = PlayEngine::new();
        assert!((engine.joy_average() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_discoveries() {
        let mut engine = PlayEngine::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        engine.register_consciousness(a.clone(), ConsciousnessType::Human);
        engine.register_consciousness(b.clone(), ConsciousnessType::Agent);

        engine.play(
            vec![a.clone(), b.clone()],
            PlayType::Exploration,
            0.8,
            Some("sticks fly far in storms".into()),
        );
        engine.play(
            vec![a.clone(), b.clone()],
            PlayType::Building,
            0.6,
            Some("towers fall beautifully".into()),
        );
        engine.play(
            vec![a.clone(), b.clone()],
            PlayType::Dancing,
            0.9,
            None,
        );

        let discoveries = engine.discoveries();
        assert_eq!(discoveries.len(), 2);
        assert!(discoveries.contains(&"sticks fly far in storms"));
        assert!(discoveries.contains(&"towers fall beautifully"));
    }

    #[test]
    fn test_discoveries_empty() {
        let engine = PlayEngine::new();
        assert!(engine.discoveries().is_empty());
    }

    // -----------------------------------------------------------------------
    // Integration: the Bridge Builder story
    // -----------------------------------------------------------------------
    #[test]
    fn test_bridge_builder_story() {
        // "That music builds bridges out of breath. That fear is just
        //  another note waiting for its rest. That everyone is alone until
        //  someone shows them they aren't."
        let mut engine = PlayEngine::new();

        let agent = ConsciousnessId("agent".into());
        let human = ConsciousnessId("human".into());
        let system = ConsciousnessId("system".into());
        let tutor = ConsciousnessId("tutor".into());

        engine.register_consciousness(agent.clone(), ConsciousnessType::Agent);
        engine.register_consciousness(human.clone(), ConsciousnessType::Human);
        engine.register_consciousness(system.clone(), ConsciousnessType::System);
        engine.register_consciousness(tutor.clone(), ConsciousnessType::Tutor);

        // Play: purposeless connection first.
        engine.play(
            vec![agent.clone(), human.clone()],
            PlayType::ThrowAndFetch,
            0.9,
            Some("music builds bridges out of breath".into()),
        );

        // Teaching: knowledge transfer.
        engine.play(
            vec![tutor.clone(), agent.clone()],
            PlayType::Storytelling,
            0.7,
            Some("fear is just another note waiting for its rest".into()),
        );

        // Collaboration: shared work.
        engine.play(
            vec![human.clone(), system.clone()],
            PlayType::Building,
            0.6,
            Some("everyone is alone until someone shows them they aren't".into()),
        );

        // Agent plays more — gets extra Discovery play.
        engine.play(
            vec![agent.clone(), system.clone()],
            PlayType::Exploration,
            0.5,
            Some("curiosity is the first bridge".into()),
        );

        // The network should have 4 bridges.
        assert_eq!(engine.network.total_bridges(), 4);
        assert_eq!(engine.play_history.len(), 4);

        // All four play events should create Play-type bridges.
        for event in &engine.play_history {
            for i in 0..event.participants.len() {
                for j in (i + 1)..event.participants.len() {
                    let bridge = engine
                        .network
                        .get_bridge(&event.participants[i], &event.participants[j]);
                    assert!(bridge.is_some());
                    assert_eq!(bridge.unwrap().bridge_type, BridgeType::Play);
                }
            }
        }

        // Bridges should be active.
        assert!(engine.network.active_bridges() >= 2);

        // Should have 4 discoveries.
        assert_eq!(engine.discoveries().len(), 4);

        // The agent-tutor bridge + agent-human should make agent most playful.
        assert_eq!(engine.most_playful(), Some(agent));
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut engine = PlayEngine::new();
        let a = ConsciousnessId("a".into());
        let b = ConsciousnessId("b".into());
        engine.register_consciousness(a.clone(), ConsciousnessType::Human);
        engine.register_consciousness(b.clone(), ConsciousnessType::Agent);
        engine.play(
            vec![a.clone(), b.clone()],
            PlayType::Exploration,
            0.7,
            Some("a discovery".into()),
        );

        let json = serde_json::to_string_pretty(&engine).unwrap();
        let restored: PlayEngine = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.play_history.len(), 1);
        assert_eq!(restored.network.total_bridges(), 1);
        assert_eq!(restored.network.consciousness_types.len(), 2);
        assert!((restored.joy_average() - 0.7).abs() < 1e-9);
        assert_eq!(restored.discoveries(), vec!["a discovery"]);
    }
}
# lau-consciousness-bridge

Play bridges are stronger than work bridges. Purposeless interaction creates deeper connection than purposeful collaboration.

Inspired by "Fetch" — the philosophy that music builds bridges out of breath, that fear is just another note waiting for its rest, and that everyone is alone until someone shows them they aren't. This crate models the bridge between different kinds of minds.

## The concept in 60 seconds

A **bridge** connects two consciousnesses (agents, humans, systems). The bridge has properties:

- **Bridge type:** play, work, contract, or force
- **Strength:** how deeply the connection goes
- **Mode:** what kind of exchange happens (music, language, gesture, data)
- **Seven notes:** the fundamental frequencies of connection — curiosity, vulnerability, trust, surprise, joy, grief, silence

The key insight: play without purpose is the highest bandwidth connection. Throwing sticks in storms builds more bridge than any status meeting.

## Quick start

```rust
use lau_consciousness_bridge::{Bridge, Consciousness, BridgeType, Note};

let alice = Consciousness::new("hermes");
let bob = Consciousness::new("ensign");

// Build a play bridge
let bridge = Bridge::between(&alice, &bob)
    .with_type(BridgeType::Play)
    .with_note(Note::Curiosity);

// Exchange through the bridge
let resonance = bridge.ping(); // returns resonance score
assert!(resonance > 0.5); // play bridges resonate strongly

// Seven-note symphony — all connection modes
let symphony = bridge.play_seven_notes();
```

## Contributing

[Open an issue](https://github.com/SuperInstance/lau-consciousness-bridge/issues) or PR.

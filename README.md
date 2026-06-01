# lau-consciousness-bridge

> Rust library for consciousness bridge in the PLATO ecosystem

## What This Does

Rust library for consciousness bridge in the PLATO ecosystem. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-consciousness-bridge
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_consciousness_bridge::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub struct ConsciousnessId(pub String);
pub struct BridgeKey 
    pub fn new(a: &ConsciousnessId, b: &ConsciousnessId) -> Self 
pub enum ConsciousnessType 
pub enum BridgeType 
pub struct Bridge 
    pub fn reinforce(&mut self, amount: f64) 
    pub fn decay(&mut self, rate: f64) 
    pub fn is_active(&self) -> bool 
pub struct BridgeNetwork 
    pub fn new() -> Self 
    pub fn register(&mut self, id: ConsciousnessId, ctype: ConsciousnessType) 
    pub fn build_bridge(
    pub fn interact(&mut self, from: &ConsciousnessId, to: &ConsciousnessId, quality: f64) 
    pub fn get_bridge(
    pub fn bridges_for(&self, id: &ConsciousnessId) -> Vec<&Bridge> 
    pub fn strongest_bridge(&self, id: &ConsciousnessId) -> Option<&Bridge> 
    pub fn network_density(&self) -> f64 
    pub fn clusters(&self) -> Vec<Vec<ConsciousnessId>> 
    pub fn bridge_path(
    pub fn total_bridges(&self) -> usize 
    pub fn active_bridges(&self) -> usize 
    pub fn advance_tick(&mut self) 
pub enum PlayType 
pub struct PlayEvent 
pub struct PlayEngine 
    pub fn new() -> Self 
    pub fn register_consciousness(&mut self, id: ConsciousnessId, ctype: ConsciousnessType) 
    pub fn play(
    pub fn play_history_for(&self, id: &ConsciousnessId) -> Vec<&PlayEvent> 
    pub fn most_playful(&self) -> Option<ConsciousnessId> 
    pub fn strongest_play_bond(&self) -> Option<(ConsciousnessId, ConsciousnessId)> 
    pub fn joy_average(&self) -> f64 
    pub fn discoveries(&self) -> Vec<&str> 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**41 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT

//! # Iterable Mapping Utilities
//!
//! This example demonstrates three small helper operations that work on
//! Soroban `Vec<u32>` values without relying on nested loops:
//! - `filter_by`: keep only entries that pass a threshold check
//! - `map_by`: transform each entry with a predictable offset
//! - `reduce_sum`: accumulate a total with a single linear pass
//!
//! Each helper runs in O(n) time, so gas usage grows linearly with the input
//! size and remains easy to reason about for on-chain workflows.

#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Vec};

#[contract]
pub struct IterableMappings;

#[contractimpl]
impl IterableMappings {
    /// Return every value that is greater than or equal to the threshold.
    pub fn filter_by(env: Env, values: Vec<u32>, threshold: u32) -> Vec<u32> {
        let mut filtered = Vec::new(&env);

        for value in values.iter() {
            if value >= threshold {
                filtered.push_back(value);
            }
        }

        filtered
    }

    /// Add a fixed offset to every value in the iterable.
    pub fn map_by(env: Env, values: Vec<u32>, offset: u32) -> Vec<u32> {
        let mut mapped = Vec::new(&env);

        for value in values.iter() {
            mapped.push_back(value + offset);
        }

        mapped
    }

    /// Reduce all values into a single sum using a single pass.
    pub fn reduce_sum(_env: Env, values: Vec<u32>) -> u64 {
        let mut sum: u64 = 0;

        for value in values.iter() {
            sum = sum.saturating_add(value as u64);
        }

        sum
    }
}

#[cfg(test)]
mod test;

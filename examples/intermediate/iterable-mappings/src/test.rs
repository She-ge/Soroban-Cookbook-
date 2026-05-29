use super::*;
use soroban_sdk::{vec, Env};

#[test]
fn test_filter_by_keeps_threshold_values() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IterableMappings);
    let client = IterableMappingsClient::new(&env, &contract_id);

    let values = vec![&env, 1u32, 4u32, 7u32, 2u32];

    let filtered = client.filter_by(&values, &4);

    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered.get(0).unwrap(), 4u32);
    assert_eq!(filtered.get(1).unwrap(), 7u32);
}

#[test]
fn test_map_by_offsets_each_value() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IterableMappings);
    let client = IterableMappingsClient::new(&env, &contract_id);

    let values = vec![&env, 1u32, 2u32, 3u32];

    let mapped = client.map_by(&values, &5);

    assert_eq!(mapped.len(), 3);
    assert_eq!(mapped.get(0).unwrap(), 6u32);
    assert_eq!(mapped.get(1).unwrap(), 7u32);
    assert_eq!(mapped.get(2).unwrap(), 8u32);
}

#[test]
fn test_reduce_sum_accumulates_linear_pass() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IterableMappings);
    let client = IterableMappingsClient::new(&env, &contract_id);

    let values = vec![&env, 10u32, 20u32, 30u32];

    let total = client.reduce_sum(&values);

    assert_eq!(total, 60u64);
}

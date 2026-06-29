extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

fn setup() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let src1 = Address::generate(&env);
    let src2 = Address::generate(&env);
    let src3 = Address::generate(&env);

    client.initialize(&admin);

    (env, admin, src1, src2, src3)
}

#[test]
fn test_initialize() {
    let (env, admin, _, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    assert!(!client.is_paused());
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_init_twice() {
    let (env, admin, _, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    client.initialize(&admin);
}

#[test]
fn test_add_source() {
    let (env, admin, src1, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    client.add_source(&admin, &src1);
    let sources = client.get_sources();
    assert_eq!(sources.len(), 1);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_add_source_unauthorized() {
    let (env, admin, src1, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    let fake_admin = Address::generate(&env);
    client.add_source(&fake_admin, &src1);
}

#[test]
#[should_panic(expected = "Exists")]
fn test_add_duplicate_source() {
    let (env, admin, src1, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    client.add_source(&admin, &src1);
    client.add_source(&admin, &src1);
}

#[test]
fn test_remove_source() {
    let (env, admin, src1, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    client.add_source(&admin, &src1);
    client.remove_source(&admin, &src1);
    let sources = client.get_sources();
    assert_eq!(sources.len(), 0);
}

#[test]
fn test_submit_data() {
    let (env, admin, src1, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    client.add_source(&admin, &src1);
    client.submit_data(&src1, &100);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_submit_unauthorized() {
    let (env, admin, src1, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    let fake = Address::generate(&env);
    client.submit_data(&fake, &100);
}

#[test]
fn test_pause_resume() {
    let (env, admin, _, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    assert!(!client.is_paused());
    client.pause(&admin);
    assert!(client.is_paused());
    client.resume(&admin);
    assert!(!client.is_paused());
}

#[test]
#[should_panic(expected = "Paused")]
fn test_submit_when_paused() {
    let (env, admin, src1, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    client.add_source(&admin, &src1);
    client.pause(&admin);
    client.submit_data(&src1, &100);
}

#[test]
#[should_panic(expected = "Not enough sources")]
fn test_aggregate_no_sources() {
    let (env, _, _, _, _) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &Address::generate(&env));
    let admin = Address::generate(&env);
    client.initialize(&admin);
    client.aggregate_data();
}

#[test]
fn test_aggregate_valid_data() {
    let (env, admin, src1, src2, src3) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    client.add_source(&admin, &src1);
    client.add_source(&admin, &src2);
    client.add_source(&admin, &src3);

    client.submit_data(&src1, &100);
    client.submit_data(&src2, &102);
    client.submit_data(&src3, &101);

    let result = client.aggregate_data();
    assert!(result.median_value > 0);
    assert_eq!(result.point_count, 3);
}

#[test]
fn test_aggregate_with_outlier() {
    let (env, admin, src1, src2, src3) = setup();
    let contract_id = env.register_contract(None, DataAggregationOracleContract);
    let client = DataAggregationOracleContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    client.add_source(&admin, &src1);
    client.add_source(&admin, &src2);
    client.add_source(&admin, &src3);

    client.submit_data(&src1, &100);
    client.submit_data(&src2, &102);
    client.submit_data(&src3, &5000); // Outlier

    let result = client.aggregate_data();
    assert!(result.outliers_removed > 0);
}

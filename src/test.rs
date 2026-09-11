#![cfg(test)]
use super::*;
use soroban_sdk::testutils::Address as _;

#[test]
fn test_initialize_and_threshold() {
    let env = Env::default();
    let contract_id = env.register(SoroSentinel, ());
    let client = SoroSentinelClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin, &75);
    assert_eq!(client.get_threshold(), 75);
}

// TODO(#issue): no test yet for authorize_agent + flag_anomaly happy path,
// no test for unauthorized flag_anomaly rejection, no test for
// double-initialize panic. See open testing issue.

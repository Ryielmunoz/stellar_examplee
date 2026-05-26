#[cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, vec, Address, Env};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient as TokenAdminClient;

fn setup_test_environment<'a>(env: &Env) -> (Address, Address, Address, Address, Address, TokenClient) {
    let employer = Address::generate(env);
    let worker = Address::generate(env);
    let family_rent = Address::generate(env);
    let family_tuition = Address::generate(env);
    
    let token_admin_address = Address::generate(env);
    let token_address = env.register_stellar_asset_contract(token_admin_address.clone());
    let token_client = TokenClient::new(env, &token_address);
    let token_admin = TokenAdminClient::new(env, &token_address);
    
    // Seed employer with salary payroll tokens
    token_admin.mint(&employer, &50_000);

    (employer, worker, family_rent, family_tuition, token_address, token_client)
}

#[test]
fn test_happy_path_payroll_split() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RemitSplitContract);
    let client = RemitSplitContractClient::new(&env, &contract_id);
    let (employer, worker, rent_addr, school_addr, token, token_client) = setup_test_environment(&env);

    // Build allocation profile: 70% goes to Rent, 30% goes to School Tuition
    let targets = vec![
        &env,
        AllocationTarget { recipient: rent_addr.clone(), basis_points: 7000 },
        AllocationTarget { recipient: school_addr.clone(), basis_points: 3000 },
    ];

    client.set_profile(&worker, &targets);
    
    // Run the payroll run of 10,000 tokens
    client.execute_payroll(&employer, &worker, &token, &10000);

    // Check outputs
    assert_eq!(token_client.balance(&rent_addr), 7000);
    assert_eq!(token_client.balance(&school_addr), 3000);
    assert_eq!(token_client.balance(&employer), 40000); // 50k initial - 10k paid
}

#[test]
#[should_panic(expected = "Total basis points must equal exactly 10000 (100%)")]
fn test_edge_case_invalid_bps_sum() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RemitSplitContract);
    let client = RemitSplitContractClient::new(&env, &contract_id);
    let (_, worker, rent_addr, _, _, _) = setup_test_environment(&env);

    // Bad breakdown setup: Only adds up to 90% (9000 BPS)
    let bad_targets = vec![
        &env,
        AllocationTarget { recipient: rent_addr, basis_points: 9000 },
    ];

    client.set_profile(&worker, &bad_targets);
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RemitSplitContract);
    let client = RemitSplitContractClient::new(&env, &contract_id);
    let (_, worker, rent_addr, school_addr, _, _) = setup_test_environment(&env);

    let targets = vec![
        &env,
        AllocationTarget { recipient: rent_addr.clone(), basis_points: 5000 },
        AllocationTarget { recipient: school_addr.clone(), basis_points: 5000 },
    ];

    client.set_profile(&worker, &targets);
    let stored_profile = client.get_profile(&worker);

    assert_eq!(stored_profile.len(), 2);
    assert_eq!(stored_profile.get(0).unwrap().basis_points, 5000);
}

#[test]
#[should_panic(expected = "Worker has not configured their remittance profile")]
fn test_edge_case_unconfigured_worker() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RemitSplitContract);
    let client = RemitSplitContractClient::new(&env, &contract_id);
    let (employer, worker, _, _, token, _) = setup_test_environment(&env);

    // Trigger processing without setting registration weights first
    client.execute_payroll(&employer, &worker, &token, &5000);
}

#[test]
#[should_panic(expected = "Salary must be greater than zero")]
fn test_edge_case_negative_salary() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RemitSplitContract);
    let client = RemitSplitContractClient::new(&env, &contract_id);
    let (employer, worker, rent_addr, _, token, _) = setup_test_environment(&env);

    let targets = vec![&env, AllocationTarget { recipient: rent_addr, basis_points: 10000 }];
    client.set_profile(&worker, &targets);

    client.execute_payroll(&employer, &worker, &token, &-100);
}
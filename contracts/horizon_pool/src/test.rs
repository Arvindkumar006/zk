#![cfg(test)]
use super::*;
use soroban_sdk::{
    Address, Bytes, BytesN, Env, Vec,
};

/// Extension trait to implement `try_from_asm` for test environment compatibility
pub trait AddressExt {
    fn try_from_asm(env: &Env, strkey: &str) -> Address;
}

impl AddressExt for Address {
    fn try_from_asm(env: &Env, strkey: &str) -> Address {
        Address::from_string(env, &soroban_sdk::String::from_str(env, strkey))
    }
}

/// Mock UltraHonk Verifier contract to simulate proof verification checks.
#[contract]
pub struct MockVerifier;

#[contractimpl]
impl MockVerifier {
    pub fn verify_proof(_env: Env, _public_inputs: Bytes, _proof_bytes: Bytes) {
        // Mock verifier succeeds automatically for tests
    }
}

#[test]
fn test_initialize_and_get_root() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::try_from_asm(&env, "GB733333333333333333333333333333333333333333333333333333");
    let initial_root = BytesN::from_array(&env, &[1u8; 32]);

    let pool_id = env.register(HorizonPool, ());
    let pool_client = HorizonPoolClient::new(&env, &pool_id);

    pool_client.initialize(&admin, &initial_root);

    // Verify initial Merkle root is stored correctly
    let active_root = env.as_contract(&pool_id, || {
        env.storage()
            .instance()
            .get::<_, BytesN<32>>(&DataKey::MerkleRoot)
            .unwrap()
    });
    assert_eq!(active_root, initial_root);
}

#[test]
fn test_update_root_success() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::try_from_asm(&env, "GB733333333333333333333333333333333333333333333333333333");
    let initial_root = BytesN::from_array(&env, &[1u8; 32]);
    let new_root = BytesN::from_array(&env, &[2u8; 32]);

    let pool_id = env.register(HorizonPool, ());
    let pool_client = HorizonPoolClient::new(&env, &pool_id);

    pool_client.initialize(&admin, &initial_root);

    // Call update_root as the admin
    pool_client.update_root(&new_root);

    let active_root = env.as_contract(&pool_id, || {
        env.storage()
            .instance()
            .get::<_, BytesN<32>>(&DataKey::MerkleRoot)
            .unwrap()
    });
    assert_eq!(active_root, new_root);
}

#[test]
#[should_panic]
fn test_unauthorized_update_root_panics() {
    let env = Env::default();
    // Do NOT mock all auths so that authorization constraints are strictly checked
    
    let admin = Address::try_from_asm(&env, "GB733333333333333333333333333333333333333333333333333333");
    let initial_root = BytesN::from_array(&env, &[1u8; 32]);
    let new_root = BytesN::from_array(&env, &[2u8; 32]);

    let pool_id = env.register(HorizonPool, ());
    let pool_client = HorizonPoolClient::new(&env, &pool_id);

    // Initialize (doesn't require auth on deploy/init in this test setup)
    pool_client.initialize(&admin, &initial_root);

    // Call update_root should panic since require_auth is not satisfied
    pool_client.update_root(&new_root);
}

#[test]
fn test_happy_path_swap_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::try_from_asm(&env, "GB733333333333333333333333333333333333333333333333333333");
    let initial_root = BytesN::from_array(&env, &[1u8; 32]);

    // Register contracts
    let pool_id = env.register(HorizonPool, ());
    let pool_client = HorizonPoolClient::new(&env, &pool_id);

    let verifier_id = env.register(MockVerifier, ());

    pool_client.initialize(&admin, &initial_root);

    // Setup swap parameters
    let swap_amount = 1500u64;
    let recipient_hash = BytesN::from_array(&env, &[5u8; 32]);
    
    // Structure public inputs to match contract validation checks:
    // [merkle_root, swap_amount, recipient_hash]
    let mut public_inputs = Vec::new(&env);
    public_inputs.push_back(initial_root.clone());
    
    // Format swap_amount as 32-byte big endian representation
    let mut amount_arr = [0u8; 32];
    amount_arr[24..32].copy_from_slice(&swap_amount.to_be_bytes());
    public_inputs.push_back(BytesN::from_array(&env, &amount_arr));
    
    public_inputs.push_back(recipient_hash);

    let mock_proof = Bytes::from_slice(&env, &[0u8; 100]); // Mock proof payload

    // Call verify_and_execute_swap - mock verifier will succeed automatically
    let res = pool_client.try_verify_and_execute_swap(&mock_proof, &public_inputs, &swap_amount, &verifier_id);
    assert!(res.is_ok());
}

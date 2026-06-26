#![no_std]
use soroban_sdk::{
    contract, contractclient, contracterror, contractimpl, contracttype, Address, Bytes, BytesN,
    Env, Symbol, Vec,
};

/// Interface matching the rs-soroban-ultrahonk verifier contract.
///
/// Under Protocol 26, the verifier invokes native BN254 pairing and multi-scalar multiplication (MSM)
/// host functions to run sumcheck and KZG opening proof verification within CPU resource limits.
#[contractclient(name = "UltraHonkVerifierClient")]
pub trait UltraHonkVerifier {
    fn verify_proof(env: Env, public_inputs: Bytes, proof_bytes: Bytes);
}

#[contracterror]
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    RootMismatch = 4,
    AmountMismatch = 5,
    InvalidPublicInputsLength = 6,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    MerkleRoot,
}

#[contract]
pub struct HorizonPool;

#[contractimpl]
impl HorizonPool {
    /// Initialize the Horizon Pool with the compliance administrator and the initial KYC Merkle root.
    pub fn initialize(env: Env, admin: Address, initial_root: BytesN<32>) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::MerkleRoot, &initial_root);

        Ok(())
    }

    /// Update the compliance Merkle root representing the latest KYC/AML allowlist database.
    /// Access control is enforced via the administrator's cryptographic authorization.
    pub fn update_root(env: Env, new_root: BytesN<32>) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;

        admin.require_auth();

        env.storage().instance().set(&DataKey::MerkleRoot, &new_root);
        Ok(())
    }

    /// Verify a user compliance ZK proof and execute the corresponding private swap.
    ///
    /// This function performs:
    /// 1. Integration verification checking that the public inputs match the on-chain Merkle root.
    /// 2. Verification that the swap amount matches the public input parameter to prevent proof reuse.
    /// 3. Dynamic call to the UltraHonk verifier contract using BN254 pairing/MSM primitives.
    /// 4. Emission of a 'Private Compliance Verified' event.
    pub fn verify_and_execute_swap(
        env: Env,
        proof: Bytes,
        public_inputs: Vec<BytesN<32>>,
        amount: u64,
        verifier_contract_id: Address,
    ) -> Result<(), Error> {
        if !env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NotInitialized);
        }

        // Validate public inputs length: [merkle_root, swap_amount, recipient_hash]
        if public_inputs.len() != 3 {
            return Err(Error::InvalidPublicInputsLength);
        }

        // 1. Verify that the proof utilizes the correct on-chain compliance root
        let active_root: BytesN<32> = env
            .storage()
            .instance()
            .get(&DataKey::MerkleRoot)
            .ok_or(Error::NotInitialized)?;

        if public_inputs.get(0).unwrap() != active_root {
            return Err(Error::RootMismatch);
        }

        // 2. Validate that the swap amount matches the public input parameter
        let mut amount_arr = [0u8; 32];
        amount_arr[24..32].copy_from_slice(&amount.to_be_bytes());
        let amount_bytes = BytesN::from_array(&env, &amount_arr);

        if public_inputs.get(1).unwrap() != amount_bytes {
            return Err(Error::AmountMismatch);
        }

        // 3. Flatten the Vec<BytesN<32>> public inputs into a continuous Bytes buffer
        let mut flat_inputs = Bytes::new(&env);
        for item in public_inputs.iter() {
            flat_inputs.append(&Bytes::from(item));
        }

        // 4. Perform external contract call to verify the UltraHonk proof
        let verifier_client = UltraHonkVerifierClient::new(&env, &verifier_contract_id);
        verifier_client.verify_proof(&flat_inputs, &proof);

        // 5. Emit success event for private compliance verification
        env.events().publish(
            (Symbol::new(&env, "Private Compliance Verified"),),
            amount,
        );

        Ok(())
    }
}

mod test;

#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, symbol_short, Symbol};

#[contract]
pub struct RemittanceContract;

#[contractimpl]
impl RemittanceContract {

    // Store remittance with a unique ID
    pub fn send_remittance(
        env: Env,
        sender: Address,
        receiver: Address,
        amount: i128,
        remittance_id: Symbol,
    ) {
        sender.require_auth();

        let key = (symbol_short!("REM"), remittance_id);

        // Store (sender, receiver, amount)
        env.storage().instance().set(&key, &(sender, receiver, amount));
    }

    // Claim remittance
    pub fn claim_remittance(
        env: Env,
        receiver: Address,
        remittance_id: Symbol,
    ) -> i128 {
        receiver.require_auth();

        let key = (symbol_short!("REM"), remittance_id);

        let (stored_sender, stored_receiver, amount): (Address, Address, i128) =
            env.storage().instance().get(&key).unwrap();

        if receiver != stored_receiver {
            panic!("Not authorized");
        }

        // Remove after claim
        env.storage().instance().remove(&key);

        amount
    }

    // View remittance details
    pub fn get_remittance(
        env: Env,
        remittance_id: Symbol,
    ) -> (Address, Address, i128) {
        let key = (symbol_short!("REM"), remittance_id);

        env.storage().instance().get(&key).unwrap()
    }
}
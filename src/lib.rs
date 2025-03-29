// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};
use alloy_primitives::{Address, Uint};
use stylus_sdk::{block, console};

// Define some persistent storage using the Solidity ABI.
// `vendingmachine` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct VendingMachine {
        mapping(address => uint256) coffee_balances;
        mapping(address => uint256) coffee_distribution_times;
    }
}

/// Declare that `VendingMachine` is a contract with the following external methods.
#[public]
impl VendingMachine {
   pub fn give_coffee_to(&mut self, user_address: Address) -> bool {
    let last_dist = self.coffee_distribution_times.get(user_address);

    let five_seconds_from_last_distribution = last_dist + U256::from(5);

    let current_time = block::timestamp();

    let user_can_recieve_coffee = five_seconds_from_last_distribution <= Uint::<256, 4>::from(current_time);

    if user_can_recieve_coffee {
        let mut balance_accessor = self.coffee_balances.setter(user_address);

        let balance = balance_accessor.get() + U256::from(1);

        balance_accessor.set(balance);

        let mut time_accessor = self.coffee_distribution_times.setter(user_address);

        let new_distribution_time = block::timestamp();

        time_accessor.set(Uint::<256, 4>::from(new_distribution_time));

        return true;
    } else{
        console!("HTTP 429: Too many cafe (you must wait at least 5 seconds0");

        return false;
    }
   }

   pub fn get_coffee_balance_for(&self, user_address: Address) -> Uint<256, 4>{
    return self.coffee_balances.get(user_address)
   }
}
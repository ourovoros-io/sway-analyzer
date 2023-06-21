contract;

use std::call_frames::msg_asset_id;
use std::context::msg_amount;
use std::storage::storage_vec::*;

abi TestContract {
    #[storage(write)]
    fn initialize_counter(value: u64) -> u64;

    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64;

    #[storage(read, write), payable]
    fn receive_funds(accounts: Vec<Address>);
}

struct Counter {
    value: u64,
}

storage {
    counter: Counter = Counter {
        value: 0,
    },

    balances: StorageMap<Address, StorageMap<ContractId, u64>> = StorageMap {},
}

impl TestContract for Contract {
    #[storage(write)]
    fn initialize_counter(value: u64) -> u64 {
        storage.counter.value.write(value);
        value
    }

    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64 {
        let mut counter = storage.counter.read();
        counter.value += amount;
        {
            storage.counter.write(counter);
        }
        counter.value += amount * 2;
        counter.value
    }

    #[storage(read, write), payable]
    fn receive_funds(accounts: Vec<Address>) {
        let account_count = accounts.len();
        let mut i = 0;

        while i < account_count {
            let balances = storage.balances.get(accounts.get(i).unwrap());
            let mut balance = balances.get(msg_asset_id()).read();
            balance += msg_amount();
            balances.insert(msg_asset_id(), balance);

            i += 1;
        }
    }
}

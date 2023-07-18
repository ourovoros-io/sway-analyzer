contract;

use std::call_frames::msg_asset_id;
use std::context::msg_amount;
use std::logging::log;
use std::storage::storage_vec::*;
use std::constants::{BASE_ASSET_ID, ZERO_B256};
use std::token::transfer;

abi TestContract {
    #[storage(write)]
    fn initialize_counter(value: u64) -> u64;

    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64;

    #[storage(read, write), payable]
    fn receive_funds(accounts: Vec<Address>);

    fn native_transfer(to: Identity, amount: u64);
}

enum Error {
    IncorrectAssetId: ContractId,
    NotEnoughAssets: u64,
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

pub fn asm_adder(a: u64, b: u64, c: u64) -> (u64, u64) {
    let empty_tuple = (0u64, 0u64);
    asm(output: empty_tuple, r1: a, r2: b, r3: c, r4, r5) {
        add  r4 r1 r2; // add a & b and put the result in r4
        add  r5 r2 r3; // add b & c and put the result in r5
        sw   output r4 i0; // store the word in r4 in output + 0 words
        sw   output r5 i1; // store the word in r5 in output + 1 word
        output: (u64, u64) // return both values
    }
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
        let mut amount = amount;
        counter.value += amount;
        {
            // storage.counter.write(counter);
            // log(counter);
        }
        amount *= 2;
        0
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

    fn native_transfer(to: Identity, amount: u64) {
        let asset_id = msg_asset_id();
        let asset_amount = msg_amount();
        require(asset_id == BASE_ASSET_ID, Error::IncorrectAssetId(asset_id));
        require(asset_amount >= amount, Error::NotEnoughAssets(asset_amount));

        match to {
            Identity::Address(x) => require(x != Address::from(ZERO_B256), "Zero address"),
            Identity::ContractId(x) => require(x != ContractId::from(ZERO_B256), "Zero contract id"),
        }

        if let Identity::Address(x) = to {
            require(x != Address::from(ZERO_B256), "Zero address");
        } else if let Identity::ContractId(x) = to {
            require(x != ContractId::from(ZERO_B256), "Zero contract id");
        }

        require(
            match to {
                Identity::Address(x) => x != Address::from(ZERO_B256),
                Identity::ContractId(x) => x != ContractId::from(ZERO_B256),
            },
            "Zero identity"
        );

        require(
            if let Identity::Address(x) = to {
                x != Address::from(ZERO_B256)
            } else if let Identity::ContractId(x) = to {
                x != ContractId::from(ZERO_B256)
            } else {
                true
            },
            "Zero identity"
        );

        //
        // TODO: show more cases of zero value identity checks
        //

        transfer(amount, asset_id, to);
    }
}

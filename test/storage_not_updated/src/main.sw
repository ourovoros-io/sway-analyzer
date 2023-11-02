contract;

use std::auth::msg_sender;
use std::hash::Hash;
use std::storage::storage_vec::*;

abi TestStorageNotUpdated {
    #[storage(read, write)] fn test_storage_u64_updated(amount: u64);
    #[storage(read, write)] fn test_storage_u64_not_updated(amount: u64);

    #[storage(read, write)] fn test_storage_struct_updated(amount: u64);
    #[storage(read, write)] fn test_storage_struct_not_updated(amount: u64);

    #[storage(read, write)] fn test_storage_vec_u64_updated(amount: u64);
    #[storage(read, write)] fn test_storage_vec_u64_not_updated(amount: u64);

    #[storage(read, write)] fn test_storage_vec_struct_updated(amount: u64);
    #[storage(read, write)] fn test_storage_vec_struct_not_updated(amount: u64);

    #[storage(read, write)] fn test_storage_map_u64_updated(amount: u64);
    #[storage(read, write)] fn test_storage_map_u64_not_updated(amount: u64);

    #[storage(read, write)] fn test_storage_map_struct_updated(amount: u64);
    #[storage(read, write)] fn test_storage_map_struct_not_updated(amount: u64);
}

struct Counter {
    value: u64,
}

storage {
    value: u64 = 0,

    counter: Counter = Counter {
        value: 0,
    },

    values_vec: StorageVec<u64> = StorageVec {},

    counters_vec: StorageVec<Counter> = StorageVec {},

    values_map: StorageMap<Identity, u64> = StorageMap {},

    counters_map: StorageMap<Identity, Counter> = StorageMap {},
}

impl TestStorageNotUpdated for Contract {
    #[storage(read, write)]
    fn test_storage_u64_updated(amount: u64) {
        // Report entry should not be created
        let mut value = storage.value.read();
        value += amount;
        storage.value.write(value);
    }

    #[storage(read, write)]
    fn test_storage_u64_not_updated(amount: u64) {
        // Report entry should be created:
        // L60: The `Contract::test_storage_u64_not_updated` function has storage bound to local variable `value` which is not written back to `storage.value`.
        let mut value = storage.value.read();
        value += amount;
    }

    #[storage(read, write)]
    fn test_storage_struct_updated(amount: u64) {
        // Report entry should not be created
        let mut counter = storage.counter.read();
        counter.value += amount;
        storage.counter.write(counter);
    }

    #[storage(read, write)]
    fn test_storage_struct_not_updated(amount: u64) {
        // Report entry should be created:
        // L76: The `Contract::test_storage_struct_not_updated` function has storage bound to local variable `counter` which is not written back to `storage.counter`.
        let mut counter = storage.counter.read();
        counter.value += amount;
    }
    
    #[storage(read, write)]
    fn test_storage_vec_u64_updated(amount: u64) {
        // Report entry should not be created
        let mut value = storage.values_vec.get(0).unwrap().read();
        value += amount;
        storage.values_vec.get(0).unwrap().write(value);
    }
    
    #[storage(read, write)]
    fn test_storage_vec_u64_not_updated(amount: u64) {
        // Report entry should be created:
        // L92: The `Contract::test_storage_vec_u64_not_updated` function has storage bound to local variable `value` which is not written back to `storage.values_vec`.
        let mut value = storage.values_vec.get(0).unwrap().read();
        value += amount;
    }
    
    #[storage(read, write)]
    fn test_storage_vec_struct_updated(amount: u64) {
        // Report entry should not be created
        let mut counter = storage.counters_vec.get(0).unwrap().read();
        counter.value += amount;
        storage.counters_vec.get(0).unwrap().write(counter);
    }
    
    #[storage(read, write)]
    fn test_storage_vec_struct_not_updated(amount: u64) {
        // Report entry should be created:
        // L108: The `Contract::test_storage_vec_struct_not_updated` function has storage bound to local variable `counter` which is not written back to `storage.counters_vec`.
        let mut counter = storage.counters_vec.get(0).unwrap().read();
        counter.value += amount;
    }
    
    #[storage(read, write)]
    fn test_storage_map_u64_updated(amount: u64) {
        let sender = msg_sender().unwrap();
        // Report entry should not be created
        let mut value = storage.values_map.get(sender).read();
        value += amount;
        storage.values_map.get(sender).write(value);
    }
    
    #[storage(read, write)]
    fn test_storage_map_u64_not_updated(amount: u64) {
        let sender = msg_sender().unwrap();
        // Report entry should be created:
        // L126: The `Contract::test_storage_map_u64_not_updated` function has storage bound to local variable `value` which is not written back to `storage.values_map`.
        let mut value = storage.values_map.get(sender).read();
        value += amount;
    }
    
    #[storage(read, write)]
    fn test_storage_map_struct_updated(amount: u64) {
        let sender = msg_sender().unwrap();
        // Report entry should not be created
        let mut counter = storage.counters_map.get(sender).read();
        counter.value += amount;
        storage.counters_map.get(sender).write(counter);
    }
    
    #[storage(read, write)]
    fn test_storage_map_struct_not_updated(amount: u64) {
        let sender = msg_sender().unwrap();
        // Report entry should be created:
        // L144: The `Contract::test_storage_map_struct_not_updated` function has storage bound to local variable `counter` which is not written back to `storage.counters_map`.
        let mut counter = storage.counters_map.get(sender).read();
        counter.value += amount;
    }
}

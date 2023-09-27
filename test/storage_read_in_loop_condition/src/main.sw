contract;

use std::storage::storage_vec::*;

abi TestStorageReadInLoopCondition {
    #[storage(read)]
    fn test_storage_read_in_loop_condition_1();
    #[storage(read)]
    fn test_storage_read_in_loop_condition_2();
}

storage {
    values: StorageVec<u64> = StorageVec {},
}

impl TestStorageReadInLoopCondition for Contract {
    #[storage(read)]
    fn test_storage_read_in_loop_condition_1() {
        let mut i = 0;

        // Report entry should be created:
        // L23: The The `Contract::test_storage_read_in_loop_condition` function contains a loop with a condition that depends on a storage read: `storage.values.len()`. Consider storing the expression in a local variable in order to reduce gas costs.
        while i < storage.values.len() {
            log(storage.values.get(i).unwrap());
            i += 1;
        }
    }

    #[storage(read)]
    fn test_storage_read_in_loop_condition_2() {
        let mut j = 0;
        let mut i = 0;

        while j < 10 {
            // Report entry should be created:
            // L37: The The `Contract::test_storage_read_in_nested_loop_condition` function contains a loop with a condition that depends on a storage read: `storage.values.len()`. Consider storing the expression in a local variable in order to reduce gas costs.
            while i < storage.values.len() {
                log(storage.values.get(i).unwrap());
                i += 1;
            }
        }
    }
}

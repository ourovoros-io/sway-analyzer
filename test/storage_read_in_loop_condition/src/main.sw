contract;

use std::storage::storage_vec::*;

abi TestStorageReadInLoopCondition {
    #[storage(read)] fn test_storage_read_in_loop_condition();
}

storage {
    values: StorageVec<u64> = StorageVec {},
}

impl TestStorageReadInLoopCondition for Contract {
    #[storage(read)]
    fn test_storage_read_in_loop_condition() {
        let mut i = 0;

        while i < storage.values.len() {
            log(storage.values.get(i).unwrap());
            i += 1;
        }
    }
}

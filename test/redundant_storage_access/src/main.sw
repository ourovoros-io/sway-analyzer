contract;

abi TestRedundantStorageAccess {
    #[storage(read)] fn test_storage_read_not_in_loop_condition();
    #[storage(read)] fn test_storage_read_in_loop_condition();
    
    #[storage(read)] fn test_non_redundant_storage_read();
    
    #[storage(read)] fn test_redundant_storage_read_1();
    #[storage(read)] fn test_redundant_storage_read_2();

    #[storage(write)] fn test_non_redundant_storage_write();
    
    #[storage(write)] fn test_redundant_storage_write_1();
    #[storage(write)] fn test_redundant_storage_write_2();
}

storage {
    counter: u64 = 0,
}

impl TestRedundantStorageAccess for Contract {
    // Report entry should not be created
    #[storage(read)]
    fn test_storage_read_not_in_loop_condition() {
        let mut x = 0;
        let counter = storage.counter.read();
        while x < counter {
            x += 1;
        }
    }

    // Report entry should be created:
    // L38: The `Contract::test_storage_read_in_loop_condition` function contains a loop condition with redundant storage access: `storage.counter.read()`. Consider storing the value in a local variable in order to lower gas costs.
    #[storage(read)]
    fn test_storage_read_in_loop_condition() {
        let mut x = 0;
        while x < storage.counter.read() {
            x += 1;
        }
    }
    
    // Report entry should not be created
    #[storage(read)]
    fn test_non_redundant_storage_read() {
        let _ = storage.counter.read();
    }

    // Report entry should be created:
    // L54: The `Contract::test_redundant_storage_read_1` function contains a redundant storage access: `storage.counter.read()`. Consider storing the value in a local variable in order to lower gas costs.
    #[storage(read)]
    fn test_redundant_storage_read_1() {
        let _ = storage.counter.read();
        let _ = storage.counter.read();
    }

    // Report entry should be created:
    // L63: The `Contract::test_redundant_storage_read_2` function contains a redundant storage access: `storage.counter.read()`. Consider storing the value in a local variable in order to lower gas costs.
    #[storage(read)]
    fn test_redundant_storage_read_2() {
        let _ = storage.counter.read();
        {
            let _ = storage.counter.read();
        }
    }

    // Report entry should not be created
    #[storage(write)]
    fn test_non_redundant_storage_write() {
        storage.counter.write(0);
    }

    // Report entry should be created:
    // L78: The `Contract::test_redundant_storage_write_1` function contains a redundant storage update: `storage.counter.write(0)`. Consider limiting to a single storage write in order to lower gas costs.
    #[storage(write)]
    fn test_redundant_storage_write_1() {
        storage.counter.write(0);
        storage.counter.write(0);
    }

    // Report entry should be created:
    // L87: The `Contract::test_redundant_storage_write_2` function contains a redundant storage update: `storage.counter.write(0)`. Consider limiting to a single storage write in order to lower gas costs.
    #[storage(write)]
    fn test_redundant_storage_write_2() {
        storage.counter.write(0);
        {
            storage.counter.write(0);
        }
    }
}

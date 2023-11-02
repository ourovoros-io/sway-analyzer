contract;

abi TestStorageNotUpdated {
    #[storage(read, write)]
    fn test_storage_updated(amount: u64);

    #[storage(read, write)]
    fn test_storage_not_updated(amount: u64);
}

struct Counter {
    value: u64,
}

storage {
    counter: Counter = Counter {
        value: 0,
    },
}

impl TestStorageNotUpdated for Contract {
    #[storage(read, write)]
    fn test_storage_updated(amount: u64) {
        // Report entry should not be created
        let mut counter = storage.counter.read();
        counter.value += amount;
        storage.counter.write(counter);
    }

    #[storage(read, write)]
    fn test_storage_not_updated(amount: u64) {
        // Report entry should be created:
        // L34: The `Contract::test_storage_not_updated` function has storage bound to local variable `counter` which is not written back to `storage.counter`.
        let mut counter = storage.counter.read();
        counter.value += amount;
    }
}

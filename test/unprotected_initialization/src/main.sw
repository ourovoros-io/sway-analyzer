contract;

abi TestUnprotectedInitialization {
    #[storage(write)]
    fn unsafe_init(value: u64);

    #[storage(read, write)]
    fn safe_init1(value: u64);

    #[storage(read, write)]
    fn safe_init2(value: u64);
}

storage {
    initialized: bool = false,
    value: u64 = 0,
}

impl TestUnprotectedInitialization for Contract {
    // Report entry should be created:
    // L23: The `Contract::unsafe_init` function is an unprotected initializer function. Consider adding a requirement to prevent it from being called multiple times.
    #[storage(write)]
    fn unsafe_init(value: u64) {
        storage.value.write(value);
    }

    // Report entry should not be created
    #[storage(read, write)]
    fn safe_init1(value: u64) {
        require(!storage.initialized.read(), "Already initialized");
        storage.value.write(value);
    }

    // Report entry should not be created
    #[storage(read, write)]
    fn safe_init2(value: u64) {
        if storage.initialized.read() {
            revert(0);
        }
        storage.value.write(value);
    }
}

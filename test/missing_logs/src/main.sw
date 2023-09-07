contract;

use std::logging::log as imported_log;

abi TestMissingLogs {
    #[storage(write)] fn test_missing_log(x: u64);
    #[storage(read, write)] fn test_not_missing_log_1(x: u64);
    #[storage(read, write)] fn test_not_missing_log_2(x: u64);
}

storage {
    value: u64 = 0,
}

impl TestMissingLogs for Contract {
    #[storage(write)]
    fn test_missing_log(x: u64) {
        // Report entry should be created:
        // L17: The `Contract::test_missing_log` function writes to `storage.value` without being logged.
        storage.value.write(x);
    }

    #[storage(read, write)]
    fn test_not_missing_log_1(x: u64) {
        // Report entry should not be created
        storage.value.write(x);
        log(storage.value.read());
    }

    #[storage(read, write)]
    fn test_not_missing_log_2(x: u64) {
        // Report entry should not be created
        storage.value.write(x);
        imported_log(storage.value.read());
    }
}

contract;

use std::constants::ZERO_B256;

use std::logging::log;
use std::logging::log as imported_log;

abi TestMissingLogs {
    #[storage(write)]
    fn test_missing_logs_1(x: u64);
    #[storage(read, write)]
    fn test_missing_logs_2(x: u64);
    #[storage(read, write)]
    fn test_missing_logs_3(x: u64);
    #[storage(read, write)]
    fn test_missing_logs_4(x: u64);
    #[storage(write)]
    fn test_missing_logs_5(x: b256);
    #[storage(read, write)]
    fn test_missing_logs_6(x: b256);
    #[storage(read, write)]
    fn test_missing_logs_7(x: b256);
    #[storage(read, write)]
    fn test_missing_logs_8(x: b256);
}

storage {
    value: u64 = 0,
    admin: b256 = ZERO_B256,
}

impl TestMissingLogs for Contract {
    #[storage(write)]
    fn test_missing_logs_1(x: u64) {
        // Report entry should be created:
        // L37: The `Contract::test_missing_logs_1` function writes to `storage.value` without being logged.
        storage.value.write(x);
    }

    #[storage(read, write)]
    fn test_missing_logs_2(x: u64) {
        // Report entry should not be created
        storage.value.write(x);
        log(storage.value.read());
    }

    #[storage(read, write)]
    fn test_missing_logs_3(x: u64) {
        // Report entry should not be created
        storage.value.write(x);
        std::logging::log(storage.value.read());
    }

    #[storage(read, write)]
    fn test_missing_logs_4(x: u64) {
        // Report entry should not be created
        storage.value.write(x);
        imported_log(storage.value.read());
    }

    #[storage(write)]
    fn test_missing_logs_5(x: b256) {
        // Report entry should be created:
        // L65: The `Contract::test_missing_logs_5` function writes to `storage.admin` without being logged.
        storage.admin.write(x);
    }

    #[storage(read, write)]
    fn test_missing_logs_6(x: b256) {
        // Report entry should not be created
        storage.admin.write(x);
        log(storage.admin.read());
    }

    #[storage(read, write)]
    fn test_missing_logs_7(x: b256) {
        // Report entry should not be created
        storage.admin.write(x);
        std::logging::log(storage.admin.read());
    }

    #[storage(read, write)]
    fn test_missing_logs_8(x: b256) {
        // Report entry should not be created
        storage.admin.write(x);
        imported_log(storage.admin.read());
    }
}

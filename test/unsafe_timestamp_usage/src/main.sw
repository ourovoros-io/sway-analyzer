contract;

use std::block::timestamp as imported_timestamp;
use std::block::timestamp_of_block as imported_timestamp_of_block;

abi TestUnsafeTimestampUsage {
    fn test_unsafe_timestamp_usage_1();
    fn test_unsafe_timestamp_usage_2();
    fn test_unsafe_timestamp_usage_3();
    fn test_unsafe_timestamp_usage_4();
}

impl TestUnsafeTimestampUsage for Contract {
    fn test_unsafe_timestamp_usage_1() {
        // Report entry should be created:
        // L17: The `Contract::test_unsafe_timestamp_usage_1` function contains dependence on a block timestamp, which can be manipulated by attackers: `std::block::timestamp()`
        log(std::block::timestamp());
    }
    
    fn test_unsafe_timestamp_usage_2() {
        // Report entry should be created:
        // L23: The `Contract::test_unsafe_timestamp_usage_2` function contains dependence on a block timestamp, which can be manipulated by attackers: `std::block::timestamp_of_block(0)`
        log(std::block::timestamp_of_block(0));
    }
    
    fn test_unsafe_timestamp_usage_3() {
        // Report entry should be created:
        // L29: The `Contract::test_unsafe_timestamp_usage_3` function contains dependence on a block timestamp, which can be manipulated by attackers: `imported_timestamp()`
        log(imported_timestamp());
    }
    
    fn test_unsafe_timestamp_usage_4() {
        // Report entry should be created:
        // L35: The `Contract::test_unsafe_timestamp_usage_4` function contains dependence on a block timestamp, which can be manipulated by attackers: `imported_timestamp_of_block(0)`
        log(imported_timestamp_of_block(0));
    }
}

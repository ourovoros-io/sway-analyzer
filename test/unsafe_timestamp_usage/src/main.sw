contract;

use std::block::timestamp;
use std::block::timestamp_of_block;
use std::block::timestamp as imported_timestamp;
use std::block::timestamp_of_block as imported_timestamp_of_block;

abi TestUnsafeTimestampUsage {
    fn test_unsafe_timestamp_usage_1();
    fn test_unsafe_timestamp_usage_2();
    fn test_unsafe_timestamp_usage_3();
    fn test_unsafe_timestamp_usage_4();
    fn test_unsafe_timestamp_usage_5();
    fn test_unsafe_timestamp_usage_6();
}

impl TestUnsafeTimestampUsage for Contract {
    fn test_unsafe_timestamp_usage_1() {
        // Report entry should be created:
        // L21: The `Contract::test_unsafe_timestamp_usage_1` function contains dependence on a block timestamp, which can be manipulated by an attacker: `std::block::timestamp()`
        log(std::block::timestamp());
    }
    
    fn test_unsafe_timestamp_usage_2() {
        // Report entry should be created:
        // L27: The `Contract::test_unsafe_timestamp_usage_2` function contains dependence on a block timestamp, which can be manipulated by an attacker: `std::block::timestamp_of_block(0)`
        log(std::block::timestamp_of_block(0));
    }
    
    fn test_unsafe_timestamp_usage_3() {
        // Report entry should be created:
        // L33: The `Contract::test_unsafe_timestamp_usage_3` function contains dependence on a block timestamp, which can be manipulated by an attacker: `imported_timestamp()`
        log(imported_timestamp());
    }
    
    fn test_unsafe_timestamp_usage_4() {
        // Report entry should be created:
        // L39: The `Contract::test_unsafe_timestamp_usage_4` function contains dependence on a block timestamp, which can be manipulated by an attacker: `imported_timestamp_of_block(0)`
        log(imported_timestamp_of_block(0));
    }

    fn test_unsafe_timestamp_usage_6() {
        // Report entry should be created:
        // L45: The `Contract::test_unsafe_timestamp_usage_6` function contains dependence on a block timestamp, which can be manipulated by an attacker: `timestamp_of_block(0)`
        log(timestamp());
    }

    fn test_unsafe_timestamp_usage_5() {
        // Report entry should be created:
        // L51: The `Contract::test_unsafe_timestamp_usage_5` function contains dependence on a block timestamp, which can be manipulated by an attacker: `timestamp()`
        log(timestamp_of_block(0));
    }

}

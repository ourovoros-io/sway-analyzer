contract;

use std::block::timestamp as imported_timestamp;
use std::block::timestamp_of_block as imported_timestamp_of_block;

abi TestWeakPrng {
    fn test_weak_prng_1() -> u64;
    fn test_weak_prng_2() -> u64;
    fn test_weak_prng_3() -> u64;
    fn test_weak_prng_4() -> u64;
    fn test_weak_prng_5() -> u64;
    fn test_weak_prng_6() -> u64;
    fn test_weak_prng_7() -> u64;
    fn test_weak_prng_8() -> u64;
    fn test_weak_prng_9() -> u64;
    fn test_weak_prng_10() -> u64;
    fn test_weak_prng_11() -> u64;
    fn test_weak_prng_12() -> u64;
}

impl TestWeakPrng for Contract {
    fn test_weak_prng_1() -> u64 {
        // Report entry should be created:
        // L25: The `Contract::test_weak_prng_1` function contains weak PRNG due to dependence on a block timestamp: `std::block::timestamp() % 10`
        std::block::timestamp() % 10
    }

    fn test_weak_prng_2() -> u64 {
        // Report entry should be created:
        // L31: The `Contract::test_weak_prng_2` function contains weak PRNG due to dependence on a block timestamp: `std::block::timestamp_of_block(1) % 10`
        std::block::timestamp_of_block(1) % 10
    }

    fn test_weak_prng_3() -> u64 {
        // Report entry should be created:
        // L37: The `Contract::test_weak_prng_3` function contains weak PRNG due to dependence on a block timestamp: `imported_timestamp() % 10`
        imported_timestamp() % 10
    }

    fn test_weak_prng_4() -> u64 {
        // Report entry should be created:
        // L43: The `Contract::test_weak_prng_4` function contains weak PRNG due to dependence on a block timestamp: `imported_timestamp_of_block(1) % 10`
        imported_timestamp_of_block(1) % 10
    }

    fn test_weak_prng_5() -> u64 {
        let x = std::block::timestamp();
        // Report entry should be created:
        // L50: The `Contract::test_weak_prng_5` function contains weak PRNG due to dependence on a block timestamp: `x % 10`
        x % 10
    }

    fn test_weak_prng_6() -> u64 {
        let x = std::block::timestamp_of_block(1);
        // Report entry should be created:
        // L57: The `Contract::test_weak_prng_6` function contains weak PRNG due to dependence on a block timestamp: `x % 10`
        x % 10
    }

    fn test_weak_prng_7() -> u64 {
        let x = imported_timestamp();
        // Report entry should be created:
        // L64: The `Contract::test_weak_prng_7` function contains weak PRNG due to dependence on a block timestamp: `x % 10`
        x % 10
    }

    fn test_weak_prng_8() -> u64 {
        let x = imported_timestamp_of_block(1);
        // Report entry should be created:
        // L71: The `Contract::test_weak_prng_8` function contains weak PRNG due to dependence on a block timestamp: `x % 10`
        x % 10
    }

    fn test_weak_prng_9() -> u64 {
        let x = std::block::timestamp();
        let y = x;
        // Report entry should be created:
        // L79: The `Contract::test_weak_prng_9` function contains weak PRNG due to dependence on a block timestamp: `y % 10`
        y % 10
    }

    fn test_weak_prng_10() -> u64 {
        let x = std::block::timestamp_of_block(1);
        let y = x;
        // Report entry should be created:
        // L87: The `Contract::test_weak_prng_10` function contains weak PRNG due to dependence on a block timestamp: `y % 10`
        y % 10
    }

    fn test_weak_prng_11() -> u64 {
        let x = imported_timestamp();
        let y = x;
        // Report entry should be created:
        // L95: The `Contract::test_weak_prng_11` function contains weak PRNG due to dependence on a block timestamp: `y % 10`
        y % 10
    }

    fn test_weak_prng_12() -> u64 {
        let x = imported_timestamp_of_block(1);
        let y = x;
        // Report entry should be created:
        // L103: The `Contract::test_weak_prng_12` function contains weak PRNG due to dependence on a block timestamp: `y % 10`
        y % 10
    }
}

contract;

abi TestStrictEquality {
    #[storage(read)]
    fn test_strict_equality_1() -> bool;
    #[storage(read)]
    fn test_strict_equality_2() -> bool;
    fn test_strict_equality_3() -> bool;
}

storage {
    balance: u64 = 0,
}

impl TestStrictEquality for Contract {
    #[storage(read)]
    fn test_strict_equality_1() -> bool {
        // Report entry should be created:
        // L20: The `Contract::test_strict_equality_1` function contains a strict equality check: `storage.balance.read() == 100`. Don't use strict equality to determine if an account has enough balance.
        return storage.balance.read() == 100;
    }

    #[storage(read)]
    fn test_strict_equality_2() -> bool {
        let a = storage.balance.read();
        // Report entry should be created:
        // L28: The `Contract::test_strict_equality_2` function contains a strict equality check: `a == 100`. Don't use strict equality to determine if an account has enough balance.
        return  a == 100;
    }

    fn test_strict_equality_3() -> bool {
        let a = 20;
        let b = 2000;
        // Report entry should not be created:
        if(a == 200) || (2000 == b) {
            return true;
        } else {
            return false;
        }
    }
}

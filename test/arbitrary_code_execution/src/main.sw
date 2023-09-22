contract;

use std::constants::ZERO_B256;
use std::auth::msg_sender as imported_msg_sender;

abi TestArbitraryCodeExecution {
    fn test_ldc_unrestricted();

    #[storage(read)] fn test_ldc_restricted_1();
    #[storage(read)] fn test_ldc_restricted_2();
    #[storage(read)] fn test_ldc_restricted_3();
    #[storage(read)] fn test_ldc_restricted_4();
    #[storage(read)] fn test_ldc_restricted_5();
    #[storage(read)] fn test_ldc_restricted_6();
}

storage {
    owner: Identity = Identity::Address(Address::from(ZERO_B256)),
}

impl TestArbitraryCodeExecution for Contract {
    fn test_ldc_unrestricted() {
        asm(r1: 0, r2: 0, r3: 0) {
            // Report entry should be created:
            // L26: The `Contract::test_ldc_unrestricted` function uses the `LDC` instruction without access restriction: `ldc r1 r2 r3`. Consider checking against `msg_sender()` in order to limit access.
            ldc r1 r2 r3;
        };
    }

    #[storage(read)]
    fn test_ldc_restricted_1() {
        require(msg_sender().unwrap() == storage.owner.read(), "Only owner");
        asm(r1: 0, r2: 0, r3: 0) {
            // Report entry should not be created
            ldc r1 r2 r3;
        };
    }

    #[storage(read)]
    fn test_ldc_restricted_2() {
        require(imported_msg_sender().unwrap() == storage.owner.read(), "Only owner");
        asm(r1: 0, r2: 0, r3: 0) {
            // Report entry should not be created
            ldc r1 r2 r3;
        };
    }

    #[storage(read)]
    fn test_ldc_restricted_3() {
        require(std::auth::msg_sender().unwrap() == storage.owner.read(), "Only owner");
        asm(r1: 0, r2: 0, r3: 0) {
            // Report entry should not be created
            ldc r1 r2 r3;
        };
    }

    #[storage(read)]
    fn test_ldc_restricted_4() {
        if msg_sender().unwrap() != storage.owner.read() {
            revert(0);
        }
        asm(r1: 0, r2: 0, r3: 0) {
            // Report entry should not be created
            ldc r1 r2 r3;
        };
    }

    #[storage(read)]
    fn test_ldc_restricted_5() {
        if imported_msg_sender().unwrap() != storage.owner.read() {
            revert(0);
        }
        asm(r1: 0, r2: 0, r3: 0) {
            // Report entry should not be created
            ldc r1 r2 r3;
        };
    }

    #[storage(read)]
    fn test_ldc_restricted_6() {
        if std::auth::msg_sender().unwrap() != storage.owner.read() {
            revert(0);
        }
        asm(r1: 0, r2: 0, r3: 0) {
            // Report entry should not be created
            ldc r1 r2 r3;
        };
    }
}

contract;

use std::context::msg_amount;
use std::context::msg_amount as alias_msg_amount;

abi TestMsgAmountInLoop {
    fn test_direct_msg_amount_in_loop();
    fn test_import_msg_amount_in_loop();
    fn test_alias_msg_amount_in_loop();
}

impl TestMsgAmountInLoop for Contract {
    fn test_direct_msg_amount_in_loop() {
        let mut value = 0;
        while true {
            // Report entry should be created:
            // L18: The `Contract::test_direct_msg_amount_in_loop` function makes a call to `std::context::msg_amount()` in a loop. Store the value in a variable outside the loop and decrement it over each iteration.
            value += std::context::msg_amount();
        }
    }

    fn test_import_msg_amount_in_loop() {
        let mut value = 0;
        while true {
            // Report entry should be created:
            // L27: The `Contract::test_import_msg_amount_in_loop` function makes a call to `msg_amount()` in a loop. Store the value in a variable outside the loop and decrement it over each iteration.
            value += msg_amount();
        }
    }

    fn test_alias_msg_amount_in_loop() {
        let mut value = 0;
        while true {
            // Report entry should be created:
            // L36: The `Contract::test_alias_msg_amount_in_loop` function makes a call to `alias_msg_amount()` in a loop. Store the value in a variable outside the loop and decrement it over each iteration.
            value += alias_msg_amount();
        }
    }
}

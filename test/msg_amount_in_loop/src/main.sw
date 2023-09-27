contract;

use std::context::msg_amount;
use std::context::msg_amount as alias_msg_amount;

abi TestMsgAmountInLoop {
    fn test_msg_amount_in_loop_1();
    fn test_msg_amount_in_loop_2();
    fn test_msg_amount_in_loop_3();
    fn test_msg_amount_in_loop_4();
}

impl TestMsgAmountInLoop for Contract {
    fn test_msg_amount_in_loop_1() {
        let mut value = 0;
        while true {
            // Report entry should be created:
            // L18: The `Contract::test_msg_amount_in_loop_1` function makes a call to `std::context::msg_amount()` in a loop. Store the value in a variable outside the loop and decrement it over each iteration.
            value += std::context::msg_amount();
        }
    }

    fn test_msg_amount_in_loop_2() {
        let mut value = 0;
        while true {
            // Report entry should be created:
            // L27: The `Contract::test_msg_amount_in_loop_2` function makes a call to `msg_amount()` in a loop. Store the value in a variable outside the loop and decrement it over each iteration.
            value += msg_amount();
        }
    }

    fn test_msg_amount_in_loop_3() {
        let mut value = 0;
        while true {
            // Report entry should be created:
            // L36: The `Contract::test_msg_amount_in_loop_3` function makes a call to `alias_msg_amount()` in a loop. Store the value in a variable outside the loop and decrement it over each iteration.
            value += alias_msg_amount();
        }
    }

    fn test_msg_amount_in_loop_4() {
        let mut value = 0;
        while true {
            // Report entry should be created:
            // L36: The `Contract::test_msg_amount_in_loop_3` function makes a call to `alias_msg_amount()` in a loop. Store the value in a variable outside the loop and decrement it over each iteration.
            value += alias_msg_amount();
        }
    }
}

contract;

abi ContractA {
    fn receive(field_1: bool, field_2: u64) -> u64;
}

const CONTRACT_A_ID = 0x79fa8779bed2f36c3581d01c79df8da45eee09fac1fd76a5a656e16326317ef0;

abi TestExternalCallInLoop {
    fn test_external_call_in_loop_1();
    fn test_external_call_in_loop_2();
    fn test_external_call_in_loop_3();
    fn test_external_call_in_loop_4();
}

impl TestExternalCallInLoop for Contract {
    // Report entry should not be created
    fn test_external_call_in_loop_1() {
        let x = abi(ContractA, CONTRACT_A_ID);
        let _return_value = x.receive(true, 3);
    }

    // Report entry should not be created
    fn test_external_call_in_loop_2() {
        let _return_value = abi(ContractA, CONTRACT_A_ID).receive(true, 3);
    }

    fn test_external_call_in_loop_3() {
        let x = abi(ContractA, CONTRACT_A_ID);
        while true {
            // Report entry should be created:
            // L33: The `Contract::test_external_call_in_loop_3` function performs an external call in a loop: `x.receive(true, 3)`
            let _return_value = x.receive(true, 3);
        }
    }

    fn test_external_call_in_loop_4() {
        while true {
            // Report entry should be created:
            // L41: The `Contract::test_external_call_in_loop_4` function performs an external call in a loop: `abi(ContractA, CONTRACT_A_ID).receive(true, 3)`
            let _return_value = abi(ContractA, CONTRACT_A_ID).receive(true, 3);
        }
    }
}

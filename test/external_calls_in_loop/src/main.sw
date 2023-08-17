contract;

abi ContractA {
    fn receive(field_1: bool, field_2: u64) -> u64;
}

const CONTRACT_A_ID = 0x79fa8779bed2f36c3581d01c79df8da45eee09fac1fd76a5a656e16326317ef0;

abi TestExternalCallsInLoop {
    fn test_does_not_make_external_call_in_loop();
    fn test_does_make_external_call_in_loop();
}

impl TestExternalCallsInLoop for Contract {
    // Report entry should not be created
    fn test_does_not_make_external_call_in_loop() {
        let x = abi(ContractA, CONTRACT_A_ID);
        let _return_value = x.receive(true, 3);
    }
    
    // Report entry should be created:
    // L26: The `Contract::test_does_make_external_call_in_loop` function performs an external call in a loop: `x.receive(true, 3)`
    fn test_does_make_external_call_in_loop() {
        let x = abi(ContractA, CONTRACT_A_ID);
        while true {
            let _return_value = x.receive(true, 3);
        }
    }
}

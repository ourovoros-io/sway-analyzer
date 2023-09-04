contract;

abi TestDiscardedAssignments {
    fn test_assignment_discarded_1();
    fn test_assignment_not_discarded_1();
    
    fn test_assignment_discarded_2();
    fn test_assignment_not_discarded_2();

    #[storage(read)] fn test_assignment_discarded_3();
    #[storage(read, write)] fn test_assignment_not_discarded_3();
}

struct Counter {
    value: u64,
}

storage {
    counter: Counter = Counter {
        value: 0,
    },
}

impl TestDiscardedAssignments for Contract {
    fn test_assignment_discarded_1() {
        // Report entry should be created:
        // L28: The `Contract::test_assignment_discarded_1` function makes an assignment to `x` which is discarded.
        let x = 1;
    }

    fn test_assignment_not_discarded_1() {
        // Report entry should not be created
        let x = 1;
        log(x);
    }

    fn test_assignment_discarded_2() {
        // Report entry should be created:
        // L40: The `Contract::test_assignment_discarded_2` functions makes an assignment to `x` which is discarded by the assignment made on L44.
        let mut x = 2;
        
        // Report entry should be created:
        // L44: The `Contract::test_assignment_discarded_2` function makes an assignment to `x` which is discarded.
        x = 1;
    }

    fn test_assignment_not_discarded_2() {
        // Report entry should not be created
        let mut x = 2;
        log(x);
        
        // Report entry should not be created
        x = 1;
        log(x);
    }

    #[storage(read)]
    fn test_assignment_discarded_3() {
        // Report entry should be created:
        // L61: The `Contract::test_assignment_discarded_3` function makes an assignment to `counter` which is discarded.
        let mut counter = storage.counter.read();

        // Report entry should be created:
        // L65: The `Contract::test_assignment_discarded_3` function makes an assignment to `counter.value` which is discarded.
        counter.value += 1;
    }

    #[storage(read, write)]
    fn test_assignment_not_discarded_3() {
        let mut counter = storage.counter.read();
        counter.value += 1;

        // Report entry should not be created
        storage.counter.write(counter);
    }
}

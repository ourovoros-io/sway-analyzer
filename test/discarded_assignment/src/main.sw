contract;

abi TestDiscardedAssignment {
    fn test_discarded_assignment_1();
    fn test_discarded_assignment_2();
    fn test_discarded_assignment_3();
    fn test_discarded_assignment_4();
    #[storage(read)]
    fn test_discarded_assignment_5();
    #[storage(read, write)]
    fn test_discarded_assignment_6();
}

struct Counter {
    value: u64,
}

storage {
    counter: Counter = Counter {
        value: 0,
    },
}

impl TestDiscardedAssignment for Contract {
    fn test_discarded_assignment_1() {
        // Report entry should be created:
        // L28: The `Contract::test_discarded_assignment_1` function makes an assignment to `x` which is discarded.
        let x = 1;
    }

    fn test_discarded_assignment_2() {
        // Report entry should not be created
        let x = 1;
        log(x);
    }

    fn test_discarded_assignment_3() {
        // Report entry should be created:
        // L40: The `Contract::test_discarded_assignment_3` function makes an assignment to `x` which is discarded by the assignment made on L44.
        let mut x = 2;
        
        // Report entry should be created:
        // L44: The `Contract::test_discarded_assignment_3` function makes an assignment to `x` which is discarded.
        x = 1;
    }

    fn test_discarded_assignment_4() {
        // Report entry should not be created
        let mut x = 2;
        log(x);
        
        // Report entry should not be created
        x = 1;
        log(x);
    }

    #[storage(read)]
    fn test_discarded_assignment_5() {
        // Report entry should be created:
        // L61: The `Contract::test_discarded_assignment_5` function makes an assignment to `counter` which is discarded.
        let mut counter = storage.counter.read();

        // Report entry should be created:
        // L65: The `Contract::test_discarded_assignment_5` function makes an assignment to `counter.value` which is discarded.
        counter.value += 1;
    }

    #[storage(read, write)]
    fn test_discarded_assignment_6() {
        let mut counter = storage.counter.read();
        counter.value += 1;

        // Report entry should not be created
        storage.counter.write(counter);
    }
}

contract;


abi TestPotentialInfiniteLoops {
    fn potential_infinite_loop();
}

impl TestPotentialInfiniteLoops for Contract {
    fn potential_infinite_loop() {
        let s = 1; 
        let z = 10;
        // Report entry should not be created
        while s < z {
            
        }

        // Report entry should not be created
        while true {
            let a = 1;
            break;
        }
        // Report entry should be created:
    	// L25: Found potential infinite loop in function `fn potential_infinite_loop()`.Consider adding a `break` statement.
        while true {
            let a = 1;
            let b = 2;
            let x = 10;
        }

        // Report entry should not be created
        let mut q = true;
        while q {
            q = false;
        }
        // Report entry should not be created
        let z = true;
        while z {
            break;
        }

        // Report entry should be created:
        // L44: Found potential infinite loop in function `fn potential_infinite_loop()`.Consider adding a `break` statement.
        while z {

        }

        // Report entry should be created:
        // L51: Found potential infinite loop in function `fn potential_infinite_loop()`.Consider adding a `break` statement.
        let w = false;
        // TODO
        while !w {
            
        }
    }
}

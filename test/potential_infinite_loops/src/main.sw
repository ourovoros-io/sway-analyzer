contract;


abi TestPotentialInfiniteLoops {
    fn potential_infinite_loop();
}

impl TestPotentialInfiniteLoops for Contract {
    fn potential_infinite_loop() {
        let s = 1; 
        let z = 10;
        // L13: The `Contract::potential_infinite_loop` function contains a potentially infinite loop: `while s < z { ... }`. Consider adding a `break` statement.
        while s < z {}

        let mut s = 1; 
        let z = 10;
        // Report entry should not be created
        while s < z {
            s += 1;
        }

        let s = 1; 
        let mut z = 10;
        // Report entry should not be created
        while s < z {
            z -= 1;
        }

        let mut s = 1; 
        let z = 10;
        // Report entry should not be created
        while s < z {
            s = s + 1;
        }

        let s = 1; 
        let mut z = 10;
        // Report entry should not be created
        while s < z {
            z = z - 1;
        }

        let mut s = 1; 
        let z = 10;
        // Report entry should not be created
        while s <= z {
            s += 1;
        }

        let s = 1; 
        let mut z = 10;
        // Report entry should not be created
        while s <= z {
            z -= 1;
        }

        let mut s = 1; 
        let z = 10;
        // Report entry should not be created
        while s <= z {
            s = s + 1;
        }

        let s = 1; 
        let mut z = 10;
        // Report entry should not be created
        while s <= z {
            z = z - 1;
        }

        let mut s = 1; 
        let z = 10;
        // Report entry should not be created
        while z > s {
            s += 1;
        }

        let s = 1; 
        let mut z = 10;
        // Report entry should not be created
        while z > s {
            z -= 1;
        }

        let mut s = 1; 
        let z = 10;
        // Report entry should not be created
        while z > s {
            s = s + 1;
        }

        let s = 1; 
        let mut z = 10;
        // Report entry should not be created
        while z > s {
            z = z - 1;
        }

        let mut s = 1; 
        let z = 10;
        // Report entry should not be created
        while z >= s {
            s += 1;
        }

        let s = 1; 
        let mut z = 10;
        // Report entry should not be created
        while z >= s {
            z -= 1;
        }

        let mut s = 1; 
        let z = 10;
        // Report entry should not be created
        while z >= s {
            s = s + 1;
        }

        let s = 1; 
        let mut z = 10;
        // Report entry should not be created
        while z >= s {
            z = z - 1;
        }

        // Report entry should not be created
        while true {
            let a = 1;
            break;
        }

        let mut s = 10;
        let z = 10;
        // Report entry should not be created
        while s == z {
            s = s - 1;
        }

        let mut s = 1; 
        let z = 10;
        // Report entry should not be created
        while s != z {
            s = s + 1;
        }

        // Report entry should be created:
    	// L149: The `Contract::potential_infinite_loop` function contains a potentially infinite loop: `while true { ... }`. Consider adding a `break` statement.
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
        // Report entry should not be created
        let mut q = false;
        while !q {
            q = true;
        }
        // Report entry should not be created
        q = false;
        while !q {
            break;
        }

        // Report entry should be created:
        // L178: The `Contract::potential_infinite_loop` function contains a potentially infinite loop: `while z { ... }`. Consider adding a `break` statement.
        while z {}
    }
}

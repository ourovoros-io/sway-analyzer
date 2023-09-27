contract;

abi TestMagicNumber {
    fn test_magic_number();
}

fn run_this(_amount: u64) {}

// Report entry should not be created
const VALUE: u64 = 120;

impl TestMagicNumber for Contract {
    fn test_magic_number() {
        let mut radius = VALUE;

        // Report entry should be created:
        // L18: The `Contract::test_magic_numbers` function contains magic number usage: `3 * radius`. Consider introducing a constant value.
        let _di = 3 * radius * radius;

        // Report entry should be created:
        // L22: The `Contract::test_magic_numbers` function contains magic number usage: `radius * 10 / 200`. Consider introducing a constant value.
        let _xi = radius * 10 / 200;
        
        // Report entry should be created:
        // L26: The `Contract::test_magic_numbers` function contains magic number usage: `radius * 10 * 200`. Consider introducing a constant value.
        let _yi = radius * 10 * 200;

        // Report entry should be created:
        // L30: The `Contract::test_magic_numbers` function contains magic number usage: `radius < 221`. Consider introducing a constant value.
        if radius < 221 {
            return;
        }

        // Report entry should not be created:
        if radius > radius {
            return;
        }

        // Report entry should be created:
        // L41: The `Contract::test_magic_numbers` function contains magic number usage: `radius > 5`. Consider introducing a constant value.
        while radius > 5 {
            radius = radius - 1;
        }

        // Report entry should not be created
        run_this(5);
    }
}

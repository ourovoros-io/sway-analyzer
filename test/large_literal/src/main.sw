contract;

use std::constants::ZERO_B256;

// Report entry should be created:
// L7: The `X` constant contains a large literal: `1000000000`. Consider refactoring it to be more readable: `1_000_000_000`
pub const X: u64 = 1000000000;

// Report entry should not be created
pub const Y: u64 = 10000;

configurable {
    // Report entry should be created:
    // L15: Configurable contains a large literal: `1000000000`. Consider refactoring it to be more readable: `1_000_000_000`
    C_CONST: u64 = 1000000000,

    // Report entry should not be created
    D_CONST: u64 = 10000,
}

abi TestLargeLiteral {
    fn test_large_literal_1();
    fn test_large_literal_2();
    fn test_large_literal_3();
}

trait HasValue {
    const VALUE: u64;
    const VALUE2: u64;
}

struct Error {}

impl HasValue for Error {
    // Report entry should be created:
    // L37: The `Error::VALUE` constant contains a large literal: `25000000`. Consider refactoring it to be more readable: `25_000_000`
    const VALUE: u64 = 25000000;

    // Report entry should not be created
    const VALUE2: u64 = 1000;
}

fn function_call_single_large_literal(_amount: u64) {}
fn function_call_double_large_literal(_amount: u64, _to: Address) {}

impl TestLargeLiteral for Contract {
    fn test_large_literal_1() {
        // Report entry should be created:
        // L50: The `Z` constant in the `const_func_large_literals` function contains a large literal: `1000000000000`. Consider refactoring it to be more readable: `1_000_000_000_000`
        const Z = 1000000000000;

        // Report entry should not be created
        const L = 100000;
    }

    fn test_large_literal_2() {
        // Report entry should be created:
        // L59: The `Contract::let_func_large_literals` function contains a large literal: `20000000000`. Consider refactoring it to be more readable: `20_000_000_000`
        let _big_a = 20000000000;

        // Report entry should not be created
        let _not_big_a = 200000;
    }

    fn test_large_literal_3() {
        // Report entry should be created:
        // L69: The `Contract::large_literals_fn_calls` function contains a large literal: `3000000000`. Consider refactoring it to be more readable: `3_000_000_000`
        function_call_single_large_literal(3000000000);

        // Report entry should not be created
        function_call_single_large_literal(30000);

        // Report entry should be created:
        // L76: The `Contract::large_literals_fn_calls` function contains a large literal: `400000000000`. Consider refactoring it to be more readable: `400_000_000_000`
        function_call_double_large_literal(400000000000, Address::from(0x0000000000000000000000000000000000000000000000000000000000000000));

        // Report entry should not be created
        function_call_double_large_literal(400000, Address::from(ZERO_B256));
    }
}

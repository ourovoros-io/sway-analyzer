contract;

use std::constants::ZERO_B256;

// Case 1: Large literals usage in constant at contract level
// Report entry should be created:
// L6: Found large literal in contract => `const X: u64 = 1000000000`. Consider refactoring it in order to be more readable: `const X: u64 = 1_000_000_000`.
pub const X: u64 = 1000000000;

// Report entry should not be created
pub const Y: u64 = 10000;

// Case2: Large literals in constants in configurable
configurable {
    // Report entry should be created:
    // L15: Found large literal in configurable => `C_CONST: u64 = 1000000000`. Consider refactoring it in order to be more readable: `C_CONST: u64 = 1_000_000_000`.
    C_CONST: u64 = 1000000000,
    // Report entry should not be created
    D_CONST: u64 = 10000,
    STR: str[6] = "helloo",
}

abi TestLargeLiterals {
    fn large_literals_fn_calls();
    fn const_func_large_literals();
    fn let_func_large_literals();
}

trait HasValue {
    const VALUE: u64;
    const VALUE2: u64;
}

struct Error {}

// Case 3: Large literals usage in trait impl
impl HasValue for Error {
    // Report entry should be created:
    // L44: Found large literal in contract => `impl HasValue for Error { const VALUE: u64 = 1000000; }`. Consider refactoring it in order to be more readable: `impl HasValue for Error { const VALUE: u64 = 1_000_000; }`.
    const VALUE: u64 = 25000000;
    // Report entry should not be created
    const VALUE2: u64 = 1000;
}

// Dummies for large_literals
fn function_call_single_large_literals(_amount: u64) {}
fn function_call_nested_large_literals(_amount: u64, _to: Address) {}
fn function_call_nested_large_literals_2(_from: Address, _to: Address, _amount: u64) {}

impl TestLargeLiterals for Contract {
    fn const_func_large_literals() {
        // Report entry should be created:
        // L53: Found large literal in function : `fn const_func_large_literals()` => `const Z = 1000000000000`. Consider refactoring it in order to be more readable: `const Z = 1_000_000_000_000`.
        const Z = 1000000000000;
        // Report entry should not be created
        const L = 100000;
    }

    fn let_func_large_literals() {
        // Report entry should be created:
        // L61: Found large literal in function : `fn let_func_large_literals()` => `let big_a = 20000000000;`. Consider refactoring it in order to be more readable: `let big_a = 20_000_000_000;`.
        let _big_a = 20000000000;
        // Report entry should not be created
        let _not_big_a = 200000;
    }

    // Large literals
    fn large_literals_fn_calls() {
        // Report entry should be created:
        // L70: Found large literal in function : `fn large_literals_fn_calls()` => `function_call_single_large_literals(3000000000)`. Consider refactoring it in order to be more readable: `function_call_single_large_literals(3_000_000_000)`.
        function_call_single_large_literals(3000000000);
        // Report entry should not be created
        function_call_single_large_literals(30000);

        // Report entry should be created:
        // L76: Found large literal in function : `fn large_literals_fn_calls()` => `function_call_nested_large_literals(400000000000, Address::from(ZERO_B256))`. Consider refactoring it in order to be more readable: `function_call_nested_large_literals(400_000_000_000, Address::from(ZERO_B256))`.
        function_call_nested_large_literals(400000000000, Address::from(ZERO_B256));
        // Report entry should not be created
        function_call_nested_large_literals(400000, Address::from(ZERO_B256));

        // Report entry should be created:
        // L82: Found large literal in function : `fn large_literals_fn_calls()` => `function_call_nested_large_literals_2(Address::from(ZERO_B256), Address::from(ZERO_B256), 500000000000)`. Consider refactoring it in order to be more readable: `function_call_nested_large_literals_2(Address::from(ZERO_B256), Address::from(ZERO_B256), 500_000_000_000)`.
        function_call_nested_large_literals_2(Address::from(ZERO_B256), Address::from(ZERO_B256), 500000000000);
        // Report entry should not be created
        function_call_nested_large_literals_2(Address::from(ZERO_B256), Address::from(ZERO_B256), 5000);
    }
}

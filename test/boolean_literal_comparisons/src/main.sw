contract;

abi MyContract {
    fn test_no_boolean_literal_comparisons() -> bool;
    fn test_boolean_literal_comparisons() -> bool;
}

// Report entry should not be created
pub const BOOL1: bool = true;

// Report entry should be created:
// L13: Found a comparison with a boolean literal, which is unnecessary: `true != false`
pub const BOOL2: bool = true != false;

configurable {
    // Report entry should not be created
    BOOL3: bool = true,

    // Report entry should be created:
    // L21: Found a comparison with a boolean literal, which is unnecessary: `true != false`
    BOOL4: bool = true != false,
}

storage {
    // Report entry should not be created
    bool1: bool = true,

    // Report entry should be created:
    // L30: Found a comparison with a boolean literal, which is unnecessary: `true != false`
    bool2: bool = true != false,
}

impl MyContract for Contract {
    // Report entry should not be created
    fn test_no_boolean_literal_comparisons() -> bool {
        true
    }

    // Report entry should be created:
    // L42: The `Contract::test_boolean_literal_comparisons` function contains a comparison with a boolean literal, which is unnecessary: `true != false`
    fn test_boolean_literal_comparisons() -> bool {
        true != false
    }
}

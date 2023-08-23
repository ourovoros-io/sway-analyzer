contract;

abi TestBooleanComparisons {
    fn test_no_boolean_literal_comparisons() -> bool;
    fn test_boolean_literal_comparisons() -> bool;
    fn test_boolean_negation_comparisons() -> bool;
}

// Report entry should not be created
pub const BOOL1: bool = true;

// Report entry should be created:
// L13: Found a comparison with a boolean literal, which is unnecessary: `true != false`
pub const BOOL2: bool = true != false;

// Report entry should be created:
// L18: Found a comparison with a boolean literal, which is unnecessary: `true != false`
pub const BOOL3: bool = !true != !false;

configurable {
    // Report entry should not be created
    BOOL4: bool = true,

    // Report entry should be created:
    // L25: Found a comparison with a boolean literal, which is unnecessary: `true != false`
    BOOL5: bool = true != false,

    // Report entry should be created:
    // L29: Found a comparison with a boolean literal, which is unnecessary: `!true != !false`
    BOOL6: bool = !true != !false,
}

storage {
    // Report entry should not be created
    bool1: bool = true,

    // Report entry should be created:
    // L38: Found a comparison with a boolean literal, which is unnecessary: `true != false`
    bool2: bool = true != false,

    // Report entry should be created:
    // L42: Found a comparison with a boolean literal, which is unnecessary: `!true != !false`
    bool3: bool = !true != !false,
}

impl TestBooleanComparisons for Contract {
    // Report entry should not be created
    fn test_no_boolean_literal_comparisons() -> bool {
        true
    }

    // Report entry should be created:
    // L54: The `Contract::test_boolean_literal_comparisons` function contains a comparison with a boolean literal, which is unnecessary: `true != false`
    fn test_boolean_literal_comparisons() -> bool {
        true != false
    }

    // Report entry should be created:
    // L61: The `Contract::test_boolean_negation_comparisons` function contains a comparison with a boolean literal, which is unnecessary: `true != false`
    fn test_boolean_negation_comparisons() -> bool {
        !true != !false
    }
}

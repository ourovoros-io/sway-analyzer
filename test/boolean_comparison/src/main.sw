contract;

abi TestBooleanComparison {
    fn test_boolean_comparison_1() -> bool;
    fn test_boolean_comparison_2() -> bool;
    fn test_boolean_comparison_3() -> bool;
    fn test_boolean_comparison_4();
    fn test_boolean_comparison_5();
}

// Report entry should not be created
pub const BOOL1: bool = true;

// Report entry should be created:
// L16: The `BOOL2` constant contains a comparison with a boolean literal, which is unnecessary: `true != false`
pub const BOOL2: bool = true != false;

// Report entry should be created:
// L20: The `BOOL3` constant contains a comparison with a boolean literal, which is unnecessary: `!true != !false`
pub const BOOL3: bool = !true != !false;

configurable {
    // Report entry should not be created
    BOOL4: bool = true,

    // Report entry should be created:
    // L28: Configurable contains a comparison with a boolean literal, which is unnecessary: `true != false`
    BOOL5: bool = true != false,

    // Report entry should be created:
    // L32: Configurable contains a comparison with a boolean literal, which is unnecessary: `!true != !false`
    BOOL6: bool = !true != !false,
}

storage {
    // Report entry should not be created
    bool1: bool = true,

    // Report entry should be created:
    // L41: Storage contains a comparison with a boolean literal, which is unnecessary: `true != false`
    bool2: bool = true != false,

    // Report entry should be created:
    // L45: Storage contains a comparison with a boolean literal, which is unnecessary: `!true != !false`
    bool3: bool = !true != !false,
}

impl TestBooleanComparison for Contract {
    // Report entry should not be created
    fn test_boolean_comparison_1() -> bool {
        true
    }

    // Report entry should be created:
    // L57: The `Contract::test_boolean_comparison_2` function contains a comparison with a boolean literal, which is unnecessary: `true != false`
    fn test_boolean_comparison_2() -> bool {
        true != false
    }

    // Report entry should be created:
    // L63: The `Contract::test_boolean_comparison_3` function contains a comparison with a boolean literal, which is unnecessary: `!true != !false`
    fn test_boolean_comparison_3() -> bool {
        !true != !false
    }

    // Report entry should be created:
    // L69: The `Contract::test_boolean_comparison_4` function contains a comparison with a boolean literal, which is unnecessary: `true`
    fn test_boolean_comparison_4() {
        if true {}
    }

    // Report entry should be created:
    // L75: The `Contract::test_boolean_comparison_5` function contains a comparison with a boolean literal, which is unnecessary: `!false`
    fn test_boolean_comparison_5() {
        if !false {}
    }
}

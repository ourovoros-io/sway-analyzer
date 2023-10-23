contract;

abi TestRedundantComparison {
    fn test_redundant_comparisons();
}

impl TestRedundantComparison for Contract {
    fn test_redundant_comparisons() {
        // Report entry should be created:
        // L11: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 == 10`
        if 10 == 10 {}

        // Report entry should be created:
        // L15: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 != 10`
        if 10 != 10 {}

        // Report entry should be created:
        // L19: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 < 10`
        if 10 < 10 {}

        // Report entry should be created:
        // L23: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 > 10`
        if 10 > 10 {}

        // Report entry should be created:
        // L27: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 <= 10`
        if 10 <= 10 {}

        // Report entry should be created:
        // L31: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 >= 10`
        if 10 >= 10 {}

        // Report entries should be created:
        // L36: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 == 10`
        // L36: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 != 10`
        if 10 == 10 || 10 != 10 {}

        // Report entry should be created:
        // L40: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 == 10`
        while 10 == 10 {}

        // Report entry should be created:
        // L44: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 != 10`
        while 10 != 10 {}

        // Report entry should be created:
        // L48: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 > 10`
        while 10 > 10 {}

        // Report entry should be created:
        // L52: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 < 10`
        while 10 < 10 {}

        // Report entry should be created:
        // L56: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 >= 10`
        while 10 >= 10 {}

        // Report entry should be created:
        // L60: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 <= 10`
        while 10 <= 10 {}

        // Report entries should be created:
        // L65: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 == 10`
        // L65: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `10 != 10`
        while 10 == 10 || 10 != 10 {}
        
        let i = 10;
        
        // Report entry should be created:
        // L71: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i == 10`
        if i == 10 {}

        // Report entry should be created:
        // L75: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i != 10`
        if i != 10 {}

        // Report entry should be created:
        // L79: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i > 10`
        if i > 10 {}

        // Report entry should be created:
        // L83: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i < 10`
        if i < 10 {}

        // Report entry should be created:
        // L87: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i >= 10`
        if i >= 10 {}

        // Report entry should be created:
        // L91: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i <= 10`
        if i <= 10 {}

        // Report entry should be created:
        // L95: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i == 10`
        while i == 10 {}

        // Report entry should be created:
        // L99: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i != 10`
        while i != 10 {}

        // Report entry should be created:
        // L103: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i > 10`
        while i > 10 {}

        // Report entry should be created:
        // L107: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i < 10`
        while i < 10 {}

        // Report entry should be created:
        // L111: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i >= 10`
        while i >= 10 {}

        // Report entry should be created:
        // L115: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i <= 10`
        while i <= 10 {}

        // Report entry should be created:
        // L119: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i == 10`
        // L119: The `Contract::test_redundant_comparisons` function contains a redundant comparison: `i != 10`
        while i == 10 || i != 10 {}

        let mut i = 10;

        // Report entry should not be created
        if i == 10 {}

        // Report entry should not be created
        if i != 10 {}

        // Report entry should not be created
        if i > 10 {}

        // Report entry should not be created
        if i < 10 {}

        // Report entry should not be created
        if i >= 10 {}

        // Report entry should not be created
        if i <= 10 {}

        // Report entry should not be created
        while i == 10 {}

        // Report entry should not be created
        while i != 10 {}

        // Report entry should not be created
        while i > 10 {}

        // Report entry should not be created
        while i < 10 {}

        // Report entry should not be created
        while i >= 10 {}

        // Report entry should not be created
        while i <= 10 {}

        // Report entry should not be created
        while i == 10 || i != 10 {}
    }
}

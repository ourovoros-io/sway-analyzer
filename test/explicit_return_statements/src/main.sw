contract;

abi TestExplicitReturnStatements {
    fn test_explicit_return_statements_1();
    fn test_explicit_return_statements_2();
    fn test_explicit_return_statements_3() -> u64;
    fn test_explicit_return_statements_4() -> u64;
}

impl TestExplicitReturnStatements for Contract {
    fn test_explicit_return_statements_1() {
        // Report entry should be created:
        // L14: The The `Contract::test_explicit_return_statements_1` function contains an explicit return expression, which is unnecessary. Consider removing `return`.
        return
    }

    fn test_explicit_return_statements_2() {
        // Report entry should be created:
        // L20: The The `Contract::test_explicit_return_statements_2` function contains an explicit return statement, which is unnecessary. Consider removing `return;`.
        return;
    }

    fn test_explicit_return_statements_3() -> u64 {
        // Report entry should be created:
        // L26: The The `Contract::test_explicit_return_statements_3` function contains an explicit return expression, which is unnecessary. Consider replacing `return 0` with `0`.
        return 0
    }

    fn test_explicit_return_statements_4() -> u64 {
        // Report entry should be created:
        // L32: The The `Contract::test_explicit_return_statements_4` function contains an explicit return statement, which is unnecessary. Consider replacing `return 0;` with `0`.
        return 0;
    }
}

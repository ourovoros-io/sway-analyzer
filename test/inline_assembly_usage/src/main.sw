contract;

abi TestInlineAssemblyUsage {
    fn test_inline_assembly_usage(a: u64, b: u64, c: u64) -> (u64, u64);
}

impl TestInlineAssemblyUsage for Contract {
    fn test_inline_assembly_usage(a: u64, b: u64, c: u64) -> (u64, u64) {
        let empty_tuple = (0u64, 0u64);
        // Report entry should be created:
        // L12: The `Contract::test_inline_assembly_usage` function contains inline assembly usage.
        asm(output: empty_tuple, r1: a, r2: b, r3: c, r4, r5) {
            add  r4 r1 r2; // add a & b and put the result in r4
            add  r5 r2 r3; // add b & c and put the result in r5
            sw   output r4 i0; // store the word in r4 in output + 0 words
            sw   output r5 i1; // store the word in r5 in output + 1 word
            output: (u64, u64) // return both values
        }
    }
}

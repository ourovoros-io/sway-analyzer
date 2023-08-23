contract;

abi TestDivisionBeforeMultiplication {
    fn division_before_multiplication();
    fn foo(amount: u64);
    fn foo2(address: Address, amount: u64);
}

// Case 1: In configurables
configurable {
    /// Report entry should be created:
    /// L13: Found division before multiplication in configurable => `C_CONST: u64 = 10 / 10 * 10`. Consider ordering multiplication before division.
    C_CONST: u64 = 10 / 10 * 10,
    /// Report entry should be created:
    /// L16: Found division before multiplication in configurable => `D_CONST: u64 = (20 / 20) * 20`. Consider ordering multiplication before division.
    D_CONST: u64 = (20 / 20) * 20,
    /// Report entry should be created:
    /// L19: Found division before multiplication in configurable => `E_CONST: u64 = (30 + 60) / 30 * 30`. Consider ordering multiplication before division.
    E_CONST: u64 = (30 + 60) / 30 * 30,
    /// Report entry should not be created
    K_CONST: u64 = 10 * 10 / 2,
}

// Case 2: In Constants at contract level
// Report entry should be created:
// L27: Found division before multiplication in constant => `const YX: u64 = 30 / 30 * 30`. Consider ordering multiplication before division.
const YX: u64 = 30 / 30 * 30;
// Report entry should be created:
//  L30: Found division before multiplication in constant => `const XY: u64 = (30 / 30) * 30`. Consider ordering multiplication before division.
const XY: u64 = (30 / 30) * 30;
// Report entry should not be created
const XXX: u64 = 30 * 100 * 2000 / 1000;

impl TestDivisionBeforeMultiplication for Contract {
    fn division_before_multiplication() {
        // Case 3: In let statements
        let a = 5;
        let b = 10;
        let c = 20;
        // Report entry should be created:
        // L42: Found division before multiplication in function : `fn division_before_multiplication()` => `let d = a / b * c;`. Consider ordering multiplication before division.
        let d = a / b * c;
        // Report entry should not be created
        let f = a * b / c;

        let o = 1000 * 1000;
        let p = 1000 / 1000;
        let q = 1000 * 1000 / 1000;
        // Report entry should be created:
        // L51: Found division before multiplication in function : `fn division_before_multiplication()` => `let q1 = (50 / 50) * 50;`. Consider ordering multiplication before division.
        let q1 = (50 / 50) * 50;
        // Report entry should be created:
        // L54: Found division before multiplication in function : `fn division_before_multiplication()` => `let q2 = (50 + 60) / 30 * 80;`. Consider ordering multiplication before division.
        let q2 = (50 + 60) / 30 * 80;

        // Case 4: In function calls
        // Report entry should be created:
        // L59: Found division before multiplication in function : `fn division_before_multiplication()` => `foo(60 / 60 * 60);`. Consider ordering multiplication before division.
        foo(60 / 60 * 60);
        // Report entry should be created:
        // L62: Found division before multiplication in function : `fn division_before_multiplication()` => `foo((70 / 70) * 70);`. Consider ordering multiplication before division.
        foo((70 / 70) * 70);
        // Report entry should not be created:
        foo(1000 * 1000 / 1000);
        foo(1000 / 1000 / 1000);
        foo(1000 * 1000 * 1000);

        // Report entry should be created:
        // L70: Found division before multiplication in function : `fn division_before_multiplication()` => `foo2(Address::zero(), 80 / 80 * 80);`. Consider ordering multiplication before division.
        foo2(Address::zero(), 80 / 80 * 80);
        // Report entry should be created:
        // L73: Found division before multiplication in function : `fn division_before_multiplication()` => `foo2(Address::zero(), (90 / 90) * 90);`. Consider ordering multiplication before division.
        foo2(Address::zero(), (90 / 90) * 90);
        foo2(Address::zero(), 1000 * 1000 / 1000);
        foo2(Address::zero(), 1000 / 1000 / 1000);
        foo2(Address::zero(), 1000 * 1000 * 1000);

        // Case 5: In constants at function level
        // Report entry should be created:
        // L81: Found division before multiplication in function : `fn division_before_multiplication()` => `const Z: u64 = 100 / 100 * 100;`. Consider ordering multiplication before division.
        const Z: u64 = 100 / 100 * 100;
        // Report entry should be created:
        // L84: Found division before multiplication in function : `fn division_before_multiplication()` => `const Z1: u64 = (110 / 110) * 110;`. Consider ordering multiplication before division.
        const Z1: u64 = (110 / 110) * 110;

        let aa = 33;
        let bb = aa / 44;
        // Report entry should be created:
        // L90: Found division before multiplication in function : `fn division_before_multiplication()` => `let cc = bb * 55;`. Consider ordering multiplication before division.
        let cc = bb * 55;

        let aaa = 666;
        let bbb = 777;
        let ccc = 888;
        let ddd = aaa / bbb;
        // Report entry should be created:
        // L98: Found division before multiplication in function : `fn division_before_multiplication()` => `let eee = ddd * ccc;`. Consider ordering multiplication before division.
        let eee = ddd * ccc;


        let division = 60 / 60;
        // Report entry should be created:
        // L104: Found division before multiplication in function : `fn division_before_multiplication()` => `let multiplication = division * 60;`. Consider ordering multiplication before division.
        let multiplication = division * 60;

        // Report entry should not be created
        require(a == b * c + a % b, "Division before multiplication truncated");
    }

    fn foo(amount: u64) {}
    fn foo2(address: Address, amount: u64) {}
}

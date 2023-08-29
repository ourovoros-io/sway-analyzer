contract;

use std::constants::ZERO_B256;

abi TestDivisionBeforeMultiplication {
    fn division_before_multiplication();
}

configurable {
    /// Report entry should be created:
    // L12: Configurable contains a multiplication on the result of a division, which can truncate: `10 / 10 * 10`. Consider refactoring in order to prevent value truncation.
    C_CONST: u64 = 10 / 10 * 10,

    /// Report entry should be created:
    // L16: Configurable contains a multiplication on the result of a division, which can truncate: `(20 / 20) * 20`. Consider refactoring in order to prevent value truncation.
    D_CONST: u64 = (20 / 20) * 20,

    /// Report entry should be created:
    // L20: Configurable contains a multiplication on the result of a division, which can truncate: `(30 + 60) / 30 * 30`. Consider refactoring in order to prevent value truncation.
    E_CONST: u64 = (30 + 60) / 30 * 30,

    /// Report entry should not be created
    K_CONST: u64 = 10 * 10 / 2,
}

// Report entry should be created:
// L28: The `YX` constant contains a multiplication on the result of a division, which can truncate: `30 / 30 * 30`. Consider refactoring in order to prevent value truncation.
pub const YX: u64 = 30 / 30 * 30;

// Report entry should be created:
// L32: The `XY` constant contains a multiplication on the result of a division, which can truncate: `(30 / 30) * 30`. Consider refactoring in order to prevent value truncation.
pub const XY: u64 = (30 / 30) * 30;

// Report entry should not be created
pub const XXX: u64 = 30 * 100 * 2000 / 1000;

fn foo(_amount: u64) {}
fn foo2(_address: Address, _amount: u64) {}

impl TestDivisionBeforeMultiplication for Contract {
    fn division_before_multiplication() {
        let a = 5;
        let b = 10;
        let c = 20;

        // Report entry should be created:
        // L48: The `Contract::division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `a / b * c`. Consider refactoring in order to prevent value truncation.
        let _d = a / b * c;

        // Report entry should not be created
        let _f = a * b / c;

        let _o = 1000 * 1000;
        let _p = 1000 / 1000;
        let _q = 1000 * 1000 / 1000;

        // Report entry should be created:
        // L59: The `Contract::division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `(50 / 50) * 50`. Consider refactoring in order to prevent value truncation.
        let _q1 = (50 / 50) * 50;

        // Report entry should be created:
        // L63: The `Contract::division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `(50 + 60) / 30 * 80`. Consider refactoring in order to prevent value truncation.
        let _q2 = (50 + 60) / 30 * 80;

        // Report entry should be created:
        // L67: The `Contract::division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `60 / 60 * 60`. Consider refactoring in order to prevent value truncation.
        foo(60 / 60 * 60);

        // Report entry should be created:
        // L71: The `Contract::division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `(70 / 70) * 70`. Consider refactoring in order to prevent value truncation.
        foo((70 / 70) * 70);

        // Report entry should not be created:
        foo(1000 * 1000 / 1000);
        foo(1000 / 1000 / 1000);
        foo(1000 * 1000 * 1000);

        // Report entry should be created:
        // L80: The `Contract::division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `80 / 80 * 80`. Consider refactoring in order to prevent value truncation.
        foo2(Address::from(ZERO_B256), 80 / 80 * 80);

        // Report entry should be created:
        // L84: The `Contract::division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `(90 / 90) * 90`. Consider refactoring in order to prevent value truncation.
        foo2(Address::from(ZERO_B256), (90 / 90) * 90);
        foo2(Address::from(ZERO_B256), 1000 * 1000 / 1000);
        foo2(Address::from(ZERO_B256), 1000 / 1000 / 1000);
        foo2(Address::from(ZERO_B256), 1000 * 1000 * 1000);

        // Report entry should be created:
        // L91: The `Z` constant in the `division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `100 / 100 * 100`. Consider refactoring in order to prevent value truncation.
        const Z: u64 = 100 / 100 * 100;

        // Report entry should be created:
        // L95: The `Z1` constant in the `division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `(110 / 110) * 110`. Consider refactoring in order to prevent value truncation.
        const Z1: u64 = (110 / 110) * 110;

        let aa = 33;
        let bb = aa / 44;

        // Report entry should be created:
        // L102: The `Contract::division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `bb * 55`. Consider refactoring in order to prevent value truncation.
        let _cc = bb * 55;

        let aaa = 666;
        let bbb = 777;
        let ccc = 888;
        let ddd = aaa / bbb;

        // Report entry should be created:
        // L111: The `Contract::division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `ddd * ccc`. Consider refactoring in order to prevent value truncation.
        let _eee = ddd * ccc;

        let division = 60 / 60;

        // Report entry should be created:
        // L117: The `Contract::division_before_multiplication` function contains a multiplication on the result of a division, which can truncate: `division * 60`. Consider refactoring in order to prevent value truncation.
        let _multiplication = division * 60;

        // Report entry should not be created
        let z = a / b * c;
        require(z == b * c + a % b, "Division before multiplication truncated");
    }
}

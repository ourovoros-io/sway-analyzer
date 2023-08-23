contract;

/// Case 1: Importing within braces
use std::constants::{BASE_ASSET_ID, ZERO_B256};
/// Case 2: Importing normaly
use std::logging::log;
use std::token::transfer;
use std::call_frames::msg_asset_id;
use std::context::msg_amount;
/// TODO
/// Case 3: Importing prelude
use std::storage::storage_vec::*;


configurable {
    C_CONST: b256 = ZERO_B256,
    D_CONST: ContractId = BASE_ASSET_ID,
    STR: str[6] = "helloo",
}

trait HasValue {
    const VALUE: b256;
}

impl HasValue for Contract {
    const VALUE: b256 = ZERO_B256;
}

abi TestUnusedImports {
    fn redundant_imports();
}

impl TestUnusedImports for Contract {
    fn redundant_imports() {
        let a = BASE_ASSET_ID;
        let b = Identity::Address(Address::from(ZERO_B256));
        log(a);
        log(b);
        transfer(0, a, b);
        require(msg_asset_id() == a, "");
    }
}

contract;

use std::constants::{BASE_ASSET_ID, ZERO_B256};

use std::logging::log;
use std::asset::transfer;

// Report entry should be created:
// L10: Found unused import: `msg_asset_id`. Consider removing any unused imports.
use std::call_frames::msg_asset_id;
// Report entry should be created:
// L13: Found unused import: `msg_amount`. Consider removing any unused imports.
use std::context::msg_amount;

/// TODO Importing prelude
use std::storage::storage_vec::*;


configurable {
    ZERO_ADDRESS: b256 = ZERO_B256,
    NATIVE_ASSET: AssetId = BASE_ASSET_ID,
}

trait HasValue {
    const VALUE: b256;
}

impl HasValue for Contract {
    const VALUE: b256 = ZERO_B256;
}

abi TestUnusedImport {
    fn test_redundant_import();
}

impl TestUnusedImport for Contract {
    fn test_redundant_import() {
        let asset = BASE_ASSET_ID;
        let to = Identity::Address(Address::from(ZERO_B256));
        log(asset);
        log(to);
        transfer(to, asset, 1);
        
    }
}

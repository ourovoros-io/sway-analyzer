contract;

use std::asset::{force_transfer_to_contract, transfer, transfer_to_address};
use std::low_level_call::{call_with_function_selector, CallParams};
use std::bytes::Bytes;

abi TestManipulatableBalanceUsage {
    fn manipulatable_balance_usage_1(to: Identity, asset_id: AssetId, amount: u64);
    fn manipulatable_balance_usage_2(to: Identity, asset_id: AssetId, amount: u64);
    
    fn manipulatable_balance_usage_3(
        to_ident: Identity,
        to_address: Address,
        to_contract: ContractId,
        asset_id: AssetId,
        amount: u64,
        target: ContractId,
        function_selector: Bytes,
        calldata: Bytes,
        call_params: CallParams,
        single_value_type_arg: bool
    );
}

storage {
    balance : u64 = 0, 
    balances: StorageMap<Identity, u64> = StorageMap::<Identity, u64> {},
}

impl TestManipulatableBalanceUsage for Contract {
    fn manipulatable_balance_usage_1(to: Identity, asset_id: AssetId, amount: u64) {
        let balance = storage.balance;
        let out_amount = balance / 2;
        
        // Report entry should be created:
        // L37: The `Contract::manipulatable_balance_usage_1` function contains manipulatable balance usage: `transfer(to, asset_id, out_amount)`
        transfer(to, asset_id, out_amount);
    }

    fn manipulatable_balance_usage_2(to: Identity, asset_id: AssetId, amount: u64) {
        let sender = msg_sender().unwrap();
        let balance = storage.balances.get(sender).try_read().unwrap_or(0);
        let amount_out = balance / 2;
        
        // Report entry should be created:
        // L47: The `Contract::manipulatable_balance_usage_2` function contains manipulatable balance usage: `transfer(to, asset_id, amount_out)`
        transfer(to, asset_id, amount_out);
    }
    
    fn manipulatable_balance_usage_3(
        to_ident: Identity,
        to_address: Address,
        to_contract: ContractId,
        asset_id: AssetId,
        amount: u64,
        target: ContractId,
        function_selector: Bytes,
        calldata: Bytes,
        call_params: CallParams,
        single_value_type_arg: bool
    ) {
        let sender = msg_sender().unwrap();
        let balance = storage.balances.get(sender).try_read().unwrap_or(0);
        let amount_out = balance / 2;
        
        // Report entry should be created:
        // L68: The `Contract::manipulatable_balance_usage_3` function contains manipulatable balance usage: `transfer(to_ident, asset_id, amount)`
        transfer(to_ident, asset_id, amount);
        
        // Report entry should be created:
        // L72: The `Contract::manipulatable_balance_usage_3` function contains manipulatable balance usage: `transfer_to_address(to_address, asset_id, amount)`
        transfer_to_address(to_address, asset_id, amount);
        
        // Report entry should be created:
        // L76: The `Contract::manipulatable_balance_usage_3` function contains manipulatable balance usage: `force_transfer_to_contract(to_contract, asset_id, amount)`
        force_transfer_to_contract(to_contract, asset_id, amount);

        // Report entry should not be created
        call_with_function_selector(target, function_selector, calldata, single_value_type_arg, call_params);
    }
}

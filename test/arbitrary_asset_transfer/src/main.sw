contract;
use std::constants::{BASE_ASSET_ID, ZERO_B256};
use std::token::{force_transfer_to_contract, transfer, transfer_to_address};
use std::low_level_call::{call_with_function_selector, CallParams};
use std::bytes::Bytes;
use std::context::msg_amount;

abi TestArbitraryAssetTransfer {
    #[storage(read, write)]
    fn constructor(admin: Identity);

    #[storage(read, write)]
    fn set_admin(admin: Identity);

    #[storage(read, write)]
    fn set_sender(sender: Identity);

    fn arbitrary_transfer(to: Identity, asset_id: AssetId, amount: u64);

    fn arbitrary_transfer_with_require_and_revert(to: Identity, asset_id: AssetId, amount: u64);

    #[storage(read)]
    fn arbitrary_transfer_protected(to: Identity, asset_id: AssetId, amount: u64);

    #[storage(read)]
    fn arbitraty_transfer_to_sender();
    #[storage(read)]
    fn arbitraty_transfer_to_sender_protected();

    fn transfer_to_msg_sender();
    fn transfer_to_msg_sender_msg_value();

    fn arbitrary_asset_transfer(to_ident: Identity, to_address: Address, to_contract: ContractId, asset_id: AssetId, amount: u64, target: ContractId,
        function_selector: Bytes,
        calldata: Bytes,
        call_params: CallParams,
        single_value_type_arg: bool);
    #[storage(read)]
    fn arbitrary_asset_transfer_protected(to_ident: Identity, to_address: Address, to_contract: ContractId, asset_id: AssetId, amount: u64, target: ContractId,
        function_selector: Bytes,
        calldata: Bytes,
        call_params: CallParams,
        single_value_type_arg: bool);
    #[storage(read)]
    fn arbitrary_asset_transfer_from_sender();
}

storage {
    /// The Identity which has the ability to clawback unclaimed tokens.
    admin: Option<Identity> = Option::None,
    sender: Option<Identity> = Option::None,
}

/// Errors related to permissions.
pub enum AccessError {
    // The caller is not the admin of the contract.
    CallerNotAdmin: (),
}

impl TestArbitraryAssetTransfer for Contract {
    #[storage(read, write)]
    fn constructor(admin: Identity) {
        storage.admin.write(Option::Some(admin));
    }

    #[storage(read, write)]
    fn set_admin(admin: Identity) {
        require(storage.admin.read().is_some() && storage.admin.read().unwrap() == msg_sender().unwrap(), AccessError::CallerNotAdmin);
        storage.admin.write(Option::Some(admin));
    }

    #[storage(read, write)]
    fn set_sender(sender: Identity) {
        storage.sender.write(Option::Some(sender));
    }

    fn arbitrary_transfer(to: Identity, asset_id: AssetId, amount: u64) {
        // Report entry should be created
        // L81: The `Contract::arbitrary_transfer` function contains an arbitrary native asset transfer: `transfer(to, asset_id, amount)`
        transfer(to, asset_id, amount);
    }

    fn arbitrary_transfer_with_require_and_revert(to: Identity, asset_id: AssetId, amount: u64) {
        require(amount > 0, "Error");
        if (amount == 0) {
            revert(0);
        }
        // Report entry should be created
        // L91: The `Contract::arbitrary_transfer_with_require_and_revert` function contains an arbitrary native asset transfer: `transfer(to, asset_id, amount)`
        transfer(to, asset_id, amount);
    }

    #[storage(read)]
    fn arbitrary_transfer_protected(to: Identity, asset_id: AssetId, amount: u64) {
        require(storage.admin.read().is_some() && storage.admin.read().unwrap() == msg_sender().unwrap(), AccessError::CallerNotAdmin);
        // Report entry should not be created
        transfer(to, asset_id, amount);
    }

     #[storage(read)]
    fn arbitraty_transfer_to_sender() {
        let sender = storage.sender.read().unwrap();
        let sender = match sender {
            Identity::Address(sender) => sender,
            _ => { revert(0)}
        };
        // Report entry should be created
        // L110: The `Contract::arbitraty_transfer_to_sender` function contains an arbitrary native asset transfer: `transfer_to_address(sender, BASE_ASSET_ID, 1)`
        transfer_to_address(sender, BASE_ASSET_ID, 1);
    }

     #[storage(read)]
    fn arbitraty_transfer_to_sender_protected() {
        let sender = storage.sender.read().unwrap();
        let sender = match sender {
            Identity::Address(sender) => sender,
            _ => { revert(0)}
        };
        require(storage.admin.read().is_some() && storage.admin.read().unwrap() == msg_sender().unwrap(), AccessError::CallerNotAdmin);
        // Report entry should not be created
        transfer_to_address(sender, BASE_ASSET_ID, 1);
       
    }

    fn transfer_to_msg_sender() {
        let sender = msg_sender().unwrap();
        let sender = match sender {
            Identity::Address(sender) => sender,
            _ => { revert(0)}
        };
        // Report entry should be created
        // L134: The `Contract::transfer_to_msg_sender` function contains an arbitrary native asset transfer: `transfer_to_address(sender, BASE_ASSET_ID, 1)`
        transfer_to_address(sender, BASE_ASSET_ID, 1);
    }

    fn transfer_to_msg_sender_msg_value() {
        let sender = msg_sender().unwrap();
        let sender = match sender {
            Identity::Address(sender) => sender,
            _ => { revert(0)}
        };
        // Report entry should be created
        // L145: The `Contract::transfer_to_msg_sender_msg_value` function contains an arbitrary native asset transfer: `transfer_to_address(sender, BASE_ASSET_ID, msg_amount())`
        transfer_to_address(sender, BASE_ASSET_ID, msg_amount());
    }

    fn arbitrary_asset_transfer(to_ident: Identity, to_address: Address, to_contract: ContractId, asset_id: AssetId, amount: u64, target: ContractId,
        function_selector: Bytes,
        calldata: Bytes,
        call_params: CallParams,
        single_value_type_arg: bool) {
        // Report entry should be created
        // L156: The `Contract::arbitrary_asset_transfer` function contains an arbitrary native asset transfer: `transfer(to_ident, asset_id, amount)`  
        transfer(to_ident, asset_id, amount);
        // L158: The `Contract::arbitrary_asset_transfer` function contains an arbitrary native asset transfer: `transfer_to_address(to_address, asset_id, amount)`
        transfer_to_address(to_address, asset_id, amount);
        // L160: The `Contract::arbitrary_asset_transfer` function contains an arbitrary native asset transfer: `force_transfer_to_contract(to_contract, asset_id, amount)`
        force_transfer_to_contract(to_contract, asset_id, amount);
        // L162: The `Contract::arbitrary_asset_transfer` function contains an arbitrary native asset transfer: `call_with_function_selector(target, function_selector, calldata, single_value_type_arg, call_params)`
        call_with_function_selector(target, function_selector, calldata, single_value_type_arg, call_params);
    }

    #[storage(read)]
    fn arbitrary_asset_transfer_protected(to_ident: Identity, to_address: Address, to_contract: ContractId, asset_id: AssetId, amount: u64, target: ContractId,
        function_selector: Bytes,
        calldata: Bytes,
        call_params: CallParams,
        single_value_type_arg: bool) {
        let admin = storage.admin.read();
        require(admin.is_some() && admin.unwrap() == msg_sender().unwrap(), AccessError::CallerNotAdmin);
        // Report entry should not be created
        transfer(to_ident, asset_id, amount);
        // Report entry should not be created
        transfer_to_address(to_address, asset_id, amount);
        // Report entry should not be created
        force_transfer_to_contract(to_contract, asset_id, amount);
        // Report entry should not be created
        call_with_function_selector(target, function_selector, calldata, single_value_type_arg, call_params);
        
    }    
     
    #[storage(read)]
    fn arbitrary_asset_transfer_from_sender() {
        let sender = storage.sender.read().unwrap();
        match sender {
            Identity::Address => {
                // Report entry should be created
                // L191: The `Contract::arbitrary_asset_transfer_from_sender` function contains an arbitrary native asset transfer: `transfer(sender, BASE_ASSET_ID, 1)`
                transfer(sender, BASE_ASSET_ID, 1);
            },
            _ => { revert(0)}
        }
    }
}

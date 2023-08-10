contract;

use std::constants::ZERO_B256;

abi TestInputIdentityValidation {
    fn test_address_validated(input: Address);
    fn test_address_not_validated(input: Address);

    fn test_contract_id_validated(input: ContractId);
    fn test_contract_id_not_validated(input: ContractId);

    fn test_identity_validated_1(input: Identity);
    fn test_identity_validated_2(input: Identity);
    fn test_identity_validated_3(input: Identity);
    fn test_identity_validated_4(input: Identity);
    fn test_identity_not_validated(input: Identity);
}

impl TestInputIdentityValidation for Contract {
    // Report entry should not be created
    fn test_address_validated(input: Address) {
        require(input != Address::from(ZERO_B256), "Zero address");
        log(input);
    }

    // Report entry should be created:
    // L28: The `Contract::test_address_not_validated` function does not check its `input` parameter for a zero value.
    fn test_address_not_validated(input: Address) {
        log(input);
    }

    // Report entry should not be created
    fn test_contract_id_validated(input: ContractId) {
        require(input != ContractId::from(ZERO_B256), "Zero contract id");
        log(input);
    }

    // Report entry should be created:
    // L40: The `Contract::test_contract_id_not_validated` function does not check its `input` parameter for a zero value.
    fn test_contract_id_not_validated(input: ContractId) {
        log(input);
    }

    // Report entry should not be created
    fn test_identity_validated_1(input: Identity) {
        match input {
            Identity::Address(x) => require(x != Address::from(ZERO_B256), "Zero address"),
            Identity::ContractId(x) => require(x != ContractId::from(ZERO_B256), "Zero contract id"),
        }
        log(input);
    }

    // Report entry should not be created
    fn test_identity_validated_2(input: Identity) {
        if let Identity::Address(x) = input {
            require(x != Address::from(ZERO_B256), "Zero address");
        } else if let Identity::ContractId(x) = input {
            require(x != ContractId::from(ZERO_B256), "Zero contract id");
        }
        log(input);
    }

    // Report entry should not be created
    fn test_identity_validated_3(input: Identity) {
        require(
            match input {
                Identity::Address(x) => x != Address::from(ZERO_B256),
                Identity::ContractId(x) => x != ContractId::from(ZERO_B256),
            },
            "Zero identity"
        );
        log(input);
    }

    // Report entry should not be created
    fn test_identity_validated_4(input: Identity) {
        require(
            if let Identity::Address(x) = input {
                x != Address::from(ZERO_B256)
            } else if let Identity::ContractId(x) = input {
                x != ContractId::from(ZERO_B256)
            } else {
                true
            },
            "Zero identity"
        );
        log(input);
    }

    // Report entry should be created:
    // L92: The `Contract::test_identity_not_validated` function does not check its `input` parameter for a zero value.
    fn test_identity_not_validated(input: Identity) {
        log(input);
    }
}

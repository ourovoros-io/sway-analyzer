contract;

use std::constants::ZERO_B256;

abi TestUnprotectedStorageVariables {
    #[storage(read, write)] fn test_unprotected_storage_variable();

    #[storage(read, write)] fn test_protected_storage_variable_1();
    #[storage(read, write)] fn test_protected_storage_variable_2();
    #[storage(read, write)] fn test_protected_storage_variable_3();
    #[storage(read, write)] fn test_protected_storage_variable_4();
    #[storage(read, write)] fn test_protected_storage_variable_5();
    #[storage(read, write)] fn test_protected_storage_variable_6();
    #[storage(read, write)] fn test_protected_storage_variable_7();
    #[storage(read, write)] fn test_protected_storage_variable_8();
    #[storage(read, write)] fn test_protected_storage_variable_9();
    #[storage(read, write)] fn test_protected_storage_variable_10();
    #[storage(read, write)] fn test_protected_storage_variable_11();
    #[storage(read, write)] fn test_protected_storage_variable_12();
}

storage {
    owner: Identity = Identity::Address(Address::from(ZERO_B256)),
    value: u64 = 0,
}

impl TestUnprotectedStorageVariables for Contract {
    #[storage(read, write)]
    fn test_unprotected_storage_variable() {
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_1() {
        require(msg_sender().unwrap() == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_2() {
        require(storage.owner.read() == msg_sender().unwrap(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_3() {
        if msg_sender().unwrap() != storage.owner.read() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_4() {
        if storage.owner.read() != msg_sender().unwrap() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_5() {
        let sender = msg_sender().unwrap();
        require(sender == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_6() {
        let sender = msg_sender().unwrap();
        require(storage.owner.read() == sender, "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_7() {
        let sender = msg_sender().unwrap();
        if sender != storage.owner.read() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_8() {
        let sender = msg_sender().unwrap();
        if storage.owner.read() != sender {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_9() {
        let sender = match msg_sender() {
            Ok(sender) => sender,
            Err(_) => revert(0),
        };
        require(sender == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_10() {
        let sender = match msg_sender() {
            Ok(sender) => sender,
            Err(_) => revert(0),
        };
        require(storage.owner.read() == sender, "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_11() {
        let sender = match msg_sender() {
            Ok(sender) => sender,
            Err(_) => revert(0),
        };
        if sender != storage.owner.read() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    #[storage(read, write)]
    fn test_protected_storage_variable_12() {
        let sender = match msg_sender() {
            Ok(sender) => sender,
            Err(_) => revert(0),
        };
        if storage.owner.read() != sender {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
}

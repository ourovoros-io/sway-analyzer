contract;

use std::constants::ZERO_B256;
use std::auth::msg_sender as imported_msg_sender;

abi TestUnprotectedStorageVariables {
    #[storage(read, write)] fn test_unprotected_storage_variable_1();
    #[storage(read, write)] fn test_unprotected_storage_variable_2();

    #[storage(read, write)] fn test_protected_storage_variable_1a();
    #[storage(read, write)] fn test_protected_storage_variable_1b();
    #[storage(read, write)] fn test_protected_storage_variable_1c();
    #[storage(read, write)] fn test_protected_storage_variable_2a();
    #[storage(read, write)] fn test_protected_storage_variable_2b();
    #[storage(read, write)] fn test_protected_storage_variable_2c();
    #[storage(read, write)] fn test_protected_storage_variable_3a();
    #[storage(read, write)] fn test_protected_storage_variable_3b();
    #[storage(read, write)] fn test_protected_storage_variable_3c();
    #[storage(read, write)] fn test_protected_storage_variable_4a();
    #[storage(read, write)] fn test_protected_storage_variable_4b();
    #[storage(read, write)] fn test_protected_storage_variable_4c();
    #[storage(read, write)] fn test_protected_storage_variable_5a();
    #[storage(read, write)] fn test_protected_storage_variable_5b();
    #[storage(read, write)] fn test_protected_storage_variable_5c();
    #[storage(read, write)] fn test_protected_storage_variable_6a();
    #[storage(read, write)] fn test_protected_storage_variable_6b();
    #[storage(read, write)] fn test_protected_storage_variable_6c();
    #[storage(read, write)] fn test_protected_storage_variable_7a();
    #[storage(read, write)] fn test_protected_storage_variable_7b();
    #[storage(read, write)] fn test_protected_storage_variable_7c();
    #[storage(read, write)] fn test_protected_storage_variable_8a();
    #[storage(read, write)] fn test_protected_storage_variable_8b();
    #[storage(read, write)] fn test_protected_storage_variable_8c();
    #[storage(read, write)] fn test_protected_storage_variable_9a();
    #[storage(read, write)] fn test_protected_storage_variable_9b();
    #[storage(read, write)] fn test_protected_storage_variable_9c();
    #[storage(read, write)] fn test_protected_storage_variable_10a();
    #[storage(read, write)] fn test_protected_storage_variable_10b();
    #[storage(read, write)] fn test_protected_storage_variable_10c();
    #[storage(read, write)] fn test_protected_storage_variable_11a();
    #[storage(read, write)] fn test_protected_storage_variable_11b();
    #[storage(read, write)] fn test_protected_storage_variable_11c();
    #[storage(read, write)] fn test_protected_storage_variable_12a();
    #[storage(read, write)] fn test_protected_storage_variable_12b();
    #[storage(read, write)] fn test_protected_storage_variable_12c();
    #[storage(read, write)] fn test_protected_storage_variable_13();
    #[storage(read, write)] fn test_protected_storage_variable_14();
}

storage {
    owner: Identity = Identity::Address(Address::from(ZERO_B256)),
    value: u64 = 0,
}

#[storage(read)]
fn only_owner() {
    require(msg_sender().unwrap() == storage.owner.read(), "Only owner");
}

// Report entry should be created:
// L63: The `increment_value_unsafe` function writes to storage without access restriction. Consider checking against `msg_sender()` in order to limit access.
#[storage(read, write)]
fn increment_value_unsafe() {
    let mut value = storage.value.read();
    value += 1;
    storage.value.write(value);
}

impl TestUnprotectedStorageVariables for Contract {
    // Report entry should be created:
    // L73: The `Contract::test_unprotected_storage_variable_1` function writes to storage without access restriction. Consider checking against `msg_sender()` in order to limit access.
    #[storage(read, write)]
    fn test_unprotected_storage_variable_1() {
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }

    // Report entry should be created:
    // L82: The `Contract::test_unprotected_storage_variable_2` function writes to storage without access restriction. Consider checking against `msg_sender()` in order to limit access.
    #[storage(read, write)]
    fn test_unprotected_storage_variable_2() {
        increment_value_unsafe();
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_1a() {
        require(msg_sender().unwrap() == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_1b() {
        require(imported_msg_sender().unwrap() == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_1c() {
        require(std::auth::msg_sender().unwrap() == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_2a() {
        require(storage.owner.read() == msg_sender().unwrap(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_2b() {
        require(storage.owner.read() == imported_msg_sender().unwrap(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_2c() {
        require(storage.owner.read() == std::auth::msg_sender().unwrap(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_3a() {
        if msg_sender().unwrap() != storage.owner.read() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_3b() {
        if imported_msg_sender().unwrap() != storage.owner.read() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_3c() {
        if std::auth::msg_sender().unwrap() != storage.owner.read() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_4a() {
        if storage.owner.read() != msg_sender().unwrap() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_4b() {
        if storage.owner.read() != imported_msg_sender().unwrap() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_4c() {
        if storage.owner.read() != std::auth::msg_sender().unwrap() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_5a() {
        let sender = msg_sender().unwrap();
        require(sender == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_5b() {
        let sender = imported_msg_sender().unwrap();
        require(sender == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_5c() {
        let sender = std::auth::msg_sender().unwrap();
        require(sender == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_6a() {
        let sender = msg_sender().unwrap();
        require(storage.owner.read() == sender, "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_6b() {
        let sender = imported_msg_sender().unwrap();
        require(storage.owner.read() == sender, "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_6c() {
        let sender = std::auth::msg_sender().unwrap();
        require(storage.owner.read() == sender, "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_7a() {
        let sender = msg_sender().unwrap();
        if sender != storage.owner.read() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_7b() {
        let sender = imported_msg_sender().unwrap();
        if sender != storage.owner.read() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_7c() {
        let sender = std::auth::msg_sender().unwrap();
        if sender != storage.owner.read() {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_8a() {
        let sender = msg_sender().unwrap();
        if storage.owner.read() != sender {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_8b() {
        let sender = imported_msg_sender().unwrap();
        if storage.owner.read() != sender {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_8c() {
        let sender = std::auth::msg_sender().unwrap();
        if storage.owner.read() != sender {
            revert(0);
        }
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_9a() {
        let sender = match msg_sender() {
            Ok(sender) => sender,
            Err(_) => revert(0),
        };
        require(sender == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_9b() {
        let sender = match imported_msg_sender() {
            Ok(sender) => sender,
            Err(_) => revert(0),
        };
        require(sender == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_9c() {
        let sender = match std::auth::msg_sender() {
            Ok(sender) => sender,
            Err(_) => revert(0),
        };
        require(sender == storage.owner.read(), "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_10a() {
        let sender = match msg_sender() {
            Ok(sender) => sender,
            Err(_) => revert(0),
        };
        require(storage.owner.read() == sender, "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_10b() {
        let sender = match imported_msg_sender() {
            Ok(sender) => sender,
            Err(_) => revert(0),
        };
        require(storage.owner.read() == sender, "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_10c() {
        let sender = match std::auth::msg_sender() {
            Ok(sender) => sender,
            Err(_) => revert(0),
        };
        require(storage.owner.read() == sender, "Only owner");
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_11a() {
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
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_11b() {
        let sender = match imported_msg_sender() {
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
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_11c() {
        let sender = match std::auth::msg_sender() {
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
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_12a() {
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
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_12b() {
        let sender = match imported_msg_sender() {
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
    
    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_12c() {
        let sender = match std::auth::msg_sender() {
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

    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_13() {
        only_owner();
        let mut value = storage.value.read();
        value += 1;
        storage.value.write(value);
    }

    // Report entry should not be created
    #[storage(read, write)]
    fn test_protected_storage_variable_14() {
        only_owner();
        increment_value_unsafe();
    }
}

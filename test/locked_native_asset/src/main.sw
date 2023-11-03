contract;

abi TestLockedNativeAsset {
    #[payable, storage(read, write)]
    fn deposit();
    #[payable, storage(read, write)]
    fn deposit2();

    fn withdraw();
}

storage {
    balance: u64 = 0;
}

impl TestLockedNativeAsset for Contract {
    // Report entry should be created
    // L20: The `Contract::deposit` function will lock native assets. Consider adding a withdraw function.    
    #[payable, storage(read, write)]
    fn deposit() {
        assert(msg_amount() > 0);
        storage.balance = storage.balance + msg_amount();
    }

    // Report entry should be created
    // L28: The `Contract::deposit2` function will lock native assets. Consider adding a withdraw function.   
    #[payable, storage(read, write)]
    fn deposit2() {
        assert(msg_amount() > 0);
        storage.balance = storage.balance + msg_amount();
    }

    fn withdraw() {
        // Uncomment will cause no entries to be produced
        // let out = (0u64);
        // asm(output: out, r1: a, r2: b, r3: c, r4: d) {
        //     call r1 r2 r3 r4;
        //     output: (u64)
        // }
        // transfer(msg_sender(), storage.balance);
        // transfer_to_address(msg_sender(), storage.balance);
    }
}

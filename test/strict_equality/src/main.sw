contract;

/// contract Crowdsale{
// function fund_reached() public returns(bool){
//     return this.balance == 100 ether;
// }

abi TestStrictEquality {
    #[storage(read)]
    fn fund_reached() -> bool;
    #[storage(read)]
    fn let_fund_reached() -> bool;
    fn happy_path() -> bool;
}

storage {
    /// The Identity which has the ability to clawback unclaimed tokens.
    balance: u64 = 0,
}



impl TestStrictEquality for Contract {
    #[storage(read)]
    fn fund_reached() -> bool {
        // Report entry should be created:
        // L28: The `Contract::fund_reached` function contains a strict equality check: `storage.balance.read() == 100`.Don't use strict equality to determine if an account has enough balance.
        return storage.balance.read() == 100;
    }

    #[storage(read)]
    fn let_fund_reached() -> bool {
        let a = storage.balance.read();
        // Report entry should be created:
        // L36: The `Contract::let_fund_reached` function contains a strict equality check: `a == 100`.Don't use strict equality to determine if an account has enough balance.
        return  a == 100;
    }
    
    fn happy_path() -> bool {
        let a = 20;
        let b = 2000;
        // Report entry should not be created:
        if(a == 200) || (2000 == b) {
            return true;
        } else {
            return false;
        }
    }
}

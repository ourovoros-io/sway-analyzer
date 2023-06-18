contract;

abi TestContract {
    #[storage(write)]
    fn initialize_counter(value: u64) -> u64;

    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64;

    #[storage(read, write)]
    fn increment_counter_map(amount: u64) -> u64;
}

struct Counter {
    value: u64,
}

storage {
    counter: Counter = Counter {
        value: 0,
    },

    counter_map: StorageMap<(), Counter> = StorageMap {},
}

impl TestContract for Contract {
    #[storage(write)]
    fn initialize_counter(value: u64) -> u64 {
        storage.counter.value.write(value);
        value
    }

    #[storage(read, write)]
    fn increment_counter(amount: u64) -> u64 {
        let mut counter = storage.counter.read();
        counter.value += amount;
        // storage.counter.write(counter);
        counter.value
    }

    #[storage(read, write)]
    fn increment_counter_map(amount: u64) -> u64 {
        let mut counter = storage.counter_map.get(()).read();
        counter.value += amount;
        // storage.counter_map.insert((), counter);
        
        let mut counter = storage.counter_map.get(()).read();
        counter.value += amount * 2;
        storage.counter_map.insert((), counter);

        counter.value
    }
}

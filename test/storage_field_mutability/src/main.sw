contract;

use std::hash::Hash;

abi TestStorageFieldMutability {
    #[storage(write)]
    fn initialize();

    #[storage(read)]
    fn get_value1() -> u64;

    #[storage(read)]
    fn get_value2() -> u64;

    #[storage(read)]
    fn get_from_map1(x: u64) -> u64;

    #[storage(read)]
    fn get_from_map2(x: u64) -> u64;

    #[storage(read)]
    fn get_from_map3(x: u64, y: u64) -> u64;

    #[storage(read)]
    fn get_from_map4(x: u64, y: u64) -> u64;
}

storage {
    // Report entry should not be created
    value1: u64 = 0,

    // Report entry should be created:
    // L32: The `value2` storage field is never mutated. Consider refactoring it into a constant or a configurable field.
    value2: u64 = 0,

    // Report entry should not be created
    map1: StorageMap<u64, u64> = StorageMap {},

    // Report entry should be created:
    // L39: The `map2` storage field is never mutated. Consider refactoring it into a constant or a configurable field.
    map2: StorageMap<u64, u64> = StorageMap {},

    // Report entry should not be created
    map3: StorageMap<u64, StorageMap<u64, u64>> = StorageMap {},

    // Report entry should be created:
    // L46: The `map4` storage field is never mutated. Consider refactoring it into a constant or a configurable field.
    map4: StorageMap<u64, StorageMap<u64, u64>> = StorageMap {},
}

impl TestStorageFieldMutability for Contract {
    #[storage(write)]
    fn initialize() {
        storage.value1.write(1);
        storage.map1.insert(0, 0);
        storage.map3.insert(0, StorageMap {});
    }

    #[storage(read)]
    fn get_value1() -> u64 {
        storage.value1.read()
    }

    #[storage(read)]
    fn get_value2() -> u64 {
        storage.value2.read()
    }

    #[storage(read)]
    fn get_from_map1(x: u64) -> u64 {
        storage.map1.get(x).read()
    }

    #[storage(read)]
    fn get_from_map2(x: u64) -> u64 {
        storage.map2.get(x).read()
    }

    #[storage(read)]
    fn get_from_map3(x: u64, y: u64) -> u64 {
        storage.map3.get(x).get(y).read()
    }

    #[storage(read)]
    fn get_from_map4(x: u64, y: u64) -> u64 {
        storage.map4.get(x).get(y).read()
    }
}

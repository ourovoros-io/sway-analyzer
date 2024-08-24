contract;

use std::bytes::Bytes;
use std::low_level_call::CallParams;

abi TestUncheckedCallPayload {
    fn test_raw_ptr_payload(payload: raw_ptr, call_params: CallParams);
    fn test_unchecked_bytes_payload(payload: Bytes, call_params: CallParams);
    fn test_checked_bytes_payload_1(payload: Bytes, call_params: CallParams);
    fn test_checked_bytes_payload_2(payload: Bytes, call_params: CallParams);
    fn test_checked_bytes_payload_3(payload: Bytes, call_params: CallParams);
    fn test_checked_bytes_payload_4(payload: Bytes, call_params: CallParams);
}

impl TestUncheckedCallPayload for Contract {
    fn test_raw_ptr_payload(payload: raw_ptr, call_params: CallParams) {
        asm(r1: payload, r2: call_params.coins, r3: call_params.asset_id, r4: call_params.gas) {
            // Report entry should be created:
            // L20: The `Contract::test_raw_ptr_payload` function uses the `payload: raw_ptr` parameter as the payload in a `CALL` instruction via register `r1`, which may revert if the data is incorrect: `call r1 r2 r3 r4`
            call r1 r2 r3 r4;
        };
    }

    fn test_unchecked_bytes_payload(payload: Bytes, call_params: CallParams) {
        asm(r1: payload.ptr(), r2: call_params.coins, r3: call_params.asset_id, r4: call_params.gas) {
            // Report entry should be created:
            // L28: The `Contract::test_unchecked_bytes_payload` function uses the `payload: Bytes` parameter as the payload in a `CALL` instruction via register `r1` without checking its length, which may revert if the data is incorrect: `call r1 r2 r3 r4`
            call r1 r2 r3 r4;
        };
    }
    
    fn test_checked_bytes_payload_1(payload: Bytes, call_params: CallParams) {
        require(payload.len() >= 32, "Invalid payload");
        asm(r1: payload.ptr(), r2: call_params.coins, r3: call_params.asset_id, r4: call_params.gas) {
            // Report entry should not be created
            call r1 r2 r3 r4;
        };
    }
    
    fn test_checked_bytes_payload_2(payload: Bytes, call_params: CallParams) {
        require(32 <= payload.len(), "Invalid payload");
        asm(r1: payload.ptr(), r2: call_params.coins, r3: call_params.asset_id, r4: call_params.gas) {
            // Report entry should not be created
            call r1 r2 r3 r4;
        };
    }
    
    fn test_checked_bytes_payload_3(payload: Bytes, call_params: CallParams) {
        if payload.len() < 32 {
            revert(0);
        }
        asm(r1: payload.ptr(), r2: call_params.coins, r3: call_params.asset_id, r4: call_params.gas) {
            // Report entry should not be created
            call r1 r2 r3 r4;
        };
    }
    
    fn test_checked_bytes_payload_4(payload: Bytes, call_params: CallParams) {
        if 32 > payload.len() {
            revert(0);
        }
        asm(r1: payload.ptr(), r2: call_params.coins, r3: call_params.asset_id, r4: call_params.gas) {
            // Report entry should not be created
            call r1 r2 r3 r4;
        };
    }
}

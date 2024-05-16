extern "C" {
    /**
     * @brief Create TVM emulator
     * @param code_boc Base64 encoded BoC serialized smart contract code cell
     * @param data_boc Base64 encoded BoC serialized smart contract data cell
     * @param vm_log_verbosity Verbosity level of VM log
     * @return Pointer to TVM emulator object
     */
    pub fn tvm_emulator_create(
        code: *const ::std::os::raw::c_char,
        data: *const ::std::os::raw::c_char,
        vm_log_verbosity: u32,
    ) -> *mut ::std::os::raw::c_void;

    /**
     * @brief Set libraries for TVM emulator
     * @param libs_boc Base64 encoded BoC serialized libraries dictionary (HashmapE 256 ^Cell).
     * @return true in case of success, false in case of error
     */
    pub fn tvm_emulator_set_libraries(
        tvm_emulator: *mut ::std::os::raw::c_void,
        libs_boc: *const ::std::os::raw::c_char,
    ) -> bool;

    /**
     * @brief Set c7 parameters
     * @param tvm_emulator Pointer to TVM emulator
     * @param address Adress of smart contract
     * @param unixtime Unix timestamp
     * @param balance Smart contract balance
     * @param rand_seed_hex Random seed as hex string of length 64
     * @param config Base64 encoded BoC serialized Config dictionary (Hashmap 32 ^Cell)
     * @return true in case of success, false in case of error
     */
    pub fn tvm_emulator_set_c7(
        tvm_emulator: *mut ::std::os::raw::c_void,
        address: *const ::std::os::raw::c_char,
        unixtime: u32,
        balance: u64,
        rand_seed_hex: *const ::std::os::raw::c_char,
        config: *const ::std::os::raw::c_char,
    ) -> bool;

    /**
     * @brief Set TVM gas limit
     * @param tvm_emulator Pointer to TVM emulator
     * @param gas_limit Gas limit
     * @return true in case of success, false in case of error
     */
    pub fn tvm_emulator_set_gas_limit(
        tvm_emulator: *mut ::std::os::raw::c_void,
        gas_limit: u64,
    ) -> bool;

    /**
     * @brief Enable or disable TVM debug primitives
     * @param tvm_emulator Pointer to TVM emulator
     * @param debug_enabled Whether debug primitives should be enabled or not
     * @return true in case of success, false in case of error
     */
    pub fn tvm_emulator_set_debug_enabled(
        tvm_emulator: *mut ::std::os::raw::c_void,
        debug_enabled: ::std::os::raw::c_int,
    ) -> bool;

    /**
     * @brief Run get method
     * @param tvm_emulator Pointer to TVM emulator
     * @param method_id Integer method id
     * @param stack_boc Base64 encoded BoC serialized stack (VmStack)
     * @return Json object with error:
     * {
     *   "success": false,
     *   "error": "Error description"
     * }
     * Or success:
     * {
     *   "success": true
     *   "vm_log": "...",
     *   "vm_exit_code": 0,
     *   "stack": "Base64 encoded BoC serialized stack (VmStack)",
     *   "missing_library": null,
     *   "gas_used": 1212
     * }
     */
    pub fn tvm_emulator_run_get_method(
        tvm_emulator: *mut ::std::os::raw::c_void,
        method_id: i32,
        stack_boc: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;

    /**
     * @brief Send external message
     * @param tvm_emulator Pointer to TVM emulator
     * @param message_body_boc Base64 encoded BoC serialized message body cell.
     * @return Json object with error:
     * {
     *   "success": false,
     *   "error": "Error description"
     * }
     * Or success:
     * {
     *   "success": true,
     *   "new_code": "Base64 boc decoded new code cell",
     *   "new_data": "Base64 boc decoded new data cell",
     *   "accepted": true,
     *   "vm_exit_code": 0,
     *   "vm_log": "...",
     *   "missing_library": null,
     *   "gas_used": 1212,
     *   "actions": "Base64 boc decoded actions cell of type (OutList n)"
     * }
     */
    pub fn tvm_emulator_send_external_message(
        tvm_emulator: *mut ::std::os::raw::c_void,
        message_body_boc: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;

    /**
     * @brief Send internal message
     * @param tvm_emulator Pointer to TVM emulator
     * @param message_body_boc Base64 encoded BoC serialized message body cell.
     * @param amount Amount of nanograms attached with internal message.
     * @return Json object with error:
     * {
     *   "success": false,
     *   "error": "Error description"
     * }
     * Or success:
     * {
     *   "success": true,
     *   "new_code": "Base64 boc decoded new code cell",
     *   "new_data": "Base64 boc decoded new data cell",
     *   "accepted": true,
     *   "vm_exit_code": 0,
     *   "vm_log": "...",
     *   "missing_library": null,
     *   "gas_used": 1212,
     *   "actions": "Base64 boc decoded actions cell of type (OutList n)"
     * }
     */
    pub fn tvm_emulator_send_internal_message(
        tvm_emulator: *mut ::std::os::raw::c_void,
        message_body_boc: *const ::std::os::raw::c_char,
        amount: u64,
    ) -> *const ::std::os::raw::c_char;

    /**
     * @brief Destroy TVM emulator object
     * @param tvm_emulator Pointer to TVM emulator object
     */
    pub fn tvm_emulator_destroy(tvm_emulator: *mut ::std::os::raw::c_void);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_tvm_emulator() {
        let code = "te6cckECCwEAAe0AART/APSkE/S88sgLAQIBYgIDAgLMBAUCA3pgCQoD79mRDjgEit8GhpgYC42Eit8H0gGADpj+mf9qJofQB9IGpqGEAKqThdRxgamqiq44L5cCSA/SB9AGoYEGhAMGuQ/QAYEogaKCF4BFAqkGQoAn0BLGeLZmZk9qpwQQg97svvKThdcYEakuAB8YEYAmACcYEvgsIH+XhAYHCACT38FCIBuCoQCaoKAeQoAn0BLGeLAOeLZmSRZGWAiXoAegBlgGSQfIA4OmRlgWUD5f/k6DvADGRlgqxniygCfQEJ5bWJZmZkuP2AQA/jYD+gD6QPgoVBIIcFQgE1QUA8hQBPoCWM8WAc8WzMkiyMsBEvQA9ADLAMn5AHB0yMsCygfL/8nQUAjHBfLgShKhA1AkyFAE+gJYzxbMzMntVAH6QDAg1wsBwwCOH4IQ1TJ223CAEMjLBVADzxYi+gISy2rLH8s/yYBC+wCRW+IAMDUVxwXy4En6QDBZyFAE+gJYzxbMzMntVAAuUUPHBfLgSdQwAchQBPoCWM8WzMzJ7VQAfa289qJofQB9IGpqGDYY/BQAuCoQCaoKAeQoAn0BLGeLAOeLZmSRZGWAiXoAegBlgGT8gDg6ZGWBZQPl/+ToQAAfrxb2omh9AH0gamoYP6qQQFEAfwk=\0";

        let data = "te6cckECFAEAA3wAAlFwOPUE4QoACAG/b+7lv/B/MjjfQ11sWK3b4LOpS7Bc7BSmJBVmyz5hdQECAEoBaHR0cHM6Ly90YXJhbnRpbmkuZGV2L3N0b24vbW9vbi5qc29uART/APSkE/S88sgLAwIBYgQFAgLMBgcAG6D2BdqJofQB9IH0gahhAgHUCAkCAUgKCwC7CDHAJJfBOAB0NMDAXGwlRNfA/AL4PpA+kAx+gAxcdch+gAx+gAwAtMfghAPin6lUiC6lTE0WfAI4IIQF41FGVIgupYxREQD8AngNYIQWV8HvLqTWfAK4F8EhA/y8IAARPpEMHC68uFNgAgEgDA0CASASEwH1APTP/oA+kAh8AHtRND6APpA+kDUMFE2oVIqxwXy4sEowv/y4sJUNEJwVCATVBQDyFAE+gJYzxYBzxbMySLIywES9AD0AMsAySD5AHB0yMsCygfL/8nQBPpA9AQx+gB3gBjIywVQCM8WcPoCF8trE8yCEBeNRRnIyx8ZgDgP3O1E0PoA+kD6QNQwCNM/+gBRUaAF+kD6QFNbxwVUc21wVCATVBQDyFAE+gJYzxYBzxbMySLIywES9AD0AMsAyfkAcHTIywLKB8v/ydBQDccFHLHy4sMK+gBRqKGCCJiWgIIImJaAErYIoYIImJaAoBihJ+MPJdcLAcMAI4A8QEQCayz9QB/oCIs8WUAbPFiX6AlADzxbJUAXMI5FykXHiUAioE6CCCJiWgKoAggiYloCgoBS88uLFBMmAQPsAECPIUAT6AljPFgHPFszJ7VQAcFJ5oBihghBzYtCcyMsfUjDLP1j6AlAHzxZQB88WyXGAGMjLBSTPFlAG+gIVy2oUzMlx+wAQJBAjAA4QSRA4N18EAHbCALCOIYIQ1TJ223CAEMjLBVAIzxZQBPoCFstqEssfEss/yXL7AJM1bCHiA8hQBPoCWM8WAc8WzMntVADbO1E0PoA+kD6QNQwB9M/+gD6QDBRUaFSSccF8uLBJ8L/8uLCggiYloCqABagFrzy4sOCEHvdl97Iyx8Vyz9QA/oCIs8WAc8WyXGAGMjLBSTPFnD6AstqzMmAQPsAQBPIUAT6AljPFgHPFszJ7VSAAgyAINch7UTQ+gD6QPpA1DAE0x+CEBeNRRlSILqCEHvdl94TuhKx8uLF0z8x+gAwE6BQI8hQBPoCWM8WAc8WzMntVIH++ZZY=\0";

        let code_slice = code.as_bytes();
        let data_slice = data.as_bytes();
        let code_packed = code_slice.as_ptr();
        let data_packed = data_slice.as_ptr();

        unsafe {
            let emulator =
                tvm_emulator_create(code_packed as *const i8, data_packed as *const i8, 2);
            tvm_emulator_run_get_method(emulator, 11111123, data_packed as *const i8);
            assert!(!emulator.is_null());
            tvm_emulator_destroy(emulator);
        }
    }
}

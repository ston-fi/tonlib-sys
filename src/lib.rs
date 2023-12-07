mod tonlibjson;
mod tvm_emulator;

pub use tonlibjson::*;
pub use tvm_emulator::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_client() {
        unsafe {
            let client = tonlib_client_json_create();
            tonlib_client_set_verbosity_level(4);
            assert!(!client.is_null());
            tonlib_client_json_send(client, "123\0".as_bytes().as_ptr() as *const i8);
            tonlib_client_json_receive(client, 1.0);
            tonlib_client_json_destroy(client);
        }
    }

    #[test]
    fn it_creates_emulator() {
        let code =  "te6cckECCwEAAe0AART/APSkE/S88sgLAQIBYgIDAgLMBAUCA3pgCQoD79mRDjgEit8GhpgYC42Eit8H0gGADpj+mf9qJofQB9IGpqGEAKqThdRxgamqiq44L5cCSA/SB9AGoYEGhAMGuQ/QAYEogaKCF4BFAqkGQoAn0BLGeLZmZk9qpwQQg97svvKThdcYEakuAB8YEYAmACcYEvgsIH+XhAYHCACT38FCIBuCoQCaoKAeQoAn0BLGeLAOeLZmSRZGWAiXoAegBlgGSQfIA4OmRlgWUD5f/k6DvADGRlgqxniygCfQEJ5bWJZmZkuP2AQA/jYD+gD6QPgoVBIIcFQgE1QUA8hQBPoCWM8WAc8WzMkiyMsBEvQA9ADLAMn5AHB0yMsCygfL/8nQUAjHBfLgShKhA1AkyFAE+gJYzxbMzMntVAH6QDAg1wsBwwCOH4IQ1TJ223CAEMjLBVADzxYi+gISy2rLH8s/yYBC+wCRW+IAMDUVxwXy4En6QDBZyFAE+gJYzxbMzMntVAAuUUPHBfLgSdQwAchQBPoCWM8WzMzJ7VQAfa289qJofQB9IGpqGDYY/BQAuCoQCaoKAeQoAn0BLGeLAOeLZmSRZGWAiXoAegBlgGT8gDg6ZGWBZQPl/+ToQAAfrxb2omh9AH0gamoYP6qQQFEAfwk=\0";

        let data =  "te6cckECFAEAA3wAAlFwOPUE4QoACAG/b+7lv/B/MjjfQ11sWK3b4LOpS7Bc7BSmJBVmyz5hdQECAEoBaHR0cHM6Ly90YXJhbnRpbmkuZGV2L3N0b24vbW9vbi5qc29uART/APSkE/S88sgLAwIBYgQFAgLMBgcAG6D2BdqJofQB9IH0gahhAgHUCAkCAUgKCwC7CDHAJJfBOAB0NMDAXGwlRNfA/AL4PpA+kAx+gAxcdch+gAx+gAwAtMfghAPin6lUiC6lTE0WfAI4IIQF41FGVIgupYxREQD8AngNYIQWV8HvLqTWfAK4F8EhA/y8IAARPpEMHC68uFNgAgEgDA0CASASEwH1APTP/oA+kAh8AHtRND6APpA+kDUMFE2oVIqxwXy4sEowv/y4sJUNEJwVCATVBQDyFAE+gJYzxYBzxbMySLIywES9AD0AMsAySD5AHB0yMsCygfL/8nQBPpA9AQx+gB3gBjIywVQCM8WcPoCF8trE8yCEBeNRRnIyx8ZgDgP3O1E0PoA+kD6QNQwCNM/+gBRUaAF+kD6QFNbxwVUc21wVCATVBQDyFAE+gJYzxYBzxbMySLIywES9AD0AMsAyfkAcHTIywLKB8v/ydBQDccFHLHy4sMK+gBRqKGCCJiWgIIImJaAErYIoYIImJaAoBihJ+MPJdcLAcMAI4A8QEQCayz9QB/oCIs8WUAbPFiX6AlADzxbJUAXMI5FykXHiUAioE6CCCJiWgKoAggiYloCgoBS88uLFBMmAQPsAECPIUAT6AljPFgHPFszJ7VQAcFJ5oBihghBzYtCcyMsfUjDLP1j6AlAHzxZQB88WyXGAGMjLBSTPFlAG+gIVy2oUzMlx+wAQJBAjAA4QSRA4N18EAHbCALCOIYIQ1TJ223CAEMjLBVAIzxZQBPoCFstqEssfEss/yXL7AJM1bCHiA8hQBPoCWM8WAc8WzMntVADbO1E0PoA+kD6QNQwB9M/+gD6QDBRUaFSSccF8uLBJ8L/8uLCggiYloCqABagFrzy4sOCEHvdl97Iyx8Vyz9QA/oCIs8WAc8WyXGAGMjLBSTPFnD6AstqzMmAQPsAQBPIUAT6AljPFgHPFszJ7VSAAgyAINch7UTQ+gD6QPpA1DAE0x+CEBeNRRlSILqCEHvdl94TuhKx8uLF0z8x+gAwE6BQI8hQBPoCWM8WAc8WzMntVIH++ZZY=\0";

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

extern "C" {
    pub fn tonlib_client_json_create() -> *mut std::os::raw::c_void;

    pub fn tonlib_client_json_send(
        client: *mut std::os::raw::c_void,
        request: *const std::os::raw::c_char,
    );

    pub fn tonlib_client_json_receive(
        client: *mut std::os::raw::c_void,
        timeout: f64,
    ) -> *const std::os::raw::c_char;

    pub fn tonlib_client_json_execute(
        client: *mut std::os::raw::c_void,
        request: *const std::os::raw::c_char,
    ) -> *const std::os::raw::c_char;

    /// Cancels all outstanding requests for a tonlib JSON client.
    ///
    /// # Safety
    ///
    /// `client` must be a valid, live pointer returned by [`tonlib_client_json_create`].
    pub fn tonlib_client_json_cancel_requests(client: *mut std::os::raw::c_void);

    pub fn tonlib_client_json_destroy(client: *mut std::os::raw::c_void);

    pub fn tonlib_client_set_verbosity_level(verbosity_level: u32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_all_tonlib_json_exports() -> Result<(), Box<dyn Error>> {
        let sync_request = CString::new(r#"{"@type":"getBip39Hints","prefix":"aban"}"#)?;
        let async_request = CString::new(r#"{"@type":"getBip39Hints","prefix":"ab"}"#)?;

        unsafe {
            let client = tonlib_client_json_create();
            assert!(!client.is_null());

            tonlib_client_set_verbosity_level(0);

            let response = tonlib_client_json_execute(client, sync_request.as_ptr());
            assert!(!response.is_null());
            let response = CStr::from_ptr(response).to_str()?;
            assert!(response.contains(r#""@type":"bip39Hints""#));
            assert!(response.contains("abandon"));

            tonlib_client_json_send(client, async_request.as_ptr());
            let response = tonlib_client_json_receive(client, 1.0);
            assert!(!response.is_null());
            assert!(CStr::from_ptr(response)
                .to_str()?
                .contains(r#""@type":"bip39Hints""#));

            tonlib_client_json_cancel_requests(client);
            tonlib_client_json_destroy(client);
        }

        Ok(())
    }
}

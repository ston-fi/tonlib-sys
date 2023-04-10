extern "C" {
    pub fn tonlib_client_json_create() -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn tonlib_client_json_send(
        client: *mut ::std::os::raw::c_void,
        request: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn tonlib_client_json_receive(
        client: *mut ::std::os::raw::c_void,
        timeout: f64,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn tonlib_client_json_execute(
        client: *mut ::std::os::raw::c_void,
        request: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn tonlib_client_json_destroy(client: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn tonlib_client_set_verbosity_level(verbosity_level: u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_client() {
        unsafe {
            let client = tonlib_client_json_create();
            assert!(!client.is_null());
            tonlib_client_json_destroy(client);
        }
    }
}

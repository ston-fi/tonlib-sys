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

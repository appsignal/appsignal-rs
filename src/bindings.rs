#[allow(dead_code)]
extern "C" {
    pub fn appsignal_start() -> ();
    pub fn appsignal_stop() -> ();
    pub fn appsignal_start_transaction(arg1: *const ::libc::c_char,
                                       arg2: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn appsignal_start_event(arg1: ::libc::c_int) -> ();
    pub fn appsignal_finish_event(arg1: ::libc::c_int,
                                  arg2: *const ::libc::c_char,
                                  arg3: *const ::libc::c_char,
                                  arg4: *const ::libc::c_char,
                                  arg5: ::libc::c_int) -> ();
    pub fn appsignal_set_transaction_error(arg1: ::libc::c_int,
                                           arg2: *const ::libc::c_char,
                                           arg3: *const ::libc::c_char,
                                           arg4: *const ::libc::c_char) -> ();
    pub fn appsignal_set_transaction_sample_data(arg1: ::libc::c_int,
                                                 arg2: *const ::libc::c_char,
                                                 arg3: *const ::libc::c_char)
     -> ();
    pub fn appsignal_set_transaction_action(arg1: ::libc::c_int,
                                            arg2: *const ::libc::c_char) -> ();
    pub fn appsignal_set_transaction_queue_start(arg1: ::libc::c_int,
                                                 arg2: ::libc::c_long) -> ();
    pub fn appsignal_set_transaction_metadata(arg1: ::libc::c_int,
                                              arg2: *const ::libc::c_char,
                                              arg3: *const ::libc::c_char)
     -> ();
    pub fn appsignal_finish_transaction(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn appsignal_complete_transaction(arg1: ::libc::c_int) -> ();
    pub fn appsignal_set_gauge(arg1: *const ::libc::c_char,
                               arg2: ::libc::c_float) -> ();
    pub fn appsignal_increment_counter(arg1: *const ::libc::c_char,
                                       arg2: ::libc::c_int) -> ();
    pub fn appsignal_add_distribution_value(arg1: *const ::libc::c_char,
                                            arg2: ::libc::c_float) -> ();
    pub fn appsignal_track_allocation() -> ();
    pub fn appsignal_track_gc_start() -> ();
    pub fn appsignal_track_gc_end() -> ();
}

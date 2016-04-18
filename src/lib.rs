#![feature(panic_handler)]

extern crate backtrace;
extern crate libc;
extern crate rustc_serialize;

use std::env;
use std::ffi::CString;
use std::fmt::Display;
use std::panic::PanicInfo;

use rustc_serialize::{Encodable, json};
use backtrace::Symbol;

mod bindings;

static AGENT_VERSION: &'static str = "4201306"; // Also present in build.rs

/// Starts appsignal agent and extension. AppSignal needs to be configured
/// through environment variables:
///
/// * `APPSIGNAL_ACTIVE`: extension starts if value is true
/// * `APPSIGNAL_AGENT_PATH`: Path to directory of agent executable
/// * `APPSIGNAL_APP_PATH`: path to the app we're monitoring
/// * `APPSIGNAL_LOG_PATH`: Path to write logs to
/// * `APPSIGNAL_PUSH_API_ENDPOINT`: Endpoint to post data to (https://push.appsignal.com)
/// * `APPSIGNAL_PUSH_API_KEY`: Key you get in the installation wizard on https://appsignal.com
/// * `APPSIGNAL_APP_NAME`: Name of the application we're monitoring
/// * `APPSIGNAL_ENVIRONMENT`: Environment we're monitoring (staging, production)
/// * `APPSIGNAL_AGENT_VERSION`: Version of the agent we're running
/// * `APPSIGNAL_TRANSMISSION_INTERVAL`: Optional, amount of time between transmissions. Default is 30
/// * `APPSIGNAL_WORKING_DIR_PATH`: Optional, set a specific path to store appsignal tmp files
///
/// To run AppSignal the extension needs to be able to start the agent. You can find this
/// executable in `target/debug,release/appsignal/out. When deploying the Rust binary you also
/// need to deploy this file and configure it's path in `APPSIGNAL_AGENT_PATH`.
///
pub fn start() {
    // Set agent version
    env::set_var("APPSIGNAL_AGENT_VERSION", AGENT_VERSION);

    // Set default config if not already present
    match env::var("APPSIGNAL_PUSH_API_ENDPOINT") {
        Err(_) => env::set_var("APPSIGNAL_PUSH_API_ENDPOINT", "https://push.appsignal.com"),
        _ => ()
    };
    match env::var("APPSIGNAL_ENVIRONMENT") {
        Err(_) => env::set_var("APPSIGNAL_ENVIRONMENT", "development"),
        _ => ()
    };
    match env::var("APPSIGNAL_APP_PATH") {
        Err(_) => env::set_var("APPSIGNAL_APP_PATH", env::var("PWD").unwrap()),
        _ => ()
    };

    unsafe {
        bindings::appsignal_start();
    }
}

/// Stops the extension after transmitting the outstanding payload to the agent
pub fn stop() {
    unsafe {
        bindings::appsignal_stop();
    }
}

pub struct Transaction {
    index: i32
}

impl Transaction {
    /// Start a transaction
    ///
    /// Call this when a transaction such as a http request or background job starts.
    pub fn start(transaction_id: &str, namespace: &str) -> Transaction {
        let transaction_id_cstring = CString::new(transaction_id).unwrap();
        let namespace_cstring = CString::new(namespace).unwrap();
        let index = unsafe {
            bindings::appsignal_start_transaction(
                transaction_id_cstring.as_ptr(),
                namespace_cstring.as_ptr()
            )
        };

        Transaction { index: index }
    }

    /// Start an event
    ///
    /// Call this when an event within a transaction you want to measure starts, such as
    /// an SQL query or http request.
    pub fn start_event(&mut self) {
        unsafe {
            bindings::appsignal_start_event(self.index);
        }
    }

    /// Finish the currently started event
    ///
    /// Call this when an event ends.
    pub fn finish_event(&mut self, name: &str, title: &str, body: &str, body_format: i32) {
        let name_cstring = CString::new(name).unwrap();
        let title_cstring = CString::new(title).unwrap();
        let body_cstring = CString::new(body).unwrap();
        unsafe {
            bindings::appsignal_finish_event(
                self.index,
                name_cstring.as_ptr(),
                title_cstring.as_ptr(),
                body_cstring.as_ptr(),
                body_format
            );
        }
    }

    /// Set an error for a transaction
    ///
    /// Call this when an error occurs within a transaction. We expect the first word in the
    /// debug output of the error to be the name of the error.
    pub fn set_error<T>(&mut self, error: T) where T: Display {
        let message = format!("{}", error);
        let name = message.split_whitespace().next().unwrap_or("unknown").to_owned();
        let name_cstring = CString::new(name).unwrap();
        let message_cstring = CString::new(message).unwrap();

        // TODO this can be better
        let bt = backtrace::Backtrace::new();
        let mut backtrace_lines = Vec::new();
        for frame in bt.frames() {
            for symbol in frame.symbols() {
                backtrace_lines.push(
                    format!(
                        "{} {}:{}",
                        symbol.name().unwrap(),
                        match symbol.filename() {
                            Some(f) => f.to_string_lossy().to_string(),
                            None => "no file".to_owned(),
                        },
                        symbol.lineno().unwrap_or(0)
                    )
                );
            }
        }
        let backtrace_json = json::encode(&backtrace_lines).unwrap();
        let backtrace_cstring = CString::new(backtrace_json.as_str()).unwrap();

        unsafe {
            bindings::appsignal_set_transaction_error(
                self.index,
                name_cstring.as_ptr(),
                message_cstring.as_ptr(),
                backtrace_cstring.as_ptr()
            );
        }
    }

    /// Set sample data for this transaction
    ///
    /// Use this to add sample data if finish_transaction returns true.
    pub fn set_sample_data<T>(&mut self, key: &str, payload: T) where T: Encodable {
        let key_cstring = CString::new(key).unwrap();
        let payload_json = json::encode(&payload).unwrap();
        let payload_json_cstring = CString::new(payload_json).unwrap();
        unsafe {
            bindings::appsignal_set_transaction_sample_data(
                self.index,
                key_cstring.as_ptr(),
                payload_json_cstring.as_ptr()
            );
        }
    }

    /// Set action of this transaction
    ///
    /// Call this when the identifying action of a transaction is known.
    pub fn set_action(&mut self, action: &str) {
        unsafe {
            let action_cstring = CString::new(action).unwrap();
            bindings::appsignal_set_transaction_action(
                self.index,
                action_cstring.as_ptr()
            );
        }
    }

    /// Set queue start time of this transaction
    ///
    /// Call this when the queue start time in miliseconds is known.
    pub fn set_queue_start(&mut self, queue_start: i64) {
        unsafe {
            bindings::appsignal_set_transaction_queue_start(self.index, queue_start);
        }
    }

    /// Set metadata for this transaction
    ///
    /// Call this when an error occurs within a transaction to set more detailed data about the error
    pub fn set_meta_data(&mut self, key: &str, value: &str) {
        let key_cstring = CString::new(key).unwrap();
        let value_cstring = CString::new(value).unwrap();
        unsafe {
            bindings::appsignal_set_transaction_metadata(
                self.index,
                key_cstring.as_ptr(),
                value_cstring.as_ptr()
            );
        }
    }

    /// Finish this transaction
    ///
    /// Call this when a transaction such as a http request or background job ends.
    pub fn finish(&mut self) {
        unsafe {
            bindings::appsignal_finish_transaction(self.index);
        }
    }

    ///
    /// Complete this transaction
    /// Call this after finishing a transaction (and adding sample data if necessary).
    pub fn complete(self) {
        unsafe {
            bindings::appsignal_complete_transaction(self.index);
        }
    }
}

/// Track panic
///
/// Create and send an error to AppSignal for this panic. Use with `std::panic::set_handler`.
pub fn track_panic(transaction_id: &str, namespace: &str, panic_info: &PanicInfo) {
    let message = if let Some(e) = panic_info.payload().downcast_ref::<&str>() {
        format!("Panic {}", e)
    } else if let Some(e) = panic_info.payload().downcast_ref::<String>() {
        format!("Panic {}", e)
    } else {
        "Panic with no further information available".to_string()
    };

    let mut transaction = Transaction::start(transaction_id, namespace);
    transaction.set_error(message);
    transaction.finish();
    transaction.complete();
}

/// Set a gauge
pub fn set_gauge(key: &str, value: f32) {
    let key = CString::new(key).unwrap();
    unsafe {
        bindings::appsignal_set_gauge(key.as_ptr(), value)
    }
}

/// Increment a counter
pub fn increment_counter(key: &str, count: i32) {
    let key = CString::new(key).unwrap();
    unsafe {
        bindings::appsignal_increment_counter(key.as_ptr(), count)
    }
}

/// Add a value to a distribution
pub fn add_distribution_value(key: &str, value: f32) {
    let key = CString::new(key).unwrap();
    unsafe {
        bindings::appsignal_add_distribution_value(key.as_ptr(), value)
    }
}

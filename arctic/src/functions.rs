use crate::rfn;
use dll_syringe::process::*;
use std::sync::OnceLock;

/// A set of useful functions from dynamic.
pub static FUNCTIONS: OnceLock<DNXFunctions> = OnceLock::new();

/// A structure that contains a set of functions from dynamic.
#[repr(Rust)]
pub struct DNXFunctions {
    /// `dynamic::log(message)` function. Logs both to the side-messages, and to `stdout`.
    dynamic_log: rfn!(&str),

    /// `Memory::read_string(address)` function. Attempts to read a string at `address`.
    memory_read_string: rfn!((i64) -> String),

    /// `PXScript::execute(code, send_to_party, use_main_thread)` function. Executes a script and
    /// optionally sends it to the party.
    pxscript_execute: rfn!(String, bool, bool),

    /// `dynamic::get_delta_time()` function. Gets the current delta-time of the process.
    dynamic_get_delta_time: rfn!(() -> f32),

    /// Special function for making dynamic eject the DLL, rather than the other way around.
    /// This is needed because otherwise the process crashes.
    dynamic_eject_payload: rfn!(OwnedProcess, ProcessModule<BorrowedProcess<'static>>),

    /// Rune VM which you may use to execute Rune code.
    rune_vm_execute: Box<dyn Fn(String) + Send + Sync>,

    /// `dynamic::create_thread_key(name)` function. Creates a globally-accessible thread-key.
    dyamic_add_thread_key: rfn!(String),

    /// `dynamic::set_thread_key_value(name, value)` function. Sets the value of a thread-key.
    dynamic_set_thread_key_value: rfn!(String, bool),

    /// `dynamic::get_thread_key(name)` function. Returns the value of the thread-key.
    dynamic_get_thread_key: rfn!((String) -> bool),

    /// `ui::add_label(identifier, text)` function. Creates a new label with the specified content.
    ui_add_label: Box<dyn Fn(String, String) + Send + Sync>,
}

impl DNXFunctions {
    /// `dynamic::log(message)` function. Logs both to the side-messages, and to `stdout`.
    pub fn dynamic_log(&self, message: &str) {
        (self.dynamic_log)(message)
    }

    /// `Memory::read_string(address) function. Attempts to read a string at `address`.
    pub fn memory_read_string(&self, address: i64) -> String {
        (self.memory_read_string)(address)
    }

    /// `PXScript::execute(code, send_to_party, use_main_thread)` function. Executes a script and
    /// optionally sends it to the party.
    pub fn pxscript_execute(&self, code: String, send_to_party: bool, use_main_thread: bool) {
        (self.pxscript_execute)(code, send_to_party, use_main_thread)
    }

    /// `dynamic::get_delta_time()` function. Gets the current delta-time of the process.
    pub fn dynamic_get_delta_time(&self) -> f32 {
        (self.dynamic_get_delta_time)()
    }

    /// Special function for making dynamic eject the DLL, rather than the other way around.
    /// This is needed because otherwise the process crashes.
    pub fn dynamic_eject_payload(
        &self,
        process: OwnedProcess,
        payload: ProcessModule<BorrowedProcess<'static>>,
    ) {
        (self.dynamic_eject_payload)(process, payload)
    }

    /// Rune VM which you may use to execute Rune code.
    pub fn rune_vm_execute(&self, source: String) {
        (self.rune_vm_execute)(source);
    }

    /// `ui::add_label(identifier, text)` function. Creates a new label with the specified content.
    pub fn ui_add_label(&self, identifier: String, text: String) {
        (self.ui_add_label)(identifier, text);
    }

    /// `dynamic::create_thread_key(name)` function. Creates a globally-accessible thread-key.
    pub fn create_thread_key(&self, identifier: String) {
        (self.dyamic_add_thread_key)(identifier);
    }

    /// `dynamic::set_thread_key_value(name, value)` function. Sets the value of a thread-key.
    pub fn set_thread_key_value(&self, identifier: String, value: bool) {
        (self.dynamic_set_thread_key_value)(identifier, value);
    }

    /// `dynamic::get_thread_key(name)` function. Returns the value of the thread-key.
    pub fn get_thread_key(&self, identifier: String) -> bool {
        (self.dynamic_get_thread_key)(identifier)
    }
}

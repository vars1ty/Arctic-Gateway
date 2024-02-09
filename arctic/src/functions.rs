use dll_syringe::process::*;
use std::sync::{Arc, OnceLock};

/// A set of useful functions from dynamic.
pub static FUNCTIONS: OnceLock<Arc<DNXFunctions>> = OnceLock::new();

/// A structure that contains a set of functions from dynamic.
#[allow(dead_code)]
#[allow(clippy::type_complexity)]
pub struct DNXFunctions {
    /// `dynamic::log(message)` function. Logs both to the side-messages, and to `stdout`.
    dynamic_log: extern "Rust" fn(&str),

    /// `Memory::read_string(address) function. Attempts to read a string at `address`.
    memory_read_string: extern "Rust" fn(i64) -> &'static str,

    /// `PXScript::execute(code, send_to_party, use_main_thread)` function. Executes a script and
    /// optionally sends it to the party.
    pxscript_execute: extern "Rust" fn(String, bool, bool),

    /// `dynamic::get_delta_time()` function. Gets the current delta-time of the process.
    dynamic_get_delta_time: extern "Rust" fn() -> f32,

    /// Special function for making dynamic eject the DLL, rather than the other way around.
    /// This is needed because otherwise the process crashes.
    dynamic_eject_payload: extern "Rust" fn(OwnedProcess, ProcessModule<BorrowedProcess<'static>>),

    /// Rune VM which you may use to execute Rune code.
    rune_vm_execute: Box<dyn Fn(String) + Send + Sync>,

    /// `dynamic::create_thread_key(name)` function. Creates a globally-accessible thread-key.
    dyamic_add_thread_key: extern "Rust" fn(String),

    /// `dynamic::set_thread_key_value(name, value)` function. Sets the value of a thread-key.
    dynamic_set_thread_key_value: extern "Rust" fn(String, bool),

    /// `dynamic::get_thread_key(name)` function. Returns the value of the thread-key.
    dynamic_get_thread_key: extern "Rust" fn(String) -> bool,

    /// `ui::add_window(name)` function. Allocates and displays a new custom window.
    ui_add_window: Box<dyn Fn(String) + Send + Sync>,

    /// `ui::focus_window` function. Focuses the defined window if present.
    ui_focus_window: Box<dyn Fn(String) + Send + Sync>,

    /// `ui::add_label(identifier, text)` function. Creates a new label with the specified content.
    ui_add_label: Box<dyn Fn(String, String, usize) + Send + Sync>,

    /// `ui::add_button(identifier, text, code)` function. Creates a new button with the specified
    /// text and Rune code.
    ui_add_button: Box<dyn Fn(String, String, String, Option<extern "Rust" fn()>) + Send + Sync>,

    /// `ui::add_separator(identifier)` function. Adds a new horizontal separator.
    ui_add_separator: Box<dyn Fn(String) + Send + Sync>,

    /// `ui::add_spacing(identifier, x, y)` function. Adds spacing between widgets.
    ui_add_spacing: Box<dyn Fn(String, f32, f32) + Send + Sync>,

    /// `ui::get_i32_slider_value(identifier)` function. Returns the i32 value of a defined slider.
    ui_get_i32_slider_value: Box<dyn Fn(String) -> i32 + Send + Sync>,

    /// `ui::get_f32_slider_value(identifier)` function. Returns the f32 value of a defined slider.
    ui_get_f32_slider_value: Box<dyn Fn(String) -> f32 + Send + Sync>,

    /// `ui::set_next_item_same_line(identifier)` function. Attempts to make the next upcoming
    /// widget on the currently-active line.
    ui_set_next_item_same_line: Box<dyn Fn(String) + Send + Sync>,

    /// `ui::add_custom_font_label(identifier, text, relative_font_path)` function. Adds a new
    /// label with a custom-loaded font.
    ui_add_custom_font_label: Box<dyn Fn(String, String, String) + Send + Sync>,

    /// `ui::remove_widget(identifier)` function. Attempts to remove the specified widget from the
    /// focused window.
    ui_remove_widget: Box<dyn Fn(String) + Send + Sync>,

    /// `ui::remove_all_widgets()` function. Removes all widgets from the focused window.
    ui_remove_all_widgets: Box<dyn Fn() + Send + Sync>,

    /// `ui::add_i32_slider(identifier, text, min, max, rune_code)` function. Adds a i32 slider to
    /// the UI with optional Rune code execution.
    ui_add_i32_slider: Box<dyn Fn(String, String, i32, i32, Option<String>) + Send + Sync>,

    /// `ui::add_f32_slider(identifier, text, min, max, rune_code)` function. Adds a f32 slider to
    /// the UI with optional Rune code execution.
    ui_add_f32_slider: Box<dyn Fn(String, String, f32, f32, Option<String>) + Send + Sync>,

    /// `Sellix::is_paying_for_product(product_id, bearer_tolen)` function. Checks if the user is
    /// paying for the specified Sellix product.
    sellix_is_paying_for_product: Box<dyn Fn(String, String) -> bool + Send + Sync>,

    /// `Config::has_serial(serial)` function. Checks if the defined serial is present in the
    /// config.
    config_has_serial: Box<dyn Fn(String) -> bool + Send + Sync>,
}

impl DNXFunctions {
    /// `dynamic::log(message)` function. Logs both to the side-messages, and to `stdout`.
    pub fn dynamic_log(&self, message: &str) {
        (self.dynamic_log)(message)
    }

    /// `Memory::read_string(address) function. Attempts to read a string at `address`.
    pub fn memory_read_string(&self, address: i64) -> &'static str {
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

    /// `ui::add_window(name)` function. Allocates and displays a new custom window.
    pub fn ui_add_window(&self, name: String) {
        (self.ui_add_window)(name);
    }

    /// `ui::focus_window` function. Focuses the defined window if present.
    pub fn ui_focus_window(&self, name: String) {
        (self.ui_focus_window)(name);
    }

    /// `ui::add_label(identifier, text)` function. Creates a new label with the specified content.
    pub fn ui_add_label(&self, identifier: String, text: String, font_id: usize) {
        (self.ui_add_label)(identifier, text, font_id);
    }

    /// `ui::add_button(identifier, text, code)` function. Creates a new button with the specified
    /// text and Rune code.
    pub fn ui_add_button(
        &self,
        identifier: String,
        text: String,
        source: String,
        callback: Option<extern "Rust" fn()>,
    ) {
        (self.ui_add_button)(identifier, text, source, callback);
    }

    /// `ui::add_separator(identifier)` function. Adds a new horizontal separator.
    pub fn ui_add_separator(&self, identifier: String) {
        (self.ui_add_separator)(identifier);
    }

    /// `ui::add_spacing(identifier, x, y)` function. Adds spacing between widgets.
    pub fn ui_add_spacing(&self, identifier: String, x: f32, y: f32) {
        (self.ui_add_spacing)(identifier, x, y);
    }

    /// `dynamic::create_thread_key(name)` function. Creates a globally-accessible thread-key.
    pub fn create_thread_key(&self, identifier: String) {
        (self.dyamic_add_thread_key)(identifier);
    }

    /// `dynamic::set_thread_key_value(name, value)` function. Sets the value of a thread-key.
    pub fn set_thread_key_value(&self, identifier: String, value: bool) {
        (self.dynamic_set_thread_key_value)(identifier, value)
    }

    /// `dynamic::get_thread_key(name)` function. Returns the value of the thread-key.
    pub fn get_thread_key(&self, identifier: String) -> bool {
        (self.dynamic_get_thread_key)(identifier)
    }

    /// `ui::get_i32_slider_value(identifier)` function. Returns the i32 value of a defined slider.
    pub fn get_i32_slider_value(&self, identifier: String) -> i32 {
        (self.ui_get_i32_slider_value)(identifier)
    }

    /// `ui::get_f32_slider_value(identifier)` function. Returns the f32 value of a defined slider.
    pub fn get_f32_slider_value(&self, identifier: String) -> f32 {
        (self.ui_get_f32_slider_value)(identifier)
    }

    /// `ui::set_next_item_same_line(identifier)` function. Attempts to make the next upcoming
    /// widget on the currently-active line.
    pub fn ui_set_next_item_same_line(&self, identifier: String) {
        (self.ui_set_next_item_same_line)(identifier);
    }

    /// `ui::add_custom_font_label(identifier, text, relative_font_path)` function. Adds a new
    /// label with a custom-loaded font.
    pub fn ui_add_custom_font_label(
        &self,
        identifier: String,
        text: String,
        relative_font_path: String,
    ) {
        (self.ui_add_custom_font_label)(identifier, text, relative_font_path);
    }

    /// `ui::remove_widget(identifier)` function. Attempts to remove the specified widget from the
    /// focused window.
    pub fn ui_remove_widget(&self, identifier: String) {
        (self.ui_remove_widget)(identifier);
    }

    /// `ui::remove_all_widgets()` function. Removes all widgets from the focused window.
    pub fn ui_remove_all_widgets(&self) {
        (self.ui_remove_all_widgets)();
    }

    /// `ui::add_i32_slider(identifier, text, min, max, rune_code)` function. Adds a i32 slider to
    /// the UI with optional Rune code execution.
    pub fn ui_add_i32_slider(
        &self,
        identifier: String,
        text: String,
        min: i32,
        max: i32,
        rune_code: Option<String>,
    ) {
        (self.ui_add_i32_slider)(identifier, text, min, max, rune_code);
    }

    /// `ui::add_f32_slider(identifier, text, min, max, rune_code)` function. Adds a f32 slider to
    /// the UI with optional Rune code execution.
    pub fn ui_add_f32_slider(
        &self,
        identifier: String,
        text: String,
        min: f32,
        max: f32,
        rune_code: Option<String>,
    ) {
        (self.ui_add_f32_slider)(identifier, text, min, max, rune_code);
    }

    /// `Sellix::is_paying_for_product(product_id, bearer_tolen)` function. Checks if the user is
    /// paying for the specified Sellix product.
    pub fn sellix_is_paying_for_product(&self, product_id: String, bearer_token: String) -> bool {
        (self.sellix_is_paying_for_product)(product_id, bearer_token)
    }

    /// `Config::has_serial(serial)` function. Checks if the defined serial is present in the
    /// config.
    pub fn config_has_serial(&self, serial: String) -> bool {
        (self.config_has_serial)(serial)
    }
}

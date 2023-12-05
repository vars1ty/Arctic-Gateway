/// `dynamic::log(message)` function. Logs both to the side-messages, and to `stdout`.
#[macro_export]
macro_rules! log {
    ($message:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.dynamic_log($message);
        }
    };
}

/// `PXScript::execute(code, send_to_party, use_main_thread)` function. Executes a script and
/// optionally sends it to the party.
#[macro_export]
macro_rules! pxscript_execute {
    ($code:expr, $send_to_party:expr, $use_main_thread:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.pxscript_execute($code.to_owned(), $send_to_party, $use_main_thread);
        }
    };
}

/// Special function for making dynamic eject the DLL, rather than the other way around.
/// This is needed because otherwise the process crashes.
#[macro_export]
macro_rules! eject_payload {
    ($process:expr, $payload:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.dynamic_eject_payload($process, $payload);
        }
    };
}

/// Rune VM which you may use to execute Rune code.
#[macro_export]
macro_rules! rune_vm_execute {
    ($source:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.rune_vm_execute($source);
        }
    };
}

/// `ui::add_label(identifier, text)` function. Creates a new label with the specified content.
#[macro_export]
macro_rules! ui_add_label {
    ($name:expr, $text:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_label($name.to_owned(), $text.to_owned());
        }
    };
}

/// `ui::add_button(identifier, text, code)` function. Creates a new button with the specified
/// text and Rune code.
#[macro_export]
macro_rules! ui_add_button {
    ($name:expr, $text:expr, $source:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_button($name.to_owned(), $text.to_owned(), $source.to_owned());
        }
    };
}

/// `ui::add_separator(identifier)` function. Adds a new horizontal separator.
#[macro_export]
macro_rules! ui_add_separator {
    ($name:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_separator($name.to_owned());
        }
    };
}

/// `ui::add_spacing(identifier, x, y)` function. Adds spacing between widgets.
#[macro_export]
macro_rules! ui_add_spacing {
    ($name:expr, $x:expr, $y:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_spacing($name.to_owned(), $x, $y);
        }
    };
}

/// Rune VM which you may use to execute Rune code.
#[macro_export]
macro_rules! create_thread_key {
    ($name:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.create_thread_key($name.to_owned());
        }
    };
}

/// Rune VM which you may use to execute Rune code.
#[macro_export]
macro_rules! set_thread_key_value {
    ($name:expr, $value:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.set_thread_key_value($name.to_owned(), $value);
        }
    };
}

/// Rune VM which you may use to execute Rune code.
#[macro_export]
macro_rules! get_thread_key {
    ($name:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.get_thread_key($name.to_owned())
        } else {
            false
        }
    };
}

/// Creates a specialized hook with a custom name, which upon setting the value to `true`, releases
/// the DLL from the process and calls the `on_pre_eject` closure, where you perform any needed
/// cleanup.
#[macro_export]
macro_rules! setup_auto_eject_tk_listener {
    ($name:expr, $process:expr, $payload:expr, $on_pre_eject:expr) => {
        create_thread_key!($name);
        set_thread_key_value!($name, false);
        std::thread::spawn(move || {
            while !get_thread_key!($name) {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            #[allow(clippy::redundant_closure_call)]
            #[allow(unused_unsafe)]
            unsafe {
                $on_pre_eject();
            }

            eject_payload!($process, $payload);
        });
    };
}

/// Enables a hook, returning an instance to the created `hook` regardless of if it fails or not.
#[macro_export]
macro_rules! enable_hook {
    ($hook:expr, $fn_address:expr, $callback:expr, $hook_name:literal) => {
        unsafe {
            let hook = $hook.initialize(std::mem::transmute($fn_address), $callback);
            if let Ok(hook) = hook {
                if let Err(error) = hook.enable() {
                    log!(&format!(
                        "[ERROR] Failed enabling hook {}, error: {error}",
                        $hook_name
                    ));
                } else {
                    log!(&format!("Hook {} loaded successfully!", $hook_name));
                }
            } else {
                log!(&format!(
                    "[ERROR] Failed initializing hook {}, error: {}",
                    $hook_name,
                    hook.as_ref().unwrap_err_unchecked()
                ));
            }

            hook
        }
    };
}

/// `ui::get_i32_slider_value(identifier)` function. Returns the i32 value of a defined slider.
#[macro_export]
macro_rules! get_i32_slider_value {
    ($name:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.get_i32_slider_value($name.to_owned())
        } else {
            0
        }
    };
}

/// `ui::get_f32_slider_value(identifier)` function. Returns the f32 value of a defined slider.
#[macro_export]
macro_rules! get_f32_slider_value {
    ($name:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.get_f32_slider_value($name.to_owned())
        } else {
            0.0
        }
    };
}

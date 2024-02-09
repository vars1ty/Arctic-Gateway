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

/// `ui::add_window(name)` function. Allocates and displays a new custom window.
#[macro_export]
macro_rules! ui_add_window {
    ($identifier:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_window($identifier.to_owned());
        }
    };
}

/// `ui::focus_window` function. Focuses the defined window if present.
#[macro_export]
macro_rules! ui_focus_window {
    ($identifier:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_focus_window($identifier.to_owned());
        }
    };
}

/// `ui::add_label(identifier, text)` function. Creates a new label with the specified content.
#[macro_export]
macro_rules! ui_add_label {
    ($identifier:expr, $text:expr, $font_id:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_label($identifier.to_owned(), $text.to_owned(), $font_id);
        }
    };
}

/// `ui::add_button(identifier, text, code)` function. Creates a new button with the specified
/// text and Rune code.
#[macro_export]
macro_rules! ui_add_button {
    ($identifier:expr, $text:expr, $source:expr, $callback:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_button(
                $identifier.to_owned(),
                $text.to_owned(),
                $source.to_owned(),
                $callback,
            );
        }
    };
}

/// `ui::add_separator(identifier)` function. Adds a new horizontal separator.
#[macro_export]
macro_rules! ui_add_separator {
    ($identifier:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_separator($identifier.to_owned());
        }
    };
}

/// `ui::add_spacing(identifier, x, y)` function. Adds spacing between widgets.
#[macro_export]
macro_rules! ui_add_spacing {
    ($identifier:expr, $x:expr, $y:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_spacing($identifier.to_owned(), $x, $y);
        }
    };
}

/// `dynamic::create_thread_key(name)` function. Creates a globally-accessible thread-key.
#[macro_export]
macro_rules! create_thread_key {
    ($identifier:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.create_thread_key($identifier.to_owned());
        }
    };
}

/// `dynamic::set_thread_key_value(name, value)` function. Sets the value of a thread-key.
#[macro_export]
macro_rules! set_thread_key_value {
    ($identifier:expr, $value:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.set_thread_key_value($identifier.to_owned(), $value);
        }
    };
}

/// `dynamic::get_thread_key(name)` function. Returns the value of the thread-key.
#[macro_export]
macro_rules! get_thread_key {
    ($identifier:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.get_thread_key($identifier.to_owned())
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
    ($identifier:expr, $process:expr, $payload:expr, $on_pre_eject:expr) => {
        create_thread_key!($identifier);
        set_thread_key_value!($identifier, false);
        std::thread::spawn(move || {
            while !get_thread_key!($identifier) {
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
    ($identifier:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.get_i32_slider_value($identifier.to_owned())
        } else {
            0
        }
    };
}

/// `ui::get_f32_slider_value(identifier)` function. Returns the f32 value of a defined slider.
#[macro_export]
macro_rules! get_f32_slider_value {
    ($identifier:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.get_f32_slider_value($identifier.to_owned())
        } else {
            0.0
        }
    };
}

/// `ui::set_next_item_same_line(identifier)` function. Attempts to make the next upcoming
/// widget on the currently-active line.
#[macro_export]
macro_rules! ui_set_next_item_same_line {
    ($identifier:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_set_next_item_same_line($identifier.to_owned());
        }
    };
}

/// `ui::add_custom_font_label(identifier, text, relative_font_path)` function. Adds a new
/// label with a custom-loaded font.
#[macro_export]
macro_rules! ui_add_custom_font_label {
    ($identifier:expr, $text:expr, $relative_font_path:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_custom_font_label(
                $identifier.to_owned(),
                $text.to_owned(),
                $relative_font_path.to_owned(),
            );
        }
    };
}

/// `ui::remove_widget(identifier)` function. Attempts to remove the specified widget from the
/// focused window.
#[macro_export]
macro_rules! ui_remove_widget {
    ($identifier:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_remove_widget($identifier.to_owned());
        }
    };
}

/// `ui::remove_all_widgets()` function. Removes all widgets from the focused window.
#[macro_export]
macro_rules! ui_remove_all_widgets {
    () => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_remove_all_widgets();
        }
    };
}

/// `ui::add_i32_slider(identifier, text, min, max, rune_code)` function. Adds a i32 slider to
/// the UI with optional Rune code execution.
#[macro_export]
macro_rules! ui_add_i32_slider {
    ($identifier:expr, $text:expr, $min:expr, $max:expr, $rune_code:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_i32_slider(
                $identifier.to_owned(),
                $text.to_owned(),
                $min,
                $max,
                $rune_code,
            );
        }
    };
}

/// `ui::add_f32_slider(identifier, text, min, max, rune_code)` function. Adds a f32 slider to
/// the UI with optional Rune code execution.
#[macro_export]
macro_rules! ui_add_f32_slider {
    ($identifier:expr, $text:expr, $min:expr, $max:expr, $rune_code:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.ui_add_f32_slider(
                $identifier.to_owned(),
                $text.to_owned(),
                $min,
                $max,
                $rune_code,
            );
        }
    };
}

/// `Sellix::is_paying_for_product(product_id, bearer_tolen)` function. Checks if the user is
/// paying for the specified Sellix product.
#[macro_export]
macro_rules! sellix_is_paying_for_product {
    ($product_id:expr, $bearer_token:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.sellix_is_paying_for_product($product_id.to_owned(), $bearer_token.to_owned())
        } else {
            false
        }
    };
}

/// `Config::has_serial(serial)` function. Checks if the defined serial is present in the
/// config.
#[macro_export]
macro_rules! config_has_serial {
    ($serial:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.config_has_serial($serial.to_owned())
        } else {
            false
        }
    };
}

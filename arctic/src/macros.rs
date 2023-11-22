/// Equivalent to typing `extern "Rust" fn (/* Types */) -> // Return Type`.
/// The return is optional and non-required.
/// Example: `myfn: rfn!((i32) -> String)`
#[macro_export]
macro_rules! rfn {
    (($($args:ty),*) -> $ret:ty) => {
        extern "Rust" fn($($args),*) -> $ret
    };
    ($($args:ty),*) => {
        extern "Rust" fn($($args),*)
    };
}

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

/// `dynamic::create_thread_key(name)` function. Creates a globally-accessible thread-key.
#[macro_export]
macro_rules! create_thread_key {
    ($name:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.create_thread_key($name.to_owned());
        }
    };
}

/// `dynamic::set_thread_key_value(name, value)` function. Sets the value of a thread-key.
#[macro_export]
macro_rules! set_thread_key_value {
    ($name:expr, $value:expr) => {
        if let Some(functions) = $crate::functions::FUNCTIONS.get() {
            functions.set_thread_key_value($name.to_owned(), $value);
        }
    };
}

/// `dynamic::get_thread_key(name)` function. Returns the value of the thread-key.
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

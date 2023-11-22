use crate::functions::*;
use dll_syringe::process::{BorrowedProcess, OwnedProcess, ProcessModule};

mod functions;

#[macro_use]
mod macros;

/// This is the Arctic Gateway main function.
/// Enable no_mangle so that the function name doesn't get mangled. This is only required for the
/// main function, not for the rest.
/// The purpose of the gateway is to collect a set of useful functions, then store them globally.
/// Note that you **need** to have all of the initial variables defined, otherwise you'll crash.
#[no_mangle]
pub fn arctic_gateway(
    process: OwnedProcess,
    payload: ProcessModule<BorrowedProcess<'static>>,
    functions: DNXFunctions,
) {
    // Store the functions structure globally.
    FUNCTIONS.get_or_init(|| functions);

    // Try and log to dynamic.
    log!("Arctic Gateway template active!");

    // Eject.
    eject_payload!(process, payload);
}

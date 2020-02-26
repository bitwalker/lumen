mod atoms;
mod symbols;

extern "C" {
    /// The target-defined entry point for the generated executable.
    ///
    /// Each target has its own runtime crate, which uses `#[entry]` from
    /// the `liblumen_core` crate to specify its entry point. This is
    /// called after the atom table, and any other core runtime functionality,
    /// is initialized and ready for use.
    #[link_name = "lumen_entry"]
    fn lumen_entry() -> i32;
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    lumen_start() as isize
}

/// The primary entry point for the Lumen runtime
///
/// This function is responsible for setting up any core functionality required
/// by the higher-level runtime, e.g. initializing the atom table. Once initialized,
/// this function invokes the platform-specific entry point which handles starting
/// up the schedulers and other high-level runtime functionality.
pub fn lumen_start() -> i32 {
    use crate::atoms::*;
    use crate::symbols::*;

    // Initialize atom table
    if unsafe { InitializeLumenAtomTable(ATOM_TABLE, NUM_ATOMS) } == false {
        return 102;
    }

    // Initialize the dispatch table
    if unsafe { InitializeLumenDispatchTable(SYMBOL_TABLE, NUM_SYMBOLS) } == false {
        return 103;
    }

    // Invoke platform-specific entry point
    unsafe { lumen_entry() }
}

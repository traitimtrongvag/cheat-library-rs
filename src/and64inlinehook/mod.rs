//! And64InlineHook – ARM64 inline hooking utilities
//!
//! A lightweight Rust implementation for patching ARM64/AArch64 functions at runtime.
//! The library rewrites instructions at the target address and redirects execution to
//! a user-defined hook, while keeping a small trampoline to call the original code.
//!
//! # What it provides
//! - Basic ARM64 instruction relocation
//! - PC-relative fixups when moving instructions
//! - A small thread-safe trampoline pool
//! - Works on Android and other ARM64 environments
//!
//! # Safety
//! All functions are unsafe because they:
//! - Patch executable memory
//! - Change memory protection flags
//! - Use raw pointers and direct instruction writes
//!
//! # Example
//! ```no_run
//! use and64inlinehook::{init_hook_pool, a64_hook_function};
//!
//! unsafe {
//!     init_hook_pool();
//!
//!     let target_fn = some_function as *mut u32;
//!     let hook_fn   = my_hook as *const u32;
//!
//!     if let Some(trampoline) = a64_hook_function(target_fn, hook_fn) {
//!         println!("Hook installed.");
//!         // trampoline can be used to call the original function
//!     }
//! }
//! ```

mod and64inlinehook;

// Re-export the main public API
pub use and64inlinehook::{
    a64_hook_function,
    a64_hook_function_v,
    init_hook_pool,
};

// Re-export C FFI functions for compatibility
pub use and64inlinehook::{
    A64HookFunction,
    A64HookFunctionV,
};

// Export constants that might be useful for users
pub mod constants {
    /// Maximum number of instructions that can be relocated
    pub const A64_MAX_INSTRUCTIONS: usize = 5;
    
    /// Maximum number of reference fixups per instruction
    pub const A64_MAX_REFERENCES: usize = A64_MAX_INSTRUCTIONS * 2;
    
    /// Maximum number of trampolines in the pool
    pub const A64_MAX_BACKUPS: usize = 256;
    
    /// ARM64 NOP instruction encoding
    pub const A64_NOP: u32 = 0xd503201f;
    
    /// System page size
    pub const PAGE_SIZE: usize = 4096;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constants() {
        assert_eq!(constants::A64_MAX_INSTRUCTIONS, 5);
        assert_eq!(constants::A64_MAX_BACKUPS, 256);
        assert_eq!(constants::A64_NOP, 0xd503201f);
    }
}
pub mod kittymemory;
pub mod il2cpp_sdk;
pub mod dobby;
pub mod tools;

pub use crate::includes::kittymemory::*;
pub use crate::includes::il2cpp_sdk::*;
pub use crate::includes::dobby::*;
// explicit re-exports from tools to avoid MonoString ambiguity with il2cpp_sdk::MonoString
pub use crate::includes::tools::base64::{base64_encode, base64_decode, base64_encode_string, base64_encode_pem, base64_encode_mime};
pub use crate::includes::tools::str_enc::StrEnc;
pub use crate::includes::tools::file_wrapper::FileWrapper;
pub use crate::includes::tools::jni_stuff::{init_jvm, get_env, get_global_context, get_clipboard, write_clipboard, show_soft_keyboard_input, hide_soft_keyboard_input};
pub use crate::includes::tools::tools::*;
// MonoString from tools is intentionally not re-exported here; use includes::tools::mono_string directly if needed

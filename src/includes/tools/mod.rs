pub mod base64;
pub mod mono_string;
pub mod str_enc;
pub mod file_wrapper;
pub mod jni_stuff;
pub mod tools;

pub use base64::{base64_encode, base64_decode, base64_encode_string, base64_encode_pem, base64_encode_mime};
pub use mono_string::{MonoString, utf16_to_utf8, utf16le_to_utf8, utf16be_to_utf8, utf8_to_utf16le, utf8_to_utf16be};
pub use str_enc::StrEnc;
pub use file_wrapper::FileWrapper;

pub use jni_stuff::{init_jvm, get_env, get_global_context, get_clipboard, write_clipboard, show_soft_keyboard_input,hide_soft_keyboard_input};
pub use tools::*;

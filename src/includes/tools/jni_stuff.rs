use jni::{JNIEnv, JavaVM};
use jni::objects::{JObject, JValue, GlobalRef};
use std::sync::{OnceLock, Mutex};

static JVM: OnceLock<JavaVM> = OnceLock::new();

pub fn init_jvm(vm: &JavaVM) {
    let _ = JVM.set(unsafe { JavaVM::from_raw(vm.get_java_vm_pointer()).unwrap() });
}

struct EnvHolder {
    guard: Option<jni::AttachGuard<'static>>,
}

thread_local! {
    static ENV_HOLDER: Mutex<EnvHolder> = Mutex::new(EnvHolder { guard: None });
}

pub fn get_env() -> Option<JNIEnv<'static>> {
    let jvm = JVM.get()?;
    
    match jvm.get_env() {
        Ok(env) => {
            Some(unsafe { std::mem::transmute(env) })
        }
        Err(_) => {
            match jvm.attach_current_thread() {
                Ok(guard) => {
                    let guard_static: jni::AttachGuard<'static> = unsafe { std::mem::transmute(guard) };
                    
                    ENV_HOLDER.with(|holder| {
                        let mut h = holder.lock().unwrap();
                        h.guard = Some(guard_static);
                    });
                    
                    match jvm.get_env() {
                        Ok(env) => Some(unsafe { std::mem::transmute(env) }),
                        Err(e) => {
                            log::error!("Failed to get env after attach: {}", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    log::error!("Error attaching thread: {}", e);
                    None
                }
            }
        }
    }
}

fn get_global_context_global(env: &mut JNIEnv) -> Option<GlobalRef> {
    let activity_thread = env.find_class("android/app/ActivityThread").ok()?;
    
    let at_result = env.call_static_method(
        activity_thread,
        "currentActivityThread",
        "()Landroid/app/ActivityThread;",
        &[]
    ).ok()?;
    
    let at = at_result.l().ok()?;
    
    let context_result = env.call_method(
        at,
        "getApplication",
        "()Landroid/app/Application;",
        &[]
    ).ok()?;
    
    let context = context_result.l().ok()?;
    env.new_global_ref(context).ok()
}

pub fn get_global_context<'a>(env: &'a mut JNIEnv) -> Option<JObject<'a>> {
    let activity_thread = env.find_class("android/app/ActivityThread").ok()?;
    
    let at_result = env.call_static_method(
        activity_thread,
        "currentActivityThread",
        "()Landroid/app/ActivityThread;",
        &[]
    ).ok()?;
    
    let at = at_result.l().ok()?;
    
    let context_result = env.call_method(
        at,
        "getApplication",
        "()Landroid/app/Application;",
        &[]
    ).ok()?;
    
    let context = context_result.l().ok()?;
    Some(context)
}

pub fn get_clipboard() -> Option<String> {
    let mut env = get_env()?;
    
    let context_global = get_global_context_global(&mut env)?;
    let context_obj = context_global.as_obj();
    
    let clipboard_service = env.new_string("clipboard").ok()?;
    
    let manager_result = env.call_method(
        context_obj,
        "getSystemService",
        "(Ljava/lang/String;)Ljava/lang/Object;",
        &[JValue::Object(&clipboard_service.into())]
    ).ok()?;
    
    let clipboard_manager = manager_result.l().ok()?;
    
    let text_result = env.call_method(
        clipboard_manager,
        "getText",
        "()Ljava/lang/CharSequence;",
        &[]
    ).ok()?;
    
    let text_obj = text_result.l().ok()?;
    
    if text_obj.is_null() {
        return None;
    }
    
    let string_result = env.call_method(
        text_obj,
        "toString",
        "()Ljava/lang/String;",
        &[]
    ).ok()?;
    
    let jstring = string_result.l().ok()?;
    
    let result: String = env.get_string((&jstring).into()).ok()?.into();
    Some(result)
}

pub fn write_clipboard(text: &str) -> Result<(), String> {
    let mut env = get_env().ok_or("Failed to get JNI env")?;
    
    let context_global = get_global_context_global(&mut env)
        .ok_or("Failed to get global context")?;
    let context_obj = context_global.as_obj();
    
    let clipboard_service = env.new_string("clipboard")
        .map_err(|e| format!("Failed to create string: {}", e))?;
    
    let manager_result = env.call_method(
        context_obj,
        "getSystemService",
        "(Ljava/lang/String;)Ljava/lang/Object;",
        &[JValue::Object(&clipboard_service.into())]
    ).map_err(|e| format!("Failed to get clipboard manager: {}", e))?;
    
    let clipboard_manager = manager_result.l()
        .map_err(|e| format!("Failed to convert: {}", e))?;
    
    let jtext = env.new_string(text)
        .map_err(|e| format!("Failed to create text string: {}", e))?;
    
    env.call_method(
        clipboard_manager,
        "setText",
        "(Ljava/lang/CharSequence;)V",
        &[JValue::Object(&jtext.into())]
    ).map_err(|e| format!("Failed to set text: {}", e))?;
    
    Ok(())
}

pub fn show_soft_keyboard_input() -> Result<bool, String> {
    let mut env = get_env().ok_or("Failed to get JNI env")?;
    
    let context_global = get_global_context_global(&mut env)
        .ok_or("Failed to get global context")?;
    let context_obj = context_global.as_obj();
    
    let input_method_service = env.new_string("input_method")
        .map_err(|e| format!("Failed to create string: {}", e))?;
    
    let manager_result = env.call_method(
        context_obj,
        "getSystemService",
        "(Ljava/lang/String;)Ljava/lang/Object;",
        &[JValue::Object(&input_method_service.into())]
    ).map_err(|e| format!("Failed to get input method service: {}", e))?;
    
    let input_method_manager = manager_result.l()
        .map_err(|e| format!("Failed to convert: {}", e))?;
    
    env.call_method(
        input_method_manager,
        "toggleSoftInput",
        "(II)V",
        &[JValue::Int(2), JValue::Int(0)]
    ).map_err(|e| format!("Failed to toggle soft input: {}", e))?;
    
    Ok(true)
}

pub fn hide_soft_keyboard_input() -> Result<(), String> {
    let mut env = get_env().ok_or("Failed to get JNI env")?;
    
    let context_global = get_global_context_global(&mut env)
        .ok_or("Failed to get global context")?;
    let context_obj = context_global.as_obj();
    
    let input_method_service = env.new_string("input_method")
        .map_err(|e| format!("Failed to create string: {}", e))?;
    
    let manager_result = env.call_method(
        context_obj,
        "getSystemService",
        "(Ljava/lang/String;)Ljava/lang/Object;",
        &[JValue::Object(&input_method_service.into())]
    ).map_err(|e| format!("Failed to get input method service: {}", e))?;
    
    let input_method_manager = manager_result.l()
        .map_err(|e| format!("Failed to convert: {}", e))?;
    
    let null_obj = JObject::null();
    env.call_method(
        input_method_manager,
        "hideSoftInputFromWindow",
        "(Landroid/os/IBinder;I)Z",
        &[JValue::Object(&null_obj), JValue::Int(0)]
    ).map_err(|e| format!("Failed to hide soft input: {}", e))?;
    
    Ok(())
}
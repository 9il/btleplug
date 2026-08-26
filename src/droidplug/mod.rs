pub mod adapter;
pub mod manager;
pub mod peripheral;

use ::jni::Env;
use once_cell::sync::OnceCell;

mod jni;
mod jni_utils;

static GLOBAL_ADAPTER: OnceCell<adapter::Adapter> = OnceCell::new();

pub fn init(env: &mut Env) -> crate::Result<()> {
    self::jni::init(env)?;
    GLOBAL_ADAPTER.get_or_try_init(|| adapter::Adapter::new())?;
    Ok(())
}

/// Initialize from a raw `JNIEnv*` (for hosts using a different `jni` crate version).
///
/// Accepts `*mut c_void` rather than this crate's `jni::sys::JNIEnv` so a host
/// on `jni` 0.21 (`jni-sys` 0.3) can pass `JNIEnv::get_raw()` into a `jni` 0.22
/// (`jni-sys` 0.4) droidplug without a type mismatch.
///
/// # Safety
/// `env` must be a valid JNI environment pointer for the current thread.
pub unsafe fn init_from_raw(env: *mut std::ffi::c_void) -> crate::Result<()> {
    let mut unowned = unsafe { ::jni::EnvUnowned::from_raw(env.cast()) };
    match unowned.with_env(init).into_outcome() {
        ::jni::Outcome::Ok(()) => Ok(()),
        ::jni::Outcome::Err(err) => Err(err),
        ::jni::Outcome::Panic(payload) => std::panic::resume_unwind(payload),
    }
}

pub fn global_adapter() -> &'static adapter::Adapter {
    GLOBAL_ADAPTER.get().expect(
        "Droidplug has not been initialized. Please initialize it with btleplug::platform::init().",
    )
}

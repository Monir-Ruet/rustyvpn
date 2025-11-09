use anyhow::Result;
use jni::{
    objects::{JObject, JValue},
    JavaVM,
};

use ndk_context::android_context;
use tracing::info;

pub fn request_permission(permission: &str) -> Result<()> {
    unsafe {
        let ctx = android_context();
        let vm = JavaVM::from_raw(ctx.vm().cast())?;
        let mut env = vm.attach_current_thread()?;
        let activity = JObject::from_raw(ctx.context() as *mut _);

        if activity.is_null() {
            anyhow::bail!("Err");
        }

        let permission_jstring = env.new_string(permission)?;
        let permission_jobject = JObject::from(permission_jstring);
        let string_class = env.find_class("java/lang/String")?;
        let permissions_array = env.new_object_array(1, string_class, JObject::null())?;
        env.set_object_array_element(&permissions_array, 0, permission_jobject)?;
        env.call_method(
            activity,
            "requestPermissions",
            "([Ljava/lang/String;I)V",
            &[JValue::Object(&permissions_array), JValue::Int(1)],
        )?;

        info!("requestPermissions called");

        Ok(())
    }
}

pub fn check_if_permission_granted(permission: &str) -> Result<bool> {
    unsafe {
        let ctx = android_context();
        let vm = JavaVM::from_raw(ctx.vm().cast())?;
        let mut env = vm.attach_current_thread()?;
        let activity = JObject::from_raw(ctx.context() as *mut _);

        if activity.is_null() {
            anyhow::bail!("Err");
        }

        let permission_jstring = env.new_string(permission)?;
        let permission_jobject = JObject::from(permission_jstring);

        let result = env.call_method(
            activity,
            "checkSelfPermission",
            "(Ljava/lang/String;)I",
            &[JValue::Object(&permission_jobject)],
        )?;

        let granted = result.i()? == 0;

        Ok(granted)
    }
}

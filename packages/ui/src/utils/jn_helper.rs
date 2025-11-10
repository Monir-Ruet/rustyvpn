use std::sync::Arc;

use anyhow::Result;
use jni::{
    objects::{JClass, JObject, JValue},
    JNIEnv, JavaVM,
};
use ndk_context::{android_context, AndroidContext};
use tracing::info;

use crate::jni_utils::{CLASS_LOADER, GLOBAL_LOCK};

pub struct JniHelper {
    android_context: AndroidContext,
    jvm: Arc<JavaVM>,
}

impl JniHelper {
    pub fn new() -> Self {
        unsafe {
            let ctx = android_context();
            let vm = JavaVM::from_raw(ctx.vm().cast()).unwrap();
            Self {
                android_context: ctx,
                jvm: Arc::new(vm),
            }
        }
    }

    pub fn request_permission(&self, permission: &str) -> Result<()> {
        unsafe {
            let ctx = self.android_context;
            let mut env = self.jvm.attach_current_thread()?;
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

    pub fn check_if_permission_granted(&self, permission: &str) -> Result<bool> {
        unsafe {
            let ctx = self.android_context;
            let mut env = self.jvm.attach_current_thread()?;
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

    pub fn call_vpn_method(&self, method: &str) -> anyhow::Result<()> {
        unsafe {
            let jvm = self.jvm.clone();
            let mut env = jvm.attach_current_thread()?;
            info!("Calling VPN method: {}", method);
            info!(
                "Android context pointer: {:p}",
                self.android_context.context()
            );

            let activity = JObject::from_raw(self.android_context.context() as *mut _);
            let param = JValue::Object(&activity);
            info!("Activity object created{:?}", activity);

            let class = JClass::from(load_class_obj(&mut env, "dev/dioxus/main/jni/VpnHelper"));

            env.call_static_method(class, method, "(Landroid/content/Context;)V", &[param])?;

            Ok(())
        }
    }
}

pub fn load_class_obj<'local>(env: &mut JNIEnv<'local>, class_name: &str) -> JClass<'local> {
    let _guard = GLOBAL_LOCK.lock().unwrap();

    let loader = unsafe { CLASS_LOADER.as_ref().unwrap().as_obj() };
    let name = env.new_string(class_name).unwrap();
    let x = JObject::from(name);

    let j = env
        .call_method(
            &loader,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[JValue::Object(&x)],
        )
        .unwrap()
        .l()
        .unwrap();
    JClass::from(j)
}

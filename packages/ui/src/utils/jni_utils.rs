use jni::objects::{GlobalRef, JObject};
use jni::{objects::JClass, JNIEnv, JavaVM};
use std::os::unix::io::FromRawFd;
use std::sync::{Mutex, Once};
use std::{fs::File, io::Read};

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_main_jni_NativeBridge_startVpn(
    _env: JNIEnv,
    _class: JClass,
    fd: i32,
) {
    std::thread::spawn(move || {
        handle_tun(fd);
    });
}

static INIT: Once = Once::new();
static mut JVM: Option<JavaVM> = None;
pub static mut CLASS_LOADER: Option<GlobalRef> = None;
pub static GLOBAL_LOCK: Mutex<()> = Mutex::new(());

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_main_jni_NativeBridge_initBridge(
    env: JNIEnv,
    _class: JClass,
    class_loader: JObject,
) {
    INIT.call_once(|| {
        let jvm = env.get_java_vm().unwrap();
        unsafe { JVM = Some(jvm) };

        let global_loader = env.new_global_ref(class_loader).unwrap();
        unsafe { CLASS_LOADER = Some(global_loader) };
    });
}

fn handle_tun(fd: i32) {
    let mut tun = unsafe { File::from_raw_fd(fd) };
    let mut buf = [0u8; 1500];

    loop {
        match tun.read(&mut buf) {
            Ok(n) if n > 0 => {
                println!("Received {} bytes from TUN", n);
                println!("Data: {:x?}", &buf[..n]);
                // Example: print IP version
                match buf[0] >> 4 {
                    4 => println!("IPv4 packet"),
                    6 => println!("IPv6 packet"),
                    _ => println!("Unknown packet"),
                }

                // Here you can forward the packet to your SOCKS/SSH proxy
            }
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
    std::mem::forget(tun);
}

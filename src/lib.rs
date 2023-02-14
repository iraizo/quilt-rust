use std::ffi::c_void;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Deref;
use std::os::windows::prelude::{AsHandle, AsRawHandle};
use std::time::Duration;

use jni::objects::JString;
use jni::{JavaVM, JNIEnv};
use jni::sys::{jsize, JNIInvokeInterface_, JNI_VERSION_1_6};
use windows::Win32::Foundation::{HINSTANCE, HANDLE};
use windows::Win32::System::Console::{AllocConsole, GetStdHandle, STD_HANDLE, SetStdHandle, AttachConsole, SetConsoleTextAttribute, STD_OUTPUT_HANDLE};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Threading::AttachThreadInput;
use windows::s;

use crate::interop::Interop;
type JNI_GetCreatedJavaVMs = fn(
    vmBuf: *mut *mut *const JNIInvokeInterface_,
    bufLen: jsize,
    nVMs: *mut jsize
) -> i32;

pub mod interop;
pub mod minecraft;

unsafe extern "system" fn mthread(_base: usize) -> u32 {
    std::env::set_var("RUST_LOG", "mc-test");
    pretty_env_logger::init();
    AllocConsole();

    let file = OpenOptions::new().write(true).read(true).open("CONOUT$").unwrap();
    SetStdHandle(STD_OUTPUT_HANDLE, HANDLE(file.as_raw_handle() as isize)); 

    let jvm = GetModuleHandleA(s!("jvm.dll")).unwrap();

    let interop = match Interop::new(jvm) {
        Ok(e) => {
            e
        },
        Err(_) => panic!("Failed to initialize interop"),
    };

    let get_mc_version = interop.env.find_class("s").unwrap();
    println!("net.minecraft.MinecraftVersion (s): {:?}", get_mc_version);

    let inst = interop.env.call_static_method(get_mc_version, "a", "()Lab;", &[]).unwrap();

    println!("minecraftVersion instance created: {:?}", inst);

    let ret = interop.env.call_method(inst.l().unwrap(), "getName", "()Ljava/lang/String;", &[]).unwrap();

    let version: String = interop.env.get_string(JString::from(ret.l().unwrap())).unwrap().into();

    println!("version: {:?}", version);


    


 /* 
    let get_name = interop.env.get_method_id(get_mc_version, "getName", "()Ljava/lang/String;").unwrap();
    println!("net.minecraft.MinecraftVersion.getName (getName()Ljava/lang/String): {:?}", get_name);

    let ret = interop.env.call_method(get_mc_version, "getName", "()Ljava/lang/String;", &[]).unwrap();
    println!("call done");

    let version: String = interop.env.get_string(JString::from(ret.l().unwrap())).unwrap().into();

    println!("version: {:?}", version); */

   // let jni_get_created_java_vms: JNI_GetCreatedJavaVMs = std::mem::transmute(GetProcAddress(jvm_h, s!("JNI_GetCreatedJavaVMs")).unwrap());

  /*   println!("Got address of JAVA VM: {:?}", jni_get_created_java_vms);

    let mut vms: *mut *mut *const JNIInvokeInterface_ = Vec::with_capacity(1).as_mut_ptr();
    let mut count: jsize = 0;
    //vm.get_env().unwrap().find_class("net/minecraft/unmapped/C_iaepbmfi").unwrap();
    //println!("env: {:?}", vm.get_env().unwrap().find_class("net/minecraft/unmapped/C_iaepbmfi"));
    let r = jni_get_created_java_vms(vms, 1, &mut count);

    let vm = &mut *(vms as *mut JavaVM);

    let guard = vm.attach_current_thread().unwrap();

    let env = guard.deref();

   // let y = env.find_class("ejf").unwrap();

    let get_mc_version = env.find_class("s").unwrap();
    println!("net.minecraft.MinecraftVersion (s): {:?}", get_mc_version);
    let get_name = env.get_method_id(get_mc_version, "getName", "()Ljava/lang/String;").unwrap();
    println!("net.minecraft.MinecraftVersion.getName (getName()Ljava/lang/String): {:?}", get_name);

    let ret = env.call_method(get_mc_version, "getName", "()Ljava/lang/String;", &[]).unwrap();
    println!("call done");

    let version: String = env.get_string(JString::from(ret.l().unwrap())).unwrap().into();

    println!("version: {:?}", version);





    

    println!("return: {:?}", r);
    println!("VM: {:?} Count: {:?}", vms, count);
 //   println!("net.minecraft.client.MinecraftClient (ejf): {:?}", y);
    println!("done"); */

    std::thread::sleep(Duration::from_secs(10)); 

    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    module: HINSTANCE,
    call_reason: u32,
    _reserved: *mut c_void,
) -> i32 {
    if call_reason == 1 {
        let base = module.0 as usize;

        std::thread::spawn(move || mthread(base));

        1
    } else {
        1
    }
}
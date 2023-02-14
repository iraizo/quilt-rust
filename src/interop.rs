use std::ops::Deref;

use jni::{JavaVM, JNIEnv, sys::{JNIInvokeInterface_, jsize}, AttachGuard, objects::{JValue, JObject}};
use windows::{Win32::{Foundation::HINSTANCE, System::LibraryLoader::GetProcAddress}, s};
use anyhow::{Result, anyhow};

type JNI_GetCreatedJavaVMs = fn(
    vmBuf: *mut *mut *const JNIInvokeInterface_,
    bufLen: jsize,
    nVMs: *mut jsize
) -> i32;

pub struct Interop<'a> {
    pub env: AttachGuard<'a>
}

impl Interop<'_> {
    /// Returns a new Interop instance to interact between Rust <-> Java/Minecraft.
    /// ## Arguments 
    /// 
    /// * `handle` - A HINSTANCE of the jvm module from the current process. 
    pub fn new(handle: HINSTANCE) -> Result<Self> {
        unsafe {
        let jni_get_created_java_vms: JNI_GetCreatedJavaVMs = std::mem::transmute(GetProcAddress(handle, s!("JNI_GetCreatedJavaVMs")).unwrap());

        // allocate buffers for JNI_GetCreatedJavaVMs to write into
        let mut vms: *mut *mut *const JNIInvokeInterface_ = Vec::with_capacity(1).as_mut_ptr();
        let mut count: jsize = 0;

        let res = jni_get_created_java_vms(vms, 1, &mut count);

        if res != 0 || count == 0 {
            return Err(anyhow!("JNI_GetCreatedJavaVMs returned faulty data!"));
        }

        let vm = &mut *(vms as *mut JavaVM);
        let guard = vm.attach_current_thread()?;



        return Ok(Self {
            env: guard,
        });
    }
    }
}


/// This is supposed to get codegenned later on which is on my TODO list using Tiny v2 mappings hopefully.
struct MinecraftVersion<'a> {
    pub env: &'a JNIEnv<'a>,
    pub instance: JObject<'a>
    // TODO: add missing (java/minecraft) fields
}

impl MinecraftVersion<'_> {
    pub fn new<'a>(env: &JNIEnv<'a>) -> Result<Self> {
        // find class
        let class = env.find_class("s")?;

        // call minecraftVersion.create over JNI
        Ok(Self {
            env,
            instance: env.call_static_method(class, "a", "()Lab;", &[])?.l()?
        })
    }

    pub fn getName(&self, instance: JObject) {
        self.env.call_method(instance, "getName", "()Ljava/lang/String;", &[]).unwrap();
    }
}
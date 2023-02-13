use std::ops::Deref;

use jni::{JavaVM, JNIEnv, sys::{JNIInvokeInterface_, jsize}, AttachGuard};
use windows::{Win32::{Foundation::HINSTANCE, System::LibraryLoader::GetProcAddress}, s};
use anyhow::{Result, anyhow};

type JNI_GetCreatedJavaVMs = fn(
    vmBuf: *mut *mut *const JNIInvokeInterface_,
    bufLen: jsize,
    nVMs: *mut jsize
) -> i32;

pub struct Interop<'a> {
    vm: &'a mut JavaVM,
    env: AttachGuard<'a>
}

impl Interop<'_> {
    /// Returns a new Interop instance to interact between Rust <-> Java/Minecraft.
    /// ## Arguments 
    /// 
    /// * `handle` - A HINSTANCE of the jvm module from the current process. 
    fn new(handle: HINSTANCE) -> Result<Self> {
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
        let guard = vm.attach_current_thread().unwrap();



        Ok(Self {
            vm,
            env: guard,
        })
    }
}
}
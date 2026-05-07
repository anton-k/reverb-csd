// libloading example: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
static REVERB_CSD_FILE: &str = include_str!("./reverb.csd");
use core::ffi::c_void;
use std::path::Path;
use std::{ffi::c_char, ffi::c_int, ptr::null};

pub fn init_csound(sample_rate: usize) -> Csound {
    let mut csound =
        Csound::new(Path::new("/usr/local/lib/libcsound64.so")).expect("Failed to init Csound");
    csound
        .set_option(format!("-n -d -+rtmidi=NULL -M0 -sr {}", sample_rate).as_str())
        .expect("Failed to set sample rate");
    csound.compile_csd_from_str(REVERB_CSD_FILE, true).unwrap();
    csound.start().expect("Failed to start Csound");
    csound
}

#[repr(C)]
struct CsoundPtr;

#[allow(dead_code)]
pub struct Csound {
    lib: libloading::Library,
    csound_ptr: *mut CsoundPtr,

    csound_set_option: unsafe extern "C" fn(*mut CsoundPtr, *const c_char) -> i32,
    csound_compile_csd: unsafe extern "C" fn(*mut CsoundPtr, *const c_char, i32, i32) -> i32,
    csound_start: unsafe extern "C" fn(*mut CsoundPtr) -> i32,
    csound_perform_ksmps: unsafe extern "C" fn(*mut CsoundPtr) -> i32,
    csound_set_control_channel: unsafe extern "C" fn(*mut CsoundPtr, *const c_char, f64) -> i32,
    csound_get_control_channel:
        unsafe extern "C" fn(*mut CsoundPtr, *const c_char, *mut c_int) -> f64,
}

unsafe impl Send for Csound {}
unsafe impl Sync for Csound {}

// TODO: rewrite functions to cache symbols(pointers to got functions)
impl Csound {
    pub fn new(filename: &Path) -> CsdResult<Csound> {
        unsafe {
            let lib = libloading::Library::new(filename)?;
            let csound_create: libloading::Symbol<
                unsafe extern "C" fn(*const c_void, *const c_void) -> *mut CsoundPtr,
            > = lib.get(b"csoundCreate")?;

            let csound_ptr = csound_create(null(), null());
            let csound_set_option = *lib.get(b"csoundSetOption")?;
            let csound_compile_csd = *lib.get(b"csoundCompileCSD")?;
            let csound_start = *lib.get(b"csoundStart")?;

            let csound_perform_ksmps = *lib.get(b"csoundPerformKsmps")?;
            let csound_set_control_channel = *lib.get(b"csoundSetControlChannel")?;
            let csound_get_control_channel = *lib.get(b"csoundGetControlChannel")?;
            Ok(Self {
                lib,
                csound_ptr,
                csound_set_option,
                csound_compile_csd,
                csound_start,
                csound_perform_ksmps,
                csound_set_control_channel,
                csound_get_control_channel,
            })
        }
        .map_err(CsdError)
    }

    pub fn set_option(&mut self, flags: &str) -> CsdResult<()> {
        unsafe {
            (self.csound_set_option)(self.csound_ptr, flags.as_ptr() as *const c_char);
            Ok(())
        }
        .map_err(CsdError)
    }
    pub fn compile_csd_from_str(&mut self, csd: &str, is_async: bool) -> Result<(), CsdError> {
        self.compile_csd_low_level(csd, 1, if is_async { 1 } else { 0 })
    }

    pub fn compile_csd_from_file(&mut self, csd: &str, is_async: bool) -> Result<(), CsdError> {
        self.compile_csd_low_level(csd, 0, if is_async { 1 } else { 0 })
    }

    fn compile_csd_low_level(
        &mut self,
        csd: &str,
        mode: i32,
        is_async: i32,
    ) -> Result<(), CsdError> {
        unsafe {
            (self.csound_compile_csd)(
                self.csound_ptr,
                csd.as_ptr() as *const c_char,
                mode,
                is_async,
            );
            Ok(())
        }
        .map_err(CsdError)
    }

    pub fn start(&mut self) -> CsdResult<()> {
        unsafe {
            (self.csound_start)(self.csound_ptr);
            Ok(())
        }
        .map_err(CsdError)
    }

    pub fn set_control_channel(&mut self, channel_name: &str, value: f64) -> Result<i32, CsdError> {
        unsafe {
            let res = (self.csound_set_control_channel)(
                self.csound_ptr,
                channel_name.as_ptr() as *const c_char,
                value,
            );
            Ok(res)
        }
        .map_err(CsdError)
    }

    pub fn get_control_channel(&mut self, channel_name: &str) -> Result<f64, CsdError> {
        unsafe {
            let err_value = 0;
            let res = (self.csound_get_control_channel)(
                self.csound_ptr,
                channel_name.as_ptr() as *const c_char,
                &err_value as *const i32 as *mut c_int,
            );
            Ok(res)
        }
        .map_err(CsdError)
    }

    pub fn perform_ksmps(&mut self) -> CsdResult<i32> {
        unsafe {
            let res = (self.csound_perform_ksmps)(self.csound_ptr);
            Ok(res)
        }
        .map_err(CsdError)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CsdError(libloading::Error);

type CsdResult<T> = Result<T, CsdError>;

impl From<libloading::Error> for CsdError {
    fn from(value: libloading::Error) -> Self {
        CsdError(value)
    }
}

// libloading example: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
static REVERB_CSD_FILE: &str = include_str!("./reverb.csd");
use core::ffi::c_void;
use std::path::Path;
use std::{
    ffi::{c_char, c_double, c_int},
    ptr::null,
};

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
    csound_reset: unsafe extern "C" fn(*mut CsoundPtr) -> c_void,
    csound_perform_ksmps: unsafe extern "C" fn(*mut CsoundPtr) -> i32,
    csound_set_control_channel: unsafe extern "C" fn(*mut CsoundPtr, *const c_char, f64) -> i32,
    csound_get_control_channel:
        unsafe extern "C" fn(*const CsoundPtr, *const c_char, *mut c_int) -> f64,
    csound_get_ksmps: unsafe extern "C" fn(*const CsoundPtr) -> u32,
    csound_get_spin: unsafe extern "C" fn(*mut CsoundPtr) -> *mut c_double,
    csound_get_spout: unsafe extern "C" fn(*const CsoundPtr) -> *mut c_double,
    csound_get_channels: unsafe extern "C" fn(*const CsoundPtr, i32) -> u32,
    csound_get_sr: unsafe extern "C" fn(*const CsoundPtr) -> u32,
    csound_get_kr: unsafe extern "C" fn(*const CsoundPtr) -> u32,
    csound_get_0dbfs: unsafe extern "C" fn(*const CsoundPtr) -> f64,
    csound_destroy: unsafe extern "C" fn(*mut CsoundPtr) -> c_void,
    csound_get_current_time_samples: unsafe extern "C" fn(*const CsoundPtr) -> i64,
    csound_event: unsafe extern "C" fn(*mut CsoundPtr, i32, *const f64, i32, i32) -> c_void,
    csound_event_string: unsafe extern "C" fn(*mut CsoundPtr, *const c_char, i32) -> c_void,
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
            let csound_reset = *lib.get(b"csoundReset")?;
            let csound_destroy = *lib.get(b"csoundDestroy")?;

            let csound_perform_ksmps = *lib.get(b"csoundPerformKsmps")?;
            let csound_set_control_channel = *lib.get(b"csoundSetControlChannel")?;
            let csound_get_control_channel = *lib.get(b"csoundGetControlChannel")?;
            let csound_get_ksmps = *lib.get(b"csoundGetKsmps")?;
            let csound_get_spin = *lib.get(b"csoundGetSpin")?;
            let csound_get_spout = *lib.get(b"csoundGetSpout")?;
            let csound_get_channels = *lib.get(b"csoundGetChannels")?;
            let csound_get_sr = *lib.get(b"csoundGetSr")?;
            let csound_get_kr = *lib.get(b"csoundGetKr")?;
            let csound_get_0dbfs = *lib.get(b"csoundGet0dBFS")?;
            let csound_get_current_time_samples = *lib.get(b"csoundGetCurrentTimeSamples")?;
            let csound_event = *lib.get(b"csoundEvent")?;
            let csound_event_string = *lib.get(b"csoundEventString")?;
            Ok(Self {
                lib,
                csound_ptr,
                csound_set_option,
                csound_compile_csd,
                csound_start,
                csound_reset,
                csound_destroy,
                csound_perform_ksmps,
                csound_set_control_channel,
                csound_get_control_channel,
                csound_get_ksmps,
                csound_get_spin,
                csound_get_spout,
                csound_get_channels,
                csound_get_sr,
                csound_get_kr,
                csound_get_current_time_samples,
                csound_get_0dbfs,
                csound_event,
                csound_event_string,
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

    pub fn reset(&mut self) -> CsdResult<()> {
        unsafe {
            (self.csound_reset)(self.csound_ptr);
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

    pub fn get_control_channel(&self, channel_name: &str) -> Result<f64, CsdError> {
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

    pub fn get_ksmps(&self) -> CsdResult<u32> {
        unsafe {
            let res = (self.csound_get_ksmps)(self.csound_ptr);
            Ok(res)
        }
        .map_err(CsdError)
    }

    pub fn get_sr(&self) -> CsdResult<u32> {
        unsafe {
            let res = (self.csound_get_sr)(self.csound_ptr);
            Ok(res)
        }
        .map_err(CsdError)
    }

    pub fn get_kr(&self) -> CsdResult<u32> {
        unsafe {
            let res = (self.csound_get_kr)(self.csound_ptr);
            Ok(res)
        }
        .map_err(CsdError)
    }

    pub fn get_0dbfs(&self) -> CsdResult<f64> {
        unsafe {
            let res = (self.csound_get_0dbfs)(self.csound_ptr);
            Ok(res)
        }
        .map_err(CsdError)
    }

    pub fn get_current_time_samples(&self) -> CsdResult<i64> {
        unsafe {
            let res = (self.csound_get_current_time_samples)(self.csound_ptr);
            Ok(res)
        }
        .map_err(CsdError)
    }

    pub fn get_channels(&self, is_input: bool) -> CsdResult<u32> {
        unsafe {
            let res = (self.csound_get_channels)(self.csound_ptr, is_input as i32);
            Ok(res)
        }
        .map_err(CsdError)
    }

    pub fn get_input_len(&self) -> CsdResult<u32> {
        self.get_channels(true)
    }
    pub fn get_output_len(&self) -> CsdResult<u32> {
        self.get_channels(false)
    }

    pub fn get_spin<'a>(&mut self) -> CsdResult<&'a mut [f64]> {
        unsafe {
            let res = (self.csound_get_spin)(self.csound_ptr);
            let ksmps = self.get_ksmps()?;
            let num_of_inputs = self.get_input_len()?;
            Ok(std::slice::from_raw_parts_mut(
                res,
                (ksmps * num_of_inputs) as usize,
            ))
        }
        .map_err(CsdError)
    }

    pub fn get_spout<'a>(&self) -> CsdResult<&'a [f64]> {
        unsafe {
            let res = (self.csound_get_spout)(self.csound_ptr);
            let ksmps = self.get_ksmps()?;
            let num_of_outputs = self.get_output_len()?;
            Ok(std::slice::from_raw_parts(
                res,
                (ksmps * num_of_outputs) as usize,
            ))
        }
        .map_err(CsdError)
    }

    pub fn destroy(&mut self) -> CsdResult<()> {
        unsafe {
            let _res = (self.csound_destroy)(self.csound_ptr);
            Ok(())
        }
        .map_err(CsdError)
    }

    pub fn event(
        &mut self,
        event_type: EventType,
        params: &[f64],
        is_async: bool,
    ) -> CsdResult<()> {
        unsafe {
            let _res = (self.csound_event)(
                self.csound_ptr,
                event_type as i32,
                params.as_ptr(),
                params.len() as i32,
                is_async as i32,
            );
            Ok(())
        }
        .map_err(CsdError)
    }

    pub fn event_string(&mut self, note: &str, is_async: bool) -> CsdResult<()> {
        unsafe {
            let _res = (self.csound_event_string)(
                self.csound_ptr,
                note.as_ptr() as *const c_char,
                is_async as i32,
            );
            Ok(())
        }
        .map_err(CsdError)
    }

    // unsafe and fast variants with no result/error wrappers

    pub fn get_ksmps_unsafe(&self) -> u32 {
        unsafe { (self.csound_get_ksmps)(self.csound_ptr) }
    }

    pub fn get_sr_unsafe(&self) -> u32 {
        unsafe { (self.csound_get_sr)(self.csound_ptr) }
    }

    pub fn get_kr_unsafe(&self) -> u32 {
        unsafe { (self.csound_get_kr)(self.csound_ptr) }
    }

    pub fn get_0dbfs_unsafe(&self) -> f64 {
        unsafe { (self.csound_get_0dbfs)(self.csound_ptr) }
    }

    pub fn get_channels_unsafe(&self, is_input: bool) -> u32 {
        unsafe { (self.csound_get_channels)(self.csound_ptr, is_input as i32) }
    }

    pub fn get_input_len_unsafe(&self) -> u32 {
        self.get_channels_unsafe(true)
    }
    pub fn get_output_len_unsafe(&self) -> u32 {
        self.get_channels_unsafe(false)
    }

    pub fn get_spin_unsafe<'a>(&mut self) -> &'a mut [f64] {
        unsafe {
            let res = (self.csound_get_spin)(self.csound_ptr);
            let ksmps = self.get_ksmps_unsafe();
            let num_of_inputs = self.get_input_len_unsafe();
            std::slice::from_raw_parts_mut(res, (ksmps * num_of_inputs) as usize)
        }
    }

    pub fn get_spout_unsafe<'a>(&self) -> &'a [f64] {
        unsafe {
            let res = (self.csound_get_spout)(self.csound_ptr);
            let ksmps = self.get_ksmps_unsafe();
            let num_of_outputs = self.get_output_len_unsafe();
            std::slice::from_raw_parts(res, (ksmps * num_of_outputs) as usize)
        }
    }

    pub fn perform_ksmps_unsafe(&mut self) -> i32 {
        unsafe { (self.csound_perform_ksmps)(self.csound_ptr) }
    }

    pub fn set_control_channel_unsafe(&mut self, channel_name: &str, value: f64) -> i32 {
        unsafe {
            (self.csound_set_control_channel)(
                self.csound_ptr,
                channel_name.as_ptr() as *const c_char,
                value,
            )
        }
    }

    // ignores errors
    pub fn get_control_channel_unsafe(&self, channel_name: &str) -> f64 {
        unsafe {
            let err_value = 0;
            (self.csound_get_control_channel)(
                self.csound_ptr,
                channel_name.as_ptr() as *const c_char,
                &err_value as *const i32 as *mut c_int,
            )
        }
    }

    pub fn event_unsafe(&mut self, event_type: EventType, params: &[f64], is_async: bool) {
        unsafe {
            let _res = (self.csound_event)(
                self.csound_ptr,
                event_type as i32,
                params.as_ptr(),
                params.len() as i32,
                is_async as i32,
            );
        }
    }

    pub fn event_string_unsafe(&mut self, note: &str, is_async: bool) {
        unsafe {
            let _res = (self.csound_event_string)(
                self.csound_ptr,
                note.as_ptr() as *const c_char,
                is_async as i32,
            );
        }
    }
}
enum EventType {
    Instrument = 0,
    FunctionTable = 1,
    End = 2,
}

impl Drop for Csound {
    fn drop(&mut self) {
        let _ = self.destroy();
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

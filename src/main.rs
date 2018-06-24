use std::ffi::CString;
use std::ffi::CStr;

use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access, State};

#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate alsa;

use docopt::Docopt;

mod ffi;


pub struct Porcupine {
    pub  porcupine_instance: *mut ffi::pv_porcupine_object_t,
}

impl Porcupine {

    pub fn new(model_file_path: String,keyword_file_path : String, sensitivity : f32) -> Result<Self, String> {

        let mut instance = ffi::get_instance();
        let c_model_file_path = CString::new(model_file_path).unwrap();
        let c_keyword_file_path = CString::new(keyword_file_path).unwrap();
        let mut porcupine_ptr: *mut ffi::pv_porcupine_object_t = &mut instance;

        unsafe {
            match ffi::pv_porcupine_init(c_model_file_path.as_ptr(),c_keyword_file_path.as_ptr(),sensitivity,&mut porcupine_ptr) {
                ffi::pv_status_t::PV_STATUS_SUCCESS => Ok(Porcupine{porcupine_instance: porcupine_ptr}),
                ffi::pv_status_t::PV_STATUS_INVALID_ARGUMENT =>  Err("Invalid Argument".to_string()),
                ffi::pv_status_t::PV_STATUS_IO_ERROR =>  Err("Status IO Error".to_string()),
                _ => Err("Failed to init porcupine".to_string()),
            }
        }
    }

    pub fn version() -> String {
        unsafe {
            CStr::from_ptr(ffi::pv_porcupine_version()).to_string_lossy().into_owned()
        }
    }

    pub fn pv_porcupine_frame_length() -> u32 {
        unsafe {
            ffi::pv_porcupine_frame_length() as u32
        }
    }
    
    pub fn pv_sample_rate() -> u32 {
        unsafe {
            ffi::pv_sample_rate() as u32
        }
    }

    pub fn pv_porcupine_process(&mut self,pcm: &[i16]) -> Result<bool, String> {
        let mut result: bool = false;
        let mut ptr: *const i16 = pcm.as_ptr();
        unsafe {
            match ffi::pv_porcupine_process(self.porcupine_instance,ptr, &mut result) {
                ffi::pv_status_t::PV_STATUS_SUCCESS => Ok(result),
                _ => Err("Failed to init porcupine".to_string()),
            }
        }
    }
}



#[derive(Debug, Deserialize)]
struct Args {
    flag_keyword_file_path: String,
    flag_model_file_path: String,
}

const USAGE: &str = "
Porcupine Zero

Usage:
  PorcupineZero [--keyword-file-path=<alsa-device> --model-file-path=<alsa-device>]
  PorcupineZero (-h | --help)

Options:
  -h --help                         Show this screen.
  --keyword-file-path=<Path>    Path to keyword file [default: ./resources/alexa_raspberrypi.ppn]
  --model-file-path=<Path>      Path to model file   [default: ./model/porcupine_params.pv]
";



fn main() {


    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("Keyword file path: {}\n  Model file path:    {}\n  ",
              args.flag_keyword_file_path,
              args.flag_model_file_path);

    let keyword_file_path = args.flag_keyword_file_path;
    let model_file_path = args.flag_model_file_path;
    let mut pinstance = Porcupine::new(model_file_path,keyword_file_path,0.8).unwrap();


    let pcm = PCM::open(&*CString::new("default").unwrap(), Direction::Capture, false).unwrap();
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(16000, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
    pcm.start().unwrap();

    let io_capture = pcm.io_i16().unwrap();
    let mut buffer: [i16; 512] = [0;512];

    loop {

        io_capture.readi(&mut buffer).unwrap();

        match pinstance.pv_porcupine_process(&mut buffer) {
             Ok(true) => eprintln!("Keyword found"),
             Ok(false) => {},
             Err(err) => eprintln!("Error"),
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pv_porcupine_init_test(){
        assert_eq!(Porcupine::version(),"1.3.0");
        assert_eq!(Porcupine::pv_porcupine_frame_length(),512);
        assert_eq!(Porcupine::pv_sample_rate(),16000);
    }
}




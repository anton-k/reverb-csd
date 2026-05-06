use csound::Csound;
static REVERB_CSD_FILE: &str = include_str!("./reverb.csd");
use clack_plugin::prelude::ClapId;

pub fn init_csound(sample_rate: usize) -> Csound {
    let csound = Csound::new().expect("Failed to init Csound");
    csound
        .set_option(format!("-sr {}", sample_rate).as_str())
        .expect("Failed to set sample rate");
    csound.compile_csd(REVERB_CSD_FILE, 0, 1).unwrap();
    csound.start().expect("Failed to start Csound");
    csound
}

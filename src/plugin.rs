use clack_extensions::{audio_ports::*, gui::PluginGui, params::PluginParams, state::PluginState};
use clack_plugin::plugin::features;
use clack_plugin::prelude::*;
pub mod audio_processor;
pub mod gui;
pub mod main_thread;
pub mod params;
pub mod shared;

pub struct Reverb;
use audio_processor::ReverbAudioProcessor;
use main_thread::ReverbMainThread;
use shared::ReverbShared;

impl Plugin for Reverb {
    type AudioProcessor<'a> = ReverbAudioProcessor<'a>;
    type Shared<'a> = ReverbShared;
    type MainThread<'a> = ReverbMainThread<'a>;

    fn declare_extensions(
        builder: &mut PluginExtensions<Self>,
        _shared: Option<&Self::Shared<'_>>,
    ) {
        builder
            .register::<PluginAudioPorts>()
            .register::<PluginParams>()
            .register::<PluginGui>()
            .register::<PluginState>();
    }
}

impl DefaultPluginFactory for Reverb {
    fn get_descriptor() -> PluginDescriptor {
        PluginDescriptor::new("csound.com", "ReverbCsd")
            .with_vendor("Reverb implemented with Csound")
            .with_features([features::AUDIO_EFFECT, features::STEREO])
    }

    fn new_shared(_host: HostSharedHandle) -> Result<Self::Shared<'_>, PluginError> {
        Ok(ReverbShared::default())
    }

    fn new_main_thread<'a>(
        _host: HostMainThreadHandle<'a>,
        shared: &'a Self::Shared<'a>,
    ) -> Result<Self::MainThread<'a>, PluginError> {
        Ok(Self::MainThread { shared, gui: None })
    }
}

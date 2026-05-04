use crate::plugin::{
    audio_processor::ReverbAudioProcessor, main_thread::ReverbMainThread, shared::ReverbShared,
};
use clack_extensions::{audio_ports::*, params::*, state::PluginState};
use clack_plugin::plugin::features;
use clack_plugin::prelude::*;

mod plugin;
mod util;

pub struct Reverb;

impl Plugin for Reverb {
    type AudioProcessor<'a> = ReverbAudioProcessor<'a>;
    type Shared<'a> = ReverbShared;
    type MainThread<'a> = ReverbMainThread<'a>;

    fn declare_extensions(builder: &mut PluginExtensions<Self>, shared: Option<&Self::Shared<'_>>) {
        builder
            .register::<PluginAudioPorts>()
            //            .register::<PluginParams>()
            .register::<PluginState>();
    }
}

impl DefaultPluginFactory for Reverb {
    fn get_descriptor() -> PluginDescriptor {
        PluginDescriptor::new("csound.com", "ReverbCsd")
            .with_vendor("Reverb implemented with Csound")
            .with_features([features::AUDIO_EFFECT, features::STEREO])
    }

    fn new_shared(host: HostSharedHandle) -> Result<Self::Shared<'_>, PluginError> {
        Ok(ReverbShared::default())
    }

    fn new_main_thread<'a>(
        host: HostMainThreadHandle<'a>,
        shared: &'a Self::Shared<'a>,
    ) -> Result<Self::MainThread<'a>, PluginError> {
        Ok(Self::MainThread { shared })
    }
}

clack_export_entry!(SinglePluginEntry<Reverb>);

use crate::plugin::main_thread::ReverbMainThread;
use crate::plugin::shared::ReverbShared;
use clack_plugin::prelude::*;

pub struct ReverbAudioProcessor<'a> {
    shared: &'a ReverbShared,
}

impl<'a> PluginAudioProcessor<'a, ReverbShared, ReverbMainThread<'a>> for ReverbAudioProcessor<'a> {
    fn activate(
        host: HostAudioProcessorHandle<'a>,
        main_thread: &mut ReverbMainThread<'a>,
        shared: &'a ReverbShared,
        audio_config: PluginAudioConfiguration,
    ) -> Result<Self, PluginError> {
        todo!()
    }

    fn process(
        &mut self,
        process: Process,
        audio: Audio,
        events: Events,
    ) -> Result<ProcessStatus, PluginError> {
        todo!()
    }
}

use crate::plugin::audio::csd;
use crate::plugin::main_thread::ReverbMainThread;
use crate::plugin::params::ReverbParamsShared;
use crate::plugin::shared::ReverbShared;
use clack_extensions::params::PluginAudioProcessorParams;
use clack_plugin::events::spaces::CoreEventSpace;
use clack_plugin::prelude::*;
use csound::Csound;

pub struct ReverbAudioProcessor<'a> {
    shared: &'a ReverbShared,
    csound: Csound,
}

impl<'a> PluginAudioProcessor<'a, ReverbShared, ReverbMainThread<'a>> for ReverbAudioProcessor<'a> {
    fn activate(
        _host: HostAudioProcessorHandle<'a>,
        _main_thread: &mut ReverbMainThread<'a>,
        shared: &'a ReverbShared,
        audio_config: PluginAudioConfiguration,
    ) -> Result<Self, PluginError> {
        let csound = csd::init_csound(audio_config.sample_rate.floor() as usize);
        Ok(Self { shared, csound })
    }

    fn process(
        &mut self,
        process: Process,
        audio: Audio,
        events: Events,
    ) -> Result<ProcessStatus, PluginError> {
        Ok(ProcessStatus::ContinueIfNotQuiet)
    }
}

impl<'a> PluginAudioProcessorParams for ReverbAudioProcessor<'a> {
    fn flush(
        &mut self,
        input_parameter_changes: &InputEvents,
        output_parameter_changes: &mut OutputEvents,
    ) {
        println!("HI FLUSH");
        for event in input_parameter_changes {
            if let Some(CoreEventSpace::ParamValue(event)) = event.as_core_event()
                && let Some(param_id) = event.param_id()
                && let Some(param) = param_id_to_name(param_id)
            {
                println!("Param change: {:?} {:.2?}", param, event.value());
                self.csound
                    .set_control_channel(param, event.value())
                    .unwrap();
            }
        }
    }
}

fn param_id_to_name(id: ClapId) -> Option<&'static str> {
    if id == ReverbParamsShared::FEEDBACK_ID {
        Some("feedback")
    } else if id == ReverbParamsShared::CUT_OFF_ID {
        Some("cut_off")
    } else if id == ReverbParamsShared::MIX_ID {
        Some("mix")
    } else {
        None
    }
}

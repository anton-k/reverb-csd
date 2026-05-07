use crate::plugin::audio::csd;
use crate::plugin::main_thread::ReverbMainThread;
use crate::plugin::params;
use crate::plugin::shared::ReverbShared;
use clack_extensions::params::PluginAudioProcessorParams;
use clack_plugin::events::spaces::CoreEventSpace;
use clack_plugin::prelude::*;
// use csound::Csound;

pub struct ReverbAudioProcessor<'a> {
    shared: &'a ReverbShared,
    csound: csd::Csound,
}

impl<'a> ReverbAudioProcessor<'a> {
    pub fn handle_events(&mut self, events: &Events) {
        for event_batch in events.input.batch() {
            for event in event_batch.events() {
                react_on_input_event(event, &mut self.csound);
            }
        }
    }

    pub fn update_csound_params(&mut self) {
        self.shared
            .params
            .params
            .iter()
            .enumerate()
            .for_each(|(index, param)| {
                let _ = param.on_update(|| {
                    self.csound
                        .set_control_channel(params::PARAMS[index].name, param.get() as f64);
                });
            });
    }
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
        _process: Process,
        _audio: Audio,
        events: Events,
    ) -> Result<ProcessStatus, PluginError> {
        self.handle_events(&events);
        self.update_csound_params();

        let _ = self.csound.perform_ksmps().unwrap();

        Ok(ProcessStatus::ContinueIfNotQuiet)
    }
}

impl<'a> PluginAudioProcessorParams for ReverbAudioProcessor<'a> {
    fn flush(
        &mut self,
        input_parameter_changes: &InputEvents,
        _output_parameter_changes: &mut OutputEvents,
    ) {
        for event in input_parameter_changes {
            react_on_input_event(event, &mut self.csound);
        }
    }
}

fn react_on_input_event(event: &UnknownEvent, csound: &mut csd::Csound) {
    if let Some(CoreEventSpace::ParamValue(event)) = event.as_core_event()
        && let Some(param_id) = event.param_id()
        && let Some(param) = params::param_id_to_name(param_id)
    {
        println!("Param change: {:?} {:.2?}", param, event.value());
        csound.set_control_channel(param, event.value()).unwrap();
    }
}

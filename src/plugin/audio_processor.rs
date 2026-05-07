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
        self.shared.params.on_feedback_updated(|| {
            self.csound
                .set_control_channel(
                    params::FEEDBACK_NAME,
                    self.shared.params.get_feedback() as f64,
                )
                .unwrap();
        });
        self.shared.params.on_cut_off_updated(|| {
            self.csound
                .set_control_channel(
                    params::CUT_OFF_NAME,
                    self.shared.params.get_cut_off() as f64,
                )
                .unwrap();
        });

        self.shared.params.on_mix_updated(|| {
            self.csound
                .set_control_channel(params::MIX_NAME, self.shared.params.get_mix() as f64)
                .unwrap();
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
        process: Process,
        audio: Audio,
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

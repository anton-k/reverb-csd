use crate::plugin::gui::ReverbGui;
use crate::plugin::params::*;
use crate::plugin::shared::ReverbShared;
use clack_extensions::gui::{GuiApiType, Window};
use clack_extensions::{audio_ports::*, gui::{PluginGuiImpl, GuiSize, GuiConfiguration}, params::*, state::PluginStateImpl};
use clack_plugin::prelude::*;
use clack_plugin::stream::{InputStream, OutputStream};
use std::fmt::Write as _;
use std::io::{Read, Write as _};

pub struct ReverbMainThread<'a> {
    pub shared: &'a ReverbShared,
    pub gui: Option<ReverbGui>,
}

impl<'a> PluginMainThread<'a, ReverbShared> for ReverbMainThread<'a> {}

impl PluginStateImpl for ReverbMainThread<'_> {
    fn save(&mut self, output: &mut OutputStream) -> Result<(), PluginError> {
        output.write_all(&self.shared.params.serialize())?;
        Ok(())
    }
    fn load(&mut self, input: &mut InputStream) -> Result<(), PluginError> {
        let mut buf: Vec<u8> = Vec::new();
        input.read_exact(&mut buf)?;
        let params = ReverbParams::deserialize(&buf);
        self.shared.params.set_feedback(params.get_feedback());
        self.shared.params.set_cut_off(params.get_cut_off());
        self.shared.params.set_mix(params.get_mix());
        Ok(())
    }
}

impl PluginAudioPortsImpl for ReverbMainThread<'_> {
    fn count(&mut self, _is_input: bool) -> u32 {
        1
    }

    fn get(&mut self, index: u32, _is_input: bool, writer: &mut AudioPortInfoWriter) {
        if index == 0 {
            writer.set(&AudioPortInfo {
                id: ClapId::new(0),
                name: b"main",
                channel_count: 2,
                flags: AudioPortFlags::IS_MAIN,
                port_type: Some(AudioPortType::STEREO),
                in_place_pair: None,
            });
        }
    }
}

impl PluginMainThreadParams for ReverbMainThread<'_> {
    fn count(&mut self) -> u32 {
        3
    }

    fn get_info(&mut self, param_index: u32, info: &mut ParamInfoWriter) {
        if param_index == 0 {
            info.set(&unit_param_info(
                PARAM_FEEDBACK_ID,
                b"Feedback",
                DEFAULT_FEEDBACK,
            ));
        } else if param_index == 1 {
            info.set(&unit_param_info(
                PARAM_CUT_OFF_ID,
                b"Cut off",
                DEFAULT_CUT_OFF,
            ));
        } else if param_index == 2 {
            info.set(&unit_param_info(PARAM_MIX_ID, b"Mix", DEFAULT_MIX));
        }
    }

    fn get_value(&mut self, param_id: ClapId) -> Option<f64> {
        if param_id == PARAM_FEEDBACK_ID {
            Some(self.shared.params.get_feedback() as f64)
        } else if param_id == PARAM_CUT_OFF_ID {
            Some(self.shared.params.get_cut_off() as f64)
        } else if param_id == PARAM_MIX_ID {
            Some(self.shared.params.get_mix() as f64)
        } else {
            None
        }
    }

    fn value_to_text(
        &mut self,
        _param_id: ClapId,
        value: f64,
        writer: &mut ParamDisplayWriter,
    ) -> core::fmt::Result {
        write!(writer, "{0:.2}", value)
    }

    fn text_to_value(&mut self, param_id: ClapId, text: &std::ffi::CStr) -> Option<f64> {
        if is_valid_param(param_id) {
            let text = text.to_str().ok()?;
            text.trim().parse().ok()
        } else {
            None
        }
    }

    fn flush(
        &mut self,
        input_parameter_changes: &InputEvents,
        _output_parameter_changes: &mut OutputEvents,
    ) {
        for event in input_parameter_changes {
            self.shared.params.handle_event(event)
        }
    }
}

fn is_valid_param(param_id: ClapId) -> bool {
    param_id >= PARAM_FEEDBACK_ID && param_id <= PARAM_MIX_ID
}

fn unit_param_info(id: ClapId, name: &[u8], init: f32) -> ParamInfo {
    ParamInfo {
        id,
        flags: ParamInfoFlags::IS_AUTOMATABLE,
        cookie: Default::default(),
        name,
        module: b"",
        min_value: 0.0,
        max_value: 1.0,
        default_value: init as f64,
    }
}

impl<'a> PluginGuiImpl for ReverbMainThread<'a> {
    fn is_api_supported(&mut self, configuration: GuiConfiguration) -> bool {
        configuration.api_type
            == GuiApiType::default_for_current_platform().expect("Unsupported platform")
            && !configuration.is_floating
    }

    fn get_preferred_api(&mut self) -> Option<GuiConfiguration<'_>> {
        Some(GuiConfiguration {
            api_type: GuiApiType::default_for_current_platform().expect("Unsupported platform"),
            is_floating: false,
        })
    }

    fn create(&mut self, configuration: GuiConfiguration) -> Result<(), PluginError> {
        if configuration.is_floating {
            return Err(PluginError::Message(
                "Invalid GUI configuration: this plugin does not support floating mode",
            ));
        }

        let supported_type =
            GuiApiType::default_for_current_platform().expect("Unsupported platform");

        if configuration.api_type != supported_type {
            return Err(PluginError::Message(
                "Invalid GUI configuration: unsupported API type",
            ));
        }

        Ok(())
    }

    fn destroy(&mut self) {
        let _ = self.gui.take();
    }

    fn set_scale(&mut self, _scale: f64) -> Result<(), PluginError> {
        Ok(())
    }

    fn get_size(&mut self) -> Option<GuiSize> {
        Some(GuiSize {
            width: 400,
            height: 200,
        })
    }

    fn set_size(&mut self, _size: GuiSize) -> Result<(), PluginError> {
        Ok(())
    }

    fn set_parent(&mut self, window: Window) -> Result<(), PluginError> {
        self.gui = Some(ReverbGui::new(window, self.shared));
        Ok(())
    }

    fn set_transient(&mut self, _window: Window) -> Result<(), PluginError> {
        Ok(())
    }

    fn show(&mut self) -> Result<(), PluginError> {
        if let Some(gui) = &self.gui {
            gui.request_repaint()
        }
        Ok(())
    }

    fn hide(&mut self) -> Result<(), PluginError> {
        if let Some(gui) = &self.gui {
            gui.request_repaint()
        }

        Ok(())
    }
}


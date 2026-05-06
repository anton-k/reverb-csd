use crate::util::atomic::AtomicF32;
use clack_plugin::events::spaces::CoreEventSpace;
use clack_plugin::prelude::*;
use std::sync::atomic::Ordering;

pub static DEFAULT_FEEDBACK: f32 = 0.6;
pub static DEFAULT_CUT_OFF: f32 = 1.0;
pub static DEFAULT_MIX: f32 = 1.0;

pub struct ReverbParamsShared {
    feedback: AtomicF32,
    cut_off: AtomicF32,
    mix: AtomicF32,
}

impl ReverbParamsShared {
    pub const FEEDBACK_ID: ClapId = ClapId::new(1);
    pub const CUT_OFF_ID: ClapId = ClapId::new(2);
    pub const MIX_ID: ClapId = ClapId::new(3);

    pub fn is_valid_param(param_id: ClapId) -> bool {
        param_id >= ReverbParamsShared::FEEDBACK_ID && param_id <= ReverbParamsShared::MIX_ID
    }

    #[inline]
    pub fn set_params_from_local(&mut self, local: &ReverbParamsLocal) {
        self.set_feedback(local.get_feedback());
        self.set_cut_off(local.get_cut_off());
        self.set_mix(local.get_mix());
    }

    #[inline]
    pub fn get_feedback(&self) -> f32 {
        self.feedback.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn get_cut_off(&self) -> f32 {
        self.cut_off.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn get_mix(&self) -> f32 {
        self.mix.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn set_feedback(&self, new_feedback: f32) {
        self.feedback
            .store(new_feedback.clamp(0.0, 1.0), Ordering::SeqCst)
    }

    #[inline]
    pub fn set_cut_off(&self, new_cut_off: f32) {
        self.feedback
            .store(new_cut_off.clamp(0., 1.), Ordering::SeqCst)
    }

    #[inline]
    pub fn set_mix(&self, new_mix: f32) {
        self.feedback.store(new_mix.clamp(0., 1.), Ordering::SeqCst)
    }
}

impl Default for ReverbParamsShared {
    fn default() -> Self {
        ReverbParamsShared {
            feedback: AtomicF32::from(DEFAULT_FEEDBACK),
            cut_off: AtomicF32::from(DEFAULT_CUT_OFF),
            mix: AtomicF32::from(DEFAULT_MIX),
        }
    }
}

pub struct ReverbParamsLocal {
    feedback: f32,
    cut_off: f32,
    mix: f32,
}

impl ReverbParamsLocal {
    pub fn new(shared: &ReverbParamsShared) -> Self {
        ReverbParamsLocal {
            feedback: shared.get_feedback(),
            cut_off: shared.get_cut_off(),
            mix: shared.get_mix(),
        }
    }

    #[inline]
    pub fn get_feedback(&self) -> f32 {
        self.feedback
    }

    #[inline]
    pub fn set_feedback(&mut self, new_feedback: f32) {
        self.feedback = new_feedback.clamp(0.0, 1.0);
    }

    pub fn push_feedback_updates(&self, shared: &ReverbParamsShared) -> bool {
        let previous_value = shared.feedback.swap(self.feedback);
        previous_value != self.feedback
    }

    #[inline]
    pub fn get_cut_off(&self) -> f32 {
        self.cut_off
    }

    #[inline]
    pub fn set_cut_off(&mut self, new_cut_off: f32) {
        self.cut_off = new_cut_off.clamp(0.0, 1.0);
    }

    pub fn push_cut_off_updates(&self, shared: &ReverbParamsShared) -> bool {
        let previous_value = shared.cut_off.swap(self.cut_off);
        previous_value != self.cut_off
    }

    #[inline]
    pub fn get_mix(&self) -> f32 {
        self.mix
    }

    #[inline]
    pub fn set_mix(&mut self, new_mix: f32) {
        self.mix = new_mix.clamp(0.0, 1.0);
    }

    pub fn push_mix_updates(&self, shared: &ReverbParamsShared) -> bool {
        let previous_value = shared.mix.swap(self.mix);
        previous_value != self.mix
    }

    #[inline]
    pub fn fetch_updates(&mut self, shared: &ReverbParamsShared) -> bool {
        let mut res = false;

        let latest_feedback = shared.feedback.load(Ordering::Relaxed);
        if latest_feedback != self.feedback {
            self.feedback = latest_feedback;
            res = true;
        }
        let latest_cut_off = shared.cut_off.load(Ordering::Relaxed);
        if latest_cut_off != self.cut_off {
            self.cut_off = latest_cut_off;
            res = true;
        }
        let latest_mix = shared.mix.load(Ordering::Relaxed);
        if latest_mix != self.mix {
            self.mix = latest_mix;
            res = true;
        }
        res
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let data = vec![self.get_feedback(), self.get_cut_off(), self.get_feedback()];
        bincode_next::encode_into_slice(data, &mut res, bincode_next::config::standard()).unwrap();
        res
    }

    pub fn deserialize(src: &[u8]) -> Self {
        let (data, _) = bincode_next::decode_from_slice::<
            Vec<f32>,
            bincode_next::config::Configuration,
        >(src, bincode_next::config::standard())
        .unwrap();
        let feedback = data[0];
        let cut_off = data[1];
        let mix = data[2];
        ReverbParamsLocal {
            feedback,
            cut_off,
            mix,
        }
    }

    pub fn handle_event(&mut self, event: &UnknownEvent) {
        if let Some(CoreEventSpace::ParamValue(event)) = event.as_core_event() {
            if event.param_id() == ReverbParamsShared::FEEDBACK_ID {
                self.set_feedback(event.value() as f32)
            } else if event.param_id() == ReverbParamsShared::CUT_OFF_ID {
                self.set_cut_off(event.value() as f32)
            } else if event.param_id() == ReverbParamsShared::MIX_ID {
                self.set_mix(event.value() as f32)
            }
        }
    }
}

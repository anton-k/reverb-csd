use crate::util::atomic::AtomicF32;
use clack_plugin::events::spaces::CoreEventSpace;
use clack_plugin::prelude::*;
use std::sync::atomic::Ordering;

pub static DEFAULT_FEEDBACK: f32 = 0.6;
pub static DEFAULT_CUT_OFF: f32 = 1.0;
pub static DEFAULT_MIX: f32 = 1.0;
pub const PARAM_FEEDBACK_ID: ClapId = ClapId::new(1);
pub const PARAM_CUT_OFF_ID: ClapId = ClapId::new(2);
pub const PARAM_MIX_ID: ClapId = ClapId::new(3);
use bincode_next::enc::Encode;

pub struct ReverbParams {
    feedback: AtomicF32,
    cut_off: AtomicF32,
    mix: AtomicF32,
}

impl ReverbParams {
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

    pub fn handle_event(&self, event: &UnknownEvent) {
        if let Some(CoreEventSpace::ParamValue(event)) = event.as_core_event() {
            if event.param_id() == PARAM_FEEDBACK_ID {
                self.set_feedback(event.value() as f32)
            } else if event.param_id() == PARAM_CUT_OFF_ID {
                self.set_cut_off(event.value() as f32)
            } else if event.param_id() == PARAM_MIX_ID {
                self.set_mix(event.value() as f32)
            }
        }
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
        let feedback = AtomicF32::from(data[0]);
        let cut_off = AtomicF32::from(data[1]);
        let mix = AtomicF32::from(data[2]);
        ReverbParams {
            feedback,
            cut_off,
            mix,
        }
    }
}

impl Default for ReverbParams {
    fn default() -> Self {
        ReverbParams {
            feedback: AtomicF32::from(DEFAULT_FEEDBACK),
            cut_off: AtomicF32::from(DEFAULT_CUT_OFF),
            mix: AtomicF32::from(DEFAULT_MIX),
        }
    }
}

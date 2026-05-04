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

pub struct ReverbParams {
    feedback: AtomicF32,
    cut_off: AtomicF32,
    mix: AtomicF32,
}

impl ReverbParams {
    #[inline]
    pub fn get_volume(&self) -> f32 {
        self.feedback.load(Ordering::SeqCst)
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

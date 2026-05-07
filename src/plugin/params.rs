use crate::plugin::audio::csd::ChannelName;
use crate::util::atomic::AtomicValue;
use clack_plugin::events::spaces::CoreEventSpace;
use clack_plugin::prelude::*;
use once_cell::sync::Lazy;

pub static PARAMS: [ParamSpec; 3] = [
    ParamSpec {
        ui_name: "size",
        id: ClapId::new(0),
        init: 0.6,
    },
    ParamSpec {
        ui_name: "filter",
        id: ClapId::new(1),
        init: 1.0,
    },
    ParamSpec {
        ui_name: "mix",
        id: ClapId::new(2),
        init: 1.0,
    },
];

// We use special names because ChannelName is Cstring under the hood,
// and to avoid allocation in audio performance loop we need to append termination
// character at the end of string (channel name) to use it in FFI call without extra allocations
// on every call.
pub static CHANNEL_NAMES: Lazy<[ChannelName; 3]> = Lazy::new(|| {
    [
        ChannelName::from("feedback"),
        ChannelName::from("cut_off"),
        ChannelName::from("mix"),
    ]
});

pub struct ParamSpec {
    pub ui_name: &'static str,
    pub id: ClapId,
    pub init: f32,
}

pub struct ReverbParamsShared {
    pub params: Vec<Param>,
}

impl ReverbParamsShared {
    pub fn set(&self, index: usize, value: f32) {
        self.params[index].value.store(value);
    }
}

pub struct Param {
    value: AtomicValue,
}

impl From<f32> for Param {
    fn from(value: f32) -> Self {
        Param {
            value: AtomicValue::from(value),
        }
    }
}

impl Param {
    #[inline]
    pub fn on_update<F: FnOnce()>(&self, run: F) -> bool {
        self.value.call_on_update(run)
    }

    #[inline]
    pub fn get(&self) -> f32 {
        self.value.load()
    }

    #[inline]
    pub fn set(&self, new_feedback: f32) {
        self.value.store(new_feedback.clamp(0.0, 1.0))
    }
}

impl ReverbParamsShared {
    pub fn is_valid_param(&self, param_id: ClapId) -> bool {
        param_id.get() < self.params.len() as u32
    }

    pub fn push(&mut self, param: Param) {
        self.params.push(param);
    }
}

impl Default for ReverbParamsShared {
    fn default() -> Self {
        let params = PARAMS.iter().map(|p| Param::from(p.init)).collect();
        ReverbParamsShared { params }
    }
}

pub struct ReverbParamsLocal {
    pub params: [f32; 3],
}

impl ReverbParamsLocal {
    pub fn new(shared: &ReverbParamsShared) -> Self {
        let mut params = [0.0; 3];
        for (i, param) in shared.params.iter().enumerate() {
            params[i] = param.get()
        }
        ReverbParamsLocal { params }
    }

    #[inline]
    pub fn get(&self, index: usize) -> f32 {
        self.params[index]
    }

    #[inline]
    pub fn set(&mut self, index: usize, new_feedback: f32) {
        self.params[index] = new_feedback.clamp(0.0, 1.0);
    }

    pub fn push_updates(&self, index: usize, shared: &ReverbParamsShared) -> bool {
        let previous_value = shared.params[index].value.swap(self.params[index]);
        previous_value != self.params[index]
    }

    #[inline]
    pub fn fetch_updates(&mut self, shared: &ReverbParamsShared) -> bool {
        let mut res = false;
        for (i, param) in shared.params.iter().enumerate() {
            let latest = param.value.load();
            if latest != self.params[i] {
                self.params[i] = latest;
                res = true;
            }
        }
        res
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        bincode_next::encode_into_slice(self.params, &mut res, bincode_next::config::standard())
            .unwrap();
        res
    }

    pub fn deserialize(src: &[u8]) -> Self {
        let (params, _) = bincode_next::decode_from_slice::<
            [f32; 3],
            bincode_next::config::Configuration,
        >(src, bincode_next::config::standard())
        .unwrap();

        ReverbParamsLocal { params }
    }

    pub fn handle_event(&mut self, event: &UnknownEvent) {
        if let Some(CoreEventSpace::ParamValue(event)) = event.as_core_event()
            && let Some(id) = event.param_id()
        {
            self.set_by_id(id, event.value() as f32);
        }
    }

    pub fn set_by_id(&mut self, id: ClapId, value: f32) {
        let index = id.get() as usize;
        if index < self.params.len() {
            self.params[index] = value;
        }
    }

    pub fn get_by_id(&self, id: ClapId) -> Option<f32> {
        let index = id.get() as usize;
        if index < self.params.len() {
            Some(self.params[index])
        } else {
            None
        }
    }
    pub fn is_valid_param(&self, id: ClapId) -> bool {
        let index = id.get() as usize;
        index < self.params.len()
    }
}

pub fn param_id_to_name<'a>(id: ClapId) -> Option<&'a ChannelName> {
    CHANNEL_NAMES.get(id.get() as usize)
}

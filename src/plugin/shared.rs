use crate::plugin::params::ReverbParamsShared;
use clack_plugin::prelude::*;
use std::sync::Arc;

#[derive(Default)]
pub struct ReverbShared {
    pub params: Arc<ReverbParamsShared>,
}

impl<'a> PluginShared<'a> for ReverbShared {}

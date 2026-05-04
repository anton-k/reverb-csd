use crate::plugin::params::ReverbParams;
use clack_plugin::prelude::*;

#[derive(Default)]
pub struct ReverbShared {
    pub params: ReverbParams,
}

impl<'a> PluginShared<'a> for ReverbShared {}

use clack_plugin::prelude::*;

mod plugin;
mod util;
use crate::plugin::Reverb;

clack_export_entry!(SinglePluginEntry<Reverb>);
clap_wrapper::export_vst3!();

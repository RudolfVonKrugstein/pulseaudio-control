use libpulse_binding::context::introspect::SourceInfo;
use libpulse_binding::def;
use libpulse_binding::volume::{ChannelVolumes, Volume};

#[derive(Debug)]
pub struct Source {
    /// Name of the source.
    pub name: Option<String>,
    /// Index of the source.
    pub index: u32,
    /// Description of this source.
    pub description: Option<String>,
    /// Volume of the source.
    pub volume: ChannelVolumes,
    /// Mute switch of the source.
    pub mute: bool,
    /// Some kind of “base” volume that refers to unamplified/unattenuated volume in the context of
    /// the input device.
    pub base_volume: Volume,
    /// State.
    pub state: def::SourceState,
    /// Number of volume steps for sources which do not support arbitrary volumes.
    pub n_volume_steps: u32,
}

impl Source {
   pub fn from_source_info(i: &SourceInfo) -> Source {
       Source {
           name: i.name.as_ref().map(|s| s.to_string()),
           index: i.index,
           description: i.description.as_ref().map(|s| s.to_string()),
           volume: i.volume,
           mute: i.mute,
           base_volume: i.base_volume,
           state: i.state,
           n_volume_steps: i.n_volume_steps,
       }
   }
}

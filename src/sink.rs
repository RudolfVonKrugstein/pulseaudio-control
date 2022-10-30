use libpulse_binding::context::introspect::SinkInfo;
use libpulse_binding::def;
use libpulse_binding::volume::{ChannelVolumes, Volume};

/** Part of the Sink, we actually care about*/
#[derive(Debug)]
pub struct Sink {
    /// Name of the sink.
    pub name: Option<String>,
    /// Index of the sink.
    pub index: u32,
    /// Description of this sink.
    pub description: Option<String>,
    /// Volume of the sink.
    pub volume: ChannelVolumes,
    /// Mute switch of the sink.
    pub mute: bool,
    /// Some kind of “base” volume that refers to unamplified/unattenuated volume in the context of
    /// the output device.
    pub base_volume: Volume,
    /// State.
    pub state: def::SinkState,
    /// Number of volume steps for sinks which do not support arbitrary volumes.
    pub n_volume_steps: u32,
}

impl Sink {
    pub fn from_sink_info(i: &SinkInfo) -> Sink {
        Sink {
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

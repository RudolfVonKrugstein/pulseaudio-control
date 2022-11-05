use libpulse_binding::context::introspect::SinkInfo;
use libpulse_binding::def;
use libpulse_binding::def::SinkState;
use libpulse_binding::volume::{ChannelVolumes, Volume};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

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

impl Serialize for Sink {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("sink", 8)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("index", &self.index)?;
        state.serialize_field("description", &self.description)?;
        let mut volumes = Vec::with_capacity(self.volume.len() as usize);
        for v in self.volume.get() {
            volumes.push(v.0);
        }
        state.serialize_field("channel_volumes", &volumes)?;
        state.serialize_field("mute", &self.mute)?;
        state.serialize_field("base_volume", &self.base_volume.0)?;
        state.serialize_field(
            "state",
            match self.state {
                SinkState::Invalid => "invalid",
                SinkState::Running => "running",
                SinkState::Idle => "idle",
                SinkState::Suspended => "suspended",
            },
        )?;
        state.serialize_field("n_volume_steps", &self.n_volume_steps)?;
        state.end()
    }
}

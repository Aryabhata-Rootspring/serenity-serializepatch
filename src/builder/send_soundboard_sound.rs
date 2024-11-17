#[cfg(feature = "http")]
use crate::http::Http;
use crate::model::prelude::*;

/// A builder which to send a soundboard sound, to be used in conjunction with
/// [`GuildChannel::send_soundboard_sound`].
///
/// Discord docs:
/// - [Send Soundboard Sound](https://discord.com/developers/docs/resources/soundboard#send-soundboard-sound)
#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct SendSoundboardSound {
    sound_id: SoundboardSoundId,
    source_guild_id: Option<GuildId>,
}

impl SendSoundboardSound {
    /// Create a new builder with the given soundboard sound id
    pub fn new(sound_id: SoundboardSoundId) -> Self {
        Self::default().sound_id(sound_id)
    }

    pub fn sound_id(mut self, sound_id: SoundboardSoundId) -> Self {
        self.sound_id = sound_id;
        self
    }

    pub fn source_guild_id(mut self, source_guild_id: GuildId) -> Self {
        self.source_guild_id = Some(source_guild_id);
        self
    }

    /// Edits the given user's voice state in a stage channel. Providing a [`UserId`] will edit
    /// that user's voice state, otherwise the current user's voice state will be edited.
    ///
    /// **Note**: Requires the [Request to Speak] permission. Also requires the [Mute Members]
    /// permission to suppress another user or unsuppress the current user. This is not required if
    /// suppressing the current user.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the user lacks permission, or if invalid data is given.
    ///
    /// [Request to Speak]: Permissions::REQUEST_TO_SPEAK
    /// [Mute Members]: Permissions::MUTE_MEMBERS
    #[cfg(feature = "http")]
    pub async fn execute(self, http: &Http, channel_id: ChannelId) -> Result<()> {
        http.send_soundboard_sound(channel_id, &self).await
    }
}

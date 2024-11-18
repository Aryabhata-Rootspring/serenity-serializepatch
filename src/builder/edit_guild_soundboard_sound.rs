use std::borrow::Cow;

#[cfg(feature = "http")]
use crate::all::Http;
use crate::model::prelude::*;

/// A builder to modify a guild soundboard sound
///
/// [Discord docs](https://discord.com/developers/docs/resources/soundboard#get-guild-soundboard-sound)
#[derive(Default, serde::Serialize, Clone, Debug)]
#[must_use]
pub struct EditGuildSoundboardSound<'a> {
    name: Option<Cow<'static, str>>,
    volume: Option<f64>,
    emoji_id: Option<EmojiId>,
    emoji_name: Option<Cow<'static, str>>,
    audit_log_reason: Option<&'a str>,
}

impl<'a> EditGuildSoundboardSound<'a> {
    /// Equivalent to [`Self::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the name of the guild soundboard sound, replacing the current value as set in
    /// [`Self::new`].
    ///
    /// **Note**: Must be between 2 and 32 characters long.
    pub fn name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the volume of the soundboard sound.
    ///
    /// **Note**: Must be between 0.0 and 1.0.
    pub fn volume(mut self, volume: f64) -> Self {
        self.volume = Some(volume);
        self
    }

    /// Set the emoji id of the soundboard sound.
    pub fn emoji_id(mut self, emoji_id: EmojiId) -> Self {
        self.emoji_id = Some(emoji_id);
        self
    }

    /// Set the unicode character (emoji name) of a standard emoji for the soundboard sound
    pub fn emoji_name(mut self, emoji_name: impl Into<Cow<'static, str>>) -> Self {
        self.emoji_name = Some(emoji_name.into());
        self
    }

    /// Sets the request's audit log reason.
    pub fn audit_log_reason(mut self, reason: &'a str) -> Self {
        self.audit_log_reason = Some(reason);
        self
    }

    /// Modifies a new guild soundboard sound in the guild with the data set, if any.
    ///
    /// **Note**: Requires the [Create Guild Expressions] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission or if invalid data is given.
    ///
    /// [Create Guild Expressions]: Permissions::CREATE_GUILD_EXPRESSIONS
    #[cfg(feature = "http")]
    pub async fn execute(
        self,
        http: &Http,
        guild_id: GuildId,
        sound_id: SoundboardSoundId,
    ) -> Result<SoundboardSound> {
        http.edit_guild_soundboard_sound(guild_id, sound_id, &self, self.audit_log_reason).await
    }
}

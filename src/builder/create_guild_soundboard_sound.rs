use std::borrow::Cow;

use super::CreateAttachment;
#[cfg(feature = "http")]
use crate::all::Http;
use crate::model::prelude::*;

/// A builder to create a guild soundboard sound
///
/// [Discord docs](https://discord.com/developers/docs/resources/soundboard#get-guild-soundboard-sound)
#[derive(serde::Serialize, Clone, Debug)]
#[must_use]
pub struct CreateGuildSoundboardSound<'a> {
    name: Cow<'static, str>,
    sound: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_id: Option<EmojiId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_name: Option<Cow<'static, str>>,
    #[serde(skip)]
    audit_log_reason: Option<&'a str>,
}

impl<'a> CreateGuildSoundboardSound<'a> {
    /// Creates a new builder with the given data.
    pub fn new(name: impl Into<Cow<'static, str>>, sound: &CreateAttachment<'a>) -> Self {
        Self {
            name: name.into(),
            sound: sound.to_base64().into(),
            volume: None,
            emoji_id: None,
            emoji_name: None,
            audit_log_reason: None,
        }
    }

    /// Set the name of the guild soundboard sound, replacing the current value as set in
    /// [`Self::new`].
    ///
    /// **Note**: Must be between 2 and 32 characters long.
    pub fn name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the soundboard file. Replaces the current value as set in [`Self::new`].
    ///
    /// **Note**: Must be a MPG or OGG, max 512 KB and max duration of 5.2 secons.
    pub fn sound(mut self, sound: &CreateAttachment<'a>) -> Self {
        self.sound = sound.to_base64().into();
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

    /// Creates a new guild soundboard sound in the guild with the data set, if any.
    ///
    /// **Note**: Requires the [Create Guild Expressions] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission or if invalid data is given.
    ///
    /// [Create Guild Expressions]: Permissions::CREATE_GUILD_EXPRESSIONS
    #[cfg(feature = "http")]
    pub async fn execute(self, http: &Http, guild_id: GuildId) -> Result<SoundboardSound> {
        http.create_guild_soundboard_sound(guild_id, &self, self.audit_log_reason).await
    }
}

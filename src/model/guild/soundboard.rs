//! Models relating to soundboard which are sounds that can be played in voice channels.
//!
//! See [Soundboard](https://discord.com/developers/docs/resources/soundboard) for more information

use crate::model::prelude::*;

/// Represents a custom guild emoji, which can either be created using the API, or via an
/// integration. Emojis created using the API only work within the guild it was created in.
///
/// [Discord docs](https://discord.com/developers/docs/resources/emoji#emoji-object).
#[bool_to_bitflags::bool_to_bitflags]
#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct SoundboardSound {
    /// The name of this sound
    pub name: FixedString<u32>,
    /// The id of this sound
    pub id: SoundboardSoundId,
    /// The volume of this sound, from 0 to 1
    pub volume: f64,
    /// the id of this sound's custom emoji
    pub emoji_id: Option<EmojiId>,
    /// the unicode character of this sound's standard emoji
    pub emoji_unicode: Option<FixedString<u8>>,
    /// the id of the guild this sound is in
    pub guild_id: Option<GuildId>,
    /// whether this sound can be used, may be false due to loss of Server Boosts
    pub available: bool,
    /// the user who created this sound
    pub user: Option<User>,
}

#[cfg(feature = "model")]
impl SoundboardSoundId {
    /// Generates a URL to the soundboard sound's file.
    ///
    /// # Examples
    ///
    /// Print the direct link to the given soundboard sound:
    ///
    /// ```rust,no_run
    /// # use serenity::model::guild::soundboard::SoundboardSound;
    /// #
    /// # fn run(sound: SoundboardSound) {
    /// // assuming sound has been set already
    /// println!("Direct link to sound file: {}", sound.id.url());
    /// # }
    /// ```
    #[must_use]
    pub fn url(self) -> String {
        cdn!("/soundboard-sounds/{}", self.get())
    }
}

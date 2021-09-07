use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

// https://github.com/Rapptz/discord-ext-native-voice/blob/006a83b279e35b902dbbe8cbf5cdee7d3e617d1e/src/payloads.rs

pub struct OpCode;

impl OpCode {
    pub const IDENTIFY: u8 = 2;
    pub const HEARTBEAT: u8 = 1;
    pub const HEARTBEAT_ACK: u8 = 11;
    pub const HELLO: u8 = 10;
    pub const DISPATCH: u8 = 0;
    pub const INVALID_SESSION: u8 = 9;
    //pub const RECONNECT: u8 = 7;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawReceivedPayload<'a> {
    pub op: u8,
    #[serde(borrow)]
    pub d: &'a RawValue,
    pub s: Option<u64>,
    pub t: Option<&'a str>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCreate {
    #[serde(rename = "type")]
    pub _type: u8,
    pub tts: bool,
    pub timestamp: String,
    pub referenced_message: Option<String>,
    pub pinned: bool,
    pub nonce : Option<String>,
    pub mentions: Vec<serde_json::Value>,
    pub mention_roles: Vec<serde_json::Value>,
    pub mention_everyone: bool,
    pub member: Option<serde_json::Value>,
    pub id: String,
    pub flags: u64,
    pub embeds: Vec<serde_json::Value>,
    pub edited_timestamp: Option<String>,
    pub content: String,
    pub components: Vec<serde_json::Value>,
    pub channel_id: String,
    pub author: Author,
    pub attachments: Vec<serde_json::Value>,
    pub guild_id: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub username: String,
    pub public_flags: u64,
    pub id: String,
    pub discriminator: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hello {
    pub heartbeat_interval: f64,
}


#[derive(Debug, Deserialize, Serialize)]
pub enum GatewayIntents {
/**
  - GUILD_CREATE
  - GUILD_UPDATE
  - GUILD_DELETE
  - GUILD_ROLE_CREATE
  - GUILD_ROLE_UPDATE
  - GUILD_ROLE_DELETE
  - CHANNEL_CREATE
  - CHANNEL_UPDATE
  - CHANNEL_DELETE
  - CHANNEL_PINS_UPDATE
  - THREAD_CREATE
  - THREAD_UPDATE
  - THREAD_DELETE
  - THREAD_LIST_SYNC
  - THREAD_MEMBER_UPDATE
  - THREAD_MEMBERS_UPDATE *
  - STAGE_INSTANCE_CREATE
  - STAGE_INSTANCE_UPDATE
  - STAGE_INSTANCE_DELETE
*/
    Guild  = 1 << 0,
/**
  - GUILD_MEMBER_ADD
  - GUILD_MEMBER_UPDATE
  - GUILD_MEMBER_REMOVE
  - THREAD_MEMBERS_UPDATE *
 */
    GuildMember = 1 << 1,
/**
  - GUILD_BAN_ADD
  - GUILD_BAN_REMOVE
 */
    GuildBans = 1 << 2,
/**
   - GUILD_EMOJIS_UPDATE
  - GUILD_STICKERS_UPDATE
 */
    GuildEmojisAndStickers = 1 << 3,
/**
   - GUILD_INTEGRATIONS_UPDATE
  - INTEGRATION_CREATE
  - INTEGRATION_UPDATE
  - INTEGRATION_DELETE
 */
    GuildIntegrations = 1 << 4,
/**
   - WEBHOOKS_UPDATE
 */
    GuildWebhooks = 1 << 5,

/**
  - INVITE_CREATE
  - INVITE_DELETE
 */
    GuildInvites = 1 << 6,

/**
 - VOICE_STATE_UPDATE
 */
    GuildVoiceStates = 1 << 7,
/**
- PRESENCE_UPDATE
 */
    GuildPresences = 1 << 8,

/**
   - MESSAGE_CREATE
  - MESSAGE_UPDATE
  - MESSAGE_DELETE
  - MESSAGE_DELETE_BULK
 */
    GuildMessages = 1 << 9,

/**
   - MESSAGE_REACTION_ADD
  - MESSAGE_REACTION_REMOVE
  - MESSAGE_REACTION_REMOVE_ALL
  - MESSAGE_REACTION_REMOVE_EMOJI
 */
    GuildMessageReactions = 1 << 10,

/**
 *   - TYPING_START
 */
    GuildMessageTyping = 1 << 11,

/**
 *   - MESSAGE_CREATE
  - MESSAGE_UPDATE
  - MESSAGE_DELETE
  - CHANNEL_PINS_UPDATE
 */
    DirectMessages = 1 << 12,
/**
 *   - MESSAGE_REACTION_ADD
  - MESSAGE_REACTION_REMOVE
  - MESSAGE_REACTION_REMOVE_ALL
  - MESSAGE_REACTION_REMOVE_EMOJI
 */
    DirectMessageReactions = 1 << 13,

/**
 *   - TYPING_START
 */
    DirectMessageTyping = 1 << 14,
}
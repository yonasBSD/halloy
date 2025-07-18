use chrono::{DateTime, Local, Utc};
use serde::Deserialize;

pub use self::away::Away;
pub use self::channel::Channel;
use crate::serde::default_bool_true;

pub mod away;
pub mod channel;

use crate::buffer::{
    DateSeparators, Nickname, SkinTone, StatusMessagePrefix, TextInput,
    Timestamp,
};
use crate::message::source;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Buffer {
    #[serde(default)]
    pub away: Away,
    #[serde(default)]
    pub timestamp: Timestamp,
    #[serde(default)]
    pub nickname: Nickname,
    #[serde(default)]
    pub text_input: TextInput,
    #[serde(default)]
    pub channel: Channel,
    #[serde(default)]
    pub server_messages: ServerMessages,
    #[serde(default)]
    pub internal_messages: InternalMessages,
    #[serde(default)]
    pub status_message_prefix: StatusMessagePrefix,
    #[serde(default)]
    pub chathistory: ChatHistory,
    #[serde(default)]
    pub date_separators: DateSeparators,
    #[serde(default)]
    pub commands: Commands,
    #[serde(default)]
    pub emojis: Emojis,
    #[serde(default)]
    pub mark_as_read: MarkAsRead,
    #[serde(default)]
    pub url: Url,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum NicknameClickAction {
    #[default]
    OpenQuery,
    InsertNickname,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Emojis {
    #[serde(default = "default_bool_true")]
    pub show_picker: bool,
    #[serde(default)]
    pub skin_tone: SkinTone,
    #[serde(default = "default_bool_true")]
    pub auto_replace: bool,
    #[serde(default = "default_characters_to_trigger_picker")]
    pub characters_to_trigger_picker: usize,
}

impl Default for Emojis {
    fn default() -> Self {
        Self {
            show_picker: default_bool_true(),
            skin_tone: SkinTone::default(),
            auto_replace: default_bool_true(),
            characters_to_trigger_picker: default_characters_to_trigger_picker(
            ),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Url {
    #[serde(default)]
    pub prompt_before_open: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarkAsRead {
    #[serde(default)]
    pub on_application_exit: bool,
    #[serde(default = "default_bool_true")]
    pub on_buffer_close: bool,
    #[serde(default = "default_bool_true")]
    pub on_scroll_to_bottom: bool,
    #[serde(default = "default_bool_true")]
    pub on_message_sent: bool,
}

impl Default for MarkAsRead {
    fn default() -> Self {
        Self {
            on_application_exit: bool::default(),
            on_buffer_close: default_bool_true(),
            on_scroll_to_bottom: default_bool_true(),
            on_message_sent: default_bool_true(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Commands {
    #[serde(default = "default_bool_true")]
    pub show_description: bool,
}

impl Default for Commands {
    fn default() -> Self {
        Self {
            show_description: default_bool_true(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ServerMessages {
    #[serde(default)]
    pub topic: ServerMessage,
    #[serde(default)]
    pub join: ServerMessage,
    #[serde(default)]
    pub part: ServerMessage,
    #[serde(default)]
    pub quit: ServerMessage,
    #[serde(default)]
    pub change_host: ServerMessage,
    #[serde(default)]
    pub change_mode: ServerMessage,
    #[serde(default)]
    pub change_nick: ServerMessage,
    #[serde(default)]
    pub monitored_online: ServerMessage,
    #[serde(default)]
    pub monitored_offline: ServerMessage,
    #[serde(default)]
    pub standard_reply_fail: ServerMessage,
    #[serde(default)]
    pub standard_reply_warn: ServerMessage,
    #[serde(default)]
    pub standard_reply_note: ServerMessage,
    #[serde(default)]
    pub wallops: ServerMessage,
}

impl ServerMessages {
    pub fn get(&self, server: &source::Server) -> Option<&ServerMessage> {
        match server.kind() {
            source::server::Kind::ReplyTopic => Some(&self.topic),
            source::server::Kind::Join => Some(&self.join),
            source::server::Kind::Part => Some(&self.part),
            source::server::Kind::Quit => Some(&self.quit),
            source::server::Kind::ChangeHost => Some(&self.change_host),
            source::server::Kind::ChangeMode => Some(&self.change_mode),
            source::server::Kind::ChangeNick => Some(&self.change_nick),
            source::server::Kind::MonitoredOnline => {
                Some(&self.monitored_online)
            }
            source::server::Kind::MonitoredOffline => {
                Some(&self.monitored_offline)
            }
            source::server::Kind::StandardReply(
                source::server::StandardReply::Fail,
            ) => Some(&self.standard_reply_fail),
            source::server::Kind::StandardReply(
                source::server::StandardReply::Warn,
            ) => Some(&self.standard_reply_warn),
            source::server::Kind::StandardReply(
                source::server::StandardReply::Note,
            ) => Some(&self.standard_reply_note),
            source::server::Kind::WAllOps => Some(&self.wallops),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerMessage {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default)]
    pub smart: Option<i64>,
    #[serde(default)]
    pub username_format: UsernameFormat,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub include: Vec<String>,
}

impl Default for ServerMessage {
    fn default() -> Self {
        Self {
            enabled: true,
            smart: Option::default(),
            username_format: UsernameFormat::default(),
            exclude: Vec::default(),
            include: Vec::default(),
        }
    }
}

impl ServerMessage {
    pub fn should_send_message(&self, channel: &str) -> bool {
        // Server Message is not enabled.
        if !self.enabled {
            return false;
        }

        let is_channel_filtered = |list: &Vec<String>, channel: &str| -> bool {
            let wildcards = ["*", "all"];

            list.iter().any(|item| {
                wildcards.contains(&item.as_str()) || item == channel
            })
        };

        let channel_included = is_channel_filtered(&self.include, channel);
        let channel_excluded = is_channel_filtered(&self.exclude, channel);

        // If the channel is included, it has precedence over excluded.
        channel_included || !channel_excluded
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternalMessages {
    #[serde(default)]
    pub success: InternalMessage,
    #[serde(default)]
    pub error: InternalMessage,
}

impl InternalMessages {
    pub fn get(&self, server: &source::Status) -> Option<&InternalMessage> {
        match server {
            source::Status::Success => Some(&self.success),
            source::Status::Error => Some(&self.error),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct InternalMessage {
    #[serde(default = "default_bool_true")]
    pub enabled: bool,
    #[serde(default)]
    pub smart: Option<i64>,
}

impl Default for InternalMessage {
    fn default() -> Self {
        Self {
            enabled: true,
            smart: Option::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LevelFilter {
    Off,
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatHistory {
    #[serde(default = "default_bool_true")]
    pub infinite_scroll: bool,
}

impl Default for ChatHistory {
    fn default() -> Self {
        Self {
            infinite_scroll: true,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UsernameFormat {
    Short,
    #[default]
    Full,
    #[serde(skip)]
    Mask,
}

impl Buffer {
    pub fn format_timestamp(
        &self,
        date_time: &DateTime<Utc>,
    ) -> Option<String> {
        if self.timestamp.format.is_empty() {
            return None;
        }

        Some(format!(
            "{} ",
            self.timestamp.brackets.format(
                date_time
                    .with_timezone(&Local)
                    .format(&self.timestamp.format)
            )
        ))
    }
}

fn default_characters_to_trigger_picker() -> usize {
    2
}

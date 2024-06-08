use crate::Notification;

pub type ChannelConnection = apsak_notify::connection::ChannelConnection<Notification>;
pub use apsak_notify::connection::ChannelType;

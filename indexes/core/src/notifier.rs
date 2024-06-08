use crate::notification::Notification;
use apsak_notify::{connection::ChannelConnection, notifier::Notifier};

pub type IndexNotifier = Notifier<Notification, ChannelConnection<Notification>>;

//! A viewer is unbanned from the specified channel.
use super::*;

/// [`channel.unban`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelunban): a viewer is unbanned from the specified channel.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelUnbanV1 {
    /// The broadcaster user ID for the channel you want to get unban notifications for.
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelUnbanV1 {
    type Payload = ChannelUnbanV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelUnban;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelModerate];
    const VERSION: &'static str = "1";
}

/// [`channel.unban`](ChannelUnbanV1) response payload.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelUnbanV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster name.
    pub broadcaster_user_name: types::UserName,
    /// The user id for the user who was unbanned on the specified channel.
    pub user_id: types::UserId,
    /// The user name for the user who was unbanned on the specified channel.
    pub user_name: types::UserName,
}

#[test]
fn parse_payload() {
    let payload = r#"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.unban",
            "version": "1",
            "condition": {
                "broadcaster_user_id": "1337"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.123Z"
        },
        "event": {
            "user_id": "1234",
            "user_name": "cool_user",
            "broadcaster_user_id": "1337",
            "broadcaster_user_name": "cooler_user"
        }
    }
    "#;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

//! Holds serializable EventSub stuff
//!
//! Use [`CreateEventSubSubscription`](crate::helix::eventsub::CreateEventSubSubscription) to subscribe to an event according to the [EventSub guide](https://dev.twitch.tv/docs/eventsub).
//! Parse the response payload text with [`Payload::parse`] or the .
//!
//! # Example
//!
//! You've used [`CreateEventSubSubscription`](crate::helix::eventsub::CreateEventSubSubscription) to create a subscription for [`user.authorization.revoke`](EventType::UserAuthorizationRevoke), after verifying your callback accordingly you will then get events sent to the callback
//!
//! To parse these, use [`Payload::parse`]
//!
//! ```rust
//! use twitch_api2::eventsub::Payload;
//! let payload = r#"{
//!     "subscription": {
//!         "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
//!         "type": "user.authorization.revoke",
//!         "version": "1",
//!         "condition": {
//!             "client_id": "crq72vsaoijkc83xx42hz6i37"
//!         },
//!          "transport": {
//!             "method": "webhook",
//!             "callback": "https://example.com/webhooks/callback"
//!         },
//!         "created_at": "2019-11-16T10:11:12.123Z"
//!     },
//!     "event": {
//!         "client_id": "crq72vsaoijkc83xx42hz6i37",
//!         "user_id": "1337",
//!         "user_name": "cool_user"
//!     }
//! }"#;
//!
//! let payload = Payload::parse(payload).unwrap();
//! match payload {
//!     Payload::UserAuthorizationRevokeV1(p) => {
//!         println!("User with id `{}` has revoked access to client `{}`",
//!             p.event.user_id,
//!             p.event.client_id
//!         )
//!     }
//!     _ => { panic!() }
//! }
//! ```

use crate::types;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

pub mod channel;
pub mod stream;
pub mod user;

/// An EventSub subscription.
pub trait EventSubscription: DeserializeOwned + Serialize + PartialEq {
    /// Payload for given subscription
    type Payload: PartialEq + std::fmt::Debug + DeserializeOwned + Serialize;

    /// Scopes needed by this subscription
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope];
    /// Optional scopes needed by this subscription
    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    /// Subscription type version
    const VERSION: &'static str;
    /// Subscription type name.
    const EVENT_TYPE: EventType;

    /// Creates the [`condition`](https://dev.twitch.tv/docs/eventsub/eventsub-reference#conditions) for this EventSub subscription
    fn condition(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
/// Subscription payload. Received on events. Enumerates all possible [`NotificationPayload`s](NotificationPayload)
///
/// Use [`Payload::parse`] to construct
#[derive(PartialEq, Debug, Serialize, Deserialize)] // FIXME: Clone?
#[serde(remote = "Self")]
#[allow(clippy::large_enum_variant)]
pub enum Payload {
    /// Channel Update V1 Event
    ChannelUpdateV1(NotificationPayload<channel::ChannelUpdateV1>),
    /// Channel Follow V1 Event
    ChannelFollowV1(NotificationPayload<channel::ChannelFollowV1>),
    /// Channel Subscribe V1 Event
    ChannelSubscribeV1(NotificationPayload<channel::ChannelSubscribeV1>),
    /// Channel Cheer V1 Event
    ChannelCheerV1(NotificationPayload<channel::ChannelCheerV1>),
    /// Channel Ban V1 Event
    ChannelBanV1(NotificationPayload<channel::ChannelBanV1>),
    /// Channel Unban V1 Event
    ChannelUnbanV1(NotificationPayload<channel::ChannelUnbanV1>),
    /// Channel Points Custom Reward Add V1 Event
    ChannelPointsCustomRewardAddV1(NotificationPayload<channel::ChannelPointsCustomRewardAddV1>),
    /// Channel Points Custom Reward Update V1 Event
    ChannelPointsCustomRewardUpdateV1(
        NotificationPayload<channel::ChannelPointsCustomRewardUpdateV1>,
    ),
    /// Channel Points Custom Reward Remove V1 Event
    ChannelPointsCustomRewardRemoveV1(
        NotificationPayload<channel::ChannelPointsCustomRewardRemoveV1>,
    ),
    /// Channel Points Custom Reward Redemption Add V1 Event
    ChannelPointsCustomRewardRedemptionAddV1(
        NotificationPayload<channel::ChannelPointsCustomRewardRedemptionAddV1>,
    ),
    /// Channel Points Custom Reward Redemption Update V1 Event
    ChannelPointsCustomRewardRedemptionUpdateV1(
        NotificationPayload<channel::ChannelPointsCustomRewardRedemptionUpdateV1>,
    ),
    /// Channel Hype Train Begin V1 Event
    ChannelHypeTrainBeginV1(NotificationPayload<channel::ChannelHypeTrainBeginV1>),
    /// Channel Hype Train Progress V1 Event
    ChannelHypeTrainProgressV1(NotificationPayload<channel::ChannelHypeTrainProgressV1>),
    /// Channel Hype Train End V1 Event
    ChannelHypeTrainEndV1(NotificationPayload<channel::ChannelHypeTrainEndV1>),
    /// StreamOnline V1 Event
    StreamOnlineV1(NotificationPayload<stream::StreamOnlineV1>),
    /// StreamOffline V1 Event
    StreamOfflineV1(NotificationPayload<stream::StreamOfflineV1>),
    /// User Update V1 Event
    UserUpdateV1(NotificationPayload<user::UserUpdateV1>),
    /// User Authorization Revoke V1 Event
    UserAuthorizationRevokeV1(NotificationPayload<user::UserAuthorizationRevokeV1>),
}

impl Payload {
    /// Parse string slice as a [Payload]
    pub fn parse(source: &str) -> Result<Payload, PayloadParseError> {
        serde_json::from_str(source).map_err(Into::into)
    }

    /// Parse string slice as a [Payload]
    pub fn parse_response(source: &http::Response<Vec<u8>>) -> Result<Payload, PayloadParseError> {
        Payload::parse(std::str::from_utf8(source.body())?)
    }
}

/// Errors that can happen when parsing payload
#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub enum PayloadParseError {
    /// could not parse [`http::Request::body()`] as UTF8
    Utf8Error(#[from] std::str::Utf8Error),
    /// could not parse [`http::Request::body()`] as a [`Payload`]
    DeserializeError(#[from] serde_json::Error),
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::convert::TryInto;
        macro_rules! match_event {
            ($response:expr; $($module:ident::$event:ident);* $(;)?) => {
                #[deny(unreachable_patterns)]
                match (&*$response.s.version, &$response.s.type_) {
                    $(  (<$module::$event as EventSubscription>::VERSION, &<$module::$event as EventSubscription>::EVENT_TYPE) => {
                        Payload::$event(NotificationPayload {
                            subscription: $response.s.try_into().map_err(serde::de::Error::custom)?,
                            event: serde_json::from_value($response.e).map_err(serde::de::Error::custom)?,
                        })
                    }  )*
                    (v, e) => return Err(serde::de::Error::custom(format!("could not find a match for version `{}` on event type `{}`", v, e)))
                }
            }
        }

        #[derive(Deserialize, Clone)]
        struct IEventSubscripionInformation {
            condition: serde_json::Value,
            created_at: types::Timestamp,
            id: String,
            transport: TransportResponse,
            #[serde(rename = "type")]
            type_: EventType,
            version: String,
        }
        #[derive(Deserialize)]
        struct IResponse {
            #[serde(rename = "subscription")]
            s: IEventSubscripionInformation,
            #[serde(rename = "event")]
            e: serde_json::Value,
        }

        impl<E: EventSubscription> std::convert::TryFrom<IEventSubscripionInformation>
            for EventSubscriptionInformation<E>
        {
            type Error = serde_json::Error;

            fn try_from(info: IEventSubscripionInformation) -> Result<Self, Self::Error> {
                debug_assert_eq!(info.version, E::VERSION);
                debug_assert_eq!(info.type_, E::EVENT_TYPE);
                Ok(EventSubscriptionInformation {
                    id: info.id,
                    condition: serde_json::from_value(info.condition)?,
                    created_at: info.created_at,
                    transport: info.transport,
                })
            }
        }

        let response = IResponse::deserialize(deserializer).map_err(|e| {
            serde::de::Error::custom(format!("could not deserialize response: {}", e))
        })?;
        Ok(match_event! { response;
            channel::ChannelUpdateV1;
            channel::ChannelFollowV1;
            channel::ChannelSubscribeV1;
            channel::ChannelCheerV1;
            channel::ChannelBanV1;
            channel::ChannelUnbanV1;
            channel::ChannelPointsCustomRewardAddV1;
            channel::ChannelPointsCustomRewardUpdateV1;
            channel::ChannelPointsCustomRewardRemoveV1;
            channel::ChannelPointsCustomRewardRedemptionAddV1;
            channel::ChannelPointsCustomRewardRedemptionUpdateV1;
            channel::ChannelHypeTrainBeginV1;
            channel::ChannelHypeTrainProgressV1;
            channel::ChannelHypeTrainEndV1;
            stream::StreamOnlineV1;
            stream::StreamOfflineV1;
            user::UserUpdateV1;
            user::UserAuthorizationRevokeV1;
        })
    }
}

/// Notification received
#[derive(PartialEq, Deserialize, Serialize, Debug)] // FIXME: Clone?
pub struct NotificationPayload<E: EventSubscription> {
    /// Subscription information.
    #[serde(bound = "E: EventSubscription")]
    pub subscription: EventSubscriptionInformation<E>,
    /// Event information.
    #[serde(bound = "E: EventSubscription")]
    pub event: <E as EventSubscription>::Payload,
}

/// Metadata about the subscription.
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct EventSubscriptionInformation<E: EventSubscription> {
    /// Your client ID.
    id: String,
    /// Subscription-specific parameters.
    #[serde(bound = "E: EventSubscription")]
    condition: E,
    /// The time the notification was created.
    created_at: types::Timestamp,
    transport: TransportResponse,
}

/// Transport setting for event notification
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct Transport {
    method: TransportMethod,
    callback: String,
    secret: String,
}

/// Transport response on event notification
///
/// Does not include secret.
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
pub struct TransportResponse {
    method: TransportMethod,
    callback: String,
}

/// Transport method
///
/// Currently, only webhooks are supported
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    /// Webhook
    Webhook,
}

/// Event name
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
pub enum EventType {
    /// `channel.update` subscription type sends notifications when a broadcaster updates the category, title, mature flag, or broadcast language for their channel.
    #[serde(rename = "channel.update")]
    ChannelUpdate,
    /// `channel.follow`: a specified channel receives a follow.
    #[serde(rename = "channel.follow")]
    ChannelFollow,
    /// `channel.subscribe`: a specified channel receives a subscriber. This does not include resubscribes.
    #[serde(rename = "channel.subscribe")]
    ChannelSubscribe,
    /// `channel.cheer`: a user cheers on the specified channel.
    #[serde(rename = "channel.cheer")]
    ChannelCheer,
    /// `channel.ban`: a viewer is banned from the specified channel.
    #[serde(rename = "channel.ban")]
    ChannelBan,
    /// `channel.unban`: a viewer is unbanned from the specified channel.
    #[serde(rename = "channel.unban")]
    ChannelUnban,
    /// `channel.channel_points_custom_reward.add`: a custom channel points reward has been created for the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward.add")]
    ChannelPointsCustomRewardAdd,
    /// `channel.channel_points_custom_reward.update`: a custom channel points reward has been updated for the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward.update")]
    ChannelPointsCustomRewardUpdate,
    /// `channel.channel_points_custom_reward.remove`: a custom channel points reward has been removed from the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward.remove")]
    ChannelPointsCustomRewardRemove,
    /// `channel.channel_points_custom_reward_redemption.add`: a viewer has redeemed a custom channel points reward on the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward_redemption.add")]
    ChannelPointsCustomRewardRedemptionAdd,
    /// `channel.channel_points_custom_reward_redemption.update`: a redemption of a channel points custom reward has been updated for the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward_redemption.update")]
    ChannelPointsCustomRewardRedemptionUpdate,
    /// `channel.hype_train.begin`: a hype train begins on the specified channel.
    #[serde(rename = "channel.hype_train.begin")]
    ChannelHypeTrainBegin,
    /// `channel.hype_train.progress`: a hype train makes progress on the specified channel.
    #[serde(rename = "channel.hype_train.progress")]
    ChannelHypeTrainProgress,
    /// `channel.hype_train.end`: a hype train ends on the specified channel.
    #[serde(rename = "channel.hype_train.end")]
    ChannelHypeTrainEnd,
    /// `stream.online`: the specified broadcaster starts a stream.
    #[serde(rename = "stream.online")]
    StreamOnline,
    /// `stream.online`: the specified broadcaster stops a stream.
    #[serde(rename = "stream.offline")]
    StreamOffline,
    /// `user.update`: user updates their account.
    #[serde(rename = "user.update")]
    UserUpdate,
    /// `user.authorization.revoke`: a user has revoked authorization for your client id. Use this webhook to meet government requirements for handling user data, such as GDPR, LGPD, or CCPA.
    #[serde(rename = "user.authorization.revoke")]
    UserAuthorizationRevoke,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.serialize(f) }
}

///  Subscription request status
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    /// Designates that the subscription is in an operable state and is valid.
    Enabled,
    /// Webhook is pending verification of the callback specified in the subscription creation request.
    WebhookCallbackVerificationPending,
    /// Webhook failed verification of the callback specified in the subscription creation request.
    WebhookCallbackVerificationFailed,
    /// Notification delivery failure rate was too high.
    NotificationFailuresExceeded,
    /// Authorization for user(s) in the condition was revoked.
    AuthorizationRevoked,
    /// A user in the condition of the subscription was removed.
    UserRemoved,
}

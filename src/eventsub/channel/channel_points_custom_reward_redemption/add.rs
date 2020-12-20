//! A viewer has redeemed a custom channel points reward on the specified channel.

use super::*;
/// [`channel.channel_points_custom_reward_redemption.add`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_custom_reward_redemptionadd): a viewer has redeemed a custom channel points reward on the specified channel.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelPointsCustomRewardRedemptionAddV1 {
    /// The broadcaster user ID for the channel you want to receive channel points custom reward redemption add notifications for.
    pub broadcaster_user_id: types::UserId,
    /// Optional. Specify a reward id to only receive notifications for a specific reward.
    pub reward_id: Option<types::RewardId>,
}

impl EventSubscription for ChannelPointsCustomRewardRedemptionAddV1 {
    type Payload = ChannelPointsCustomRewardRedemptionAddV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelPointsCustomRewardRedemptionAdd;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadRedemptions];
    const VERSION: &'static str = "1";
}

// FIXME: Same as update
/// [`channel.channel_points_custom_reward_redemption.add`](ChannelPointsCustomRewardRedemptionAddV1) response payload.
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct ChannelPointsCustomRewardRedemptionAddV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: String,
    /// The requested broadcaster name.
    pub broadcaster_user_name: String,
    /// The redemption identifier.
    pub id: types::RewardId,
    /// RFC3339 timestamp of when the reward was redeemed.
    pub redeemed_at: types::Timestamp,
    /// Basic information about the reward that was redeemed, at the time it was redeemed.
    pub reward: Reward,
    /// Defaults to unfulfilled. Possible values are unknown, unfulfilled, fulfilled, and canceled.
    pub status: String,
    /// User ID of the user that redeemed the reward.
    pub user_id: String,
    /// The user input provided. Empty string if not provided.
    pub user_input: String,
    /// Display name of the user that redeemed the reward.
    pub user_name: String,
}

#[test]
fn parse_payload() {
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.channel_points_custom_reward_redemption.add",
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
            "id": "1234",
            "broadcaster_user_id": "1337",
            "broadcaster_user_name": "cool_user",
            "user_id": "9001",
            "user_name": "cooler_user",
            "user_input": "pogchamp",
            "status": "unfulfilled",
            "reward": {
                "id": "9001",
                "title": "title",
                "cost": 100,
                "prompt": "reward prompt"
            },
            "redeemed_at": "2020-07-15T17:16:03.17106713Z"
        }
    }
    "##;

    dbg!(crate::eventsub::Payload::parse(payload).unwrap());
}

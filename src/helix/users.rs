//! Endpoints regarding users
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, users::GetUsersRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetUsersRequest::builder()
//!     .login(vec!["justinfan1337".to_string()])
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
#[doc(inline)]
pub use get_users::{GetUsersRequest, User};

#[doc(inline)]
pub use get_users_follows::{GetUsersFollowsRequest, UsersFollows};

#[doc(inline)]
pub use delete_user_follows::{DeleteUserFollows, DeleteUserFollowsRequest};

#[doc(inline)]
pub use create_user_follows::{CreateUserFollows, CreateUserFollowsBody, CreateUserFollowsRequest};

use crate::helix;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Gets information about one or more specified Twitch users.
/// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
pub mod get_users {
    use super::*;
    /// Query Parameters for [Get Users](super::get_users)
    ///
    /// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetUsersRequest {
        /// User ID. Multiple user IDs can be specified. Limit: 100.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub id: Vec<String>,
        /// User login name. Multiple login names can be specified. Limit: 100.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub login: Vec<String>,
    }

    /// Return Values for [Get Users](super::get_users)
    ///
    /// [`get-users`](https://dev.twitch.tv/docs/api/reference#get-users)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct User {
        /// User’s broadcaster type: "partner", "affiliate", or "".
        pub broadcaster_type: Option<String>,
        /// User’s channel description.
        pub description: Option<String>,
        /// User’s display name.
        pub display_name: String,
        /// User’s email address. Returned if the request includes the [user:read:email scope](twitch_oauth2::Scope::UserReadEmail).
        pub email: Option<String>,
        /// User’s ID.
        pub id: String,
        /// User’s login name.
        pub login: String,
        /// URL of the user’s offline image.
        pub offline_image_url: Option<String>,
        /// URL of the user’s profile image.
        pub profile_image_url: Option<String>,
        /// User’s type: "staff", "admin", "global_mod", or "".
        #[serde(rename = "type")]
        pub type_: Option<String>,
        /// Total number of views of the user’s channel.
        pub view_count: usize,
    }

    impl helix::Request for GetUsersRequest {
        type Response = User;

        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
        const PATH: &'static str = "users";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetUsersRequest {}
}

/// Gets information on follow relationships between two Twitch users.
/// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
pub mod get_users_follows {
    use super::*;
    /// Query Parameters for [Get Users Follows](super::get_users_follows)
    ///
    /// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetUsersFollowsRequest {
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first: Option<usize>,
        /// User ID. The request returns information about users who are being followed by the from_id user.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<String>,
        /// User ID. The request returns information about users who are following the to_id user.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to_id: Option<String>,
    }

    /// Return Values for [Get Users Follows](super::get_users_follows)
    ///
    /// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct UsersFollows {
        ///Date and time when the from_id user followed the to_id user.
        pub followed_at: String,
        ///ID of the user following the to_id user.
        pub from_id: String,
        ///Display name corresponding to from_id.
        pub from_name: String,
        ///ID of the user being followed by the from_id user.
        pub to_id: String,
        ///Display name corresponding to to_id.
        pub to_name: String,
        // FIXME: This never seems to be returned.
        /// Total number of items returned.
        ///
        /// * If only `from_id` was in the request, this is the total number of followed users.
        /// * If only `to_id` was in the request, this is the total number of followers.
        /// * If both `from_id` and to_id were in the request, this is 1 (if the "from" user follows the "to" user) or 0.
        pub total: Option<usize>,
    }

    impl helix::Request for GetUsersFollowsRequest {
        type Response = UsersFollows;

        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
        const PATH: &'static str = "users/follows";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetUsersFollowsRequest {}
    impl helix::Paginated for GetUsersFollowsRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor); }
    }
}

/// Deletes a specified user from the followers of a specified channel.
/// [`delete-user-follows`](https://dev.twitch.tv/docs/api/reference#delete-user-follows)
///
/// # Notes
///
/// This doesn't seem to work for removing people who follow owner of token. Use irc `/block <user_login>` for that
pub mod delete_user_follows {
    use super::*;
    /// Query Parameters for [Delete Users Follows](super::delete_user_follows)
    ///
    /// [`delete-user-follows`](https://dev.twitch.tv/docs/api/reference#delete-user-follows)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct DeleteUserFollowsRequest {
        /// User ID of the follower
        #[builder(default, setter(into))]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<String>,
        /// Channel to be unfollowed by the user
        #[builder(default, setter(into))]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to_id: Option<String>,
    }
    /// Return Values for [[Delete Users Follows](super::delete_user_follows)
    ///
    /// [`delete-user-follows`](https://dev.twitch.tv/docs/api/reference#delete-user-follows)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub enum DeleteUserFollows {
        /// 204 - User successfully deleted from list of channel followers
        Success,
        /// 400 - Missing Query Parameter
        MissingQuery,
        /// 422 - Entity cannot be processed
        ProcessingError,
    }

    impl std::convert::TryFrom<http::StatusCode> for DeleteUserFollows {
        type Error = std::borrow::Cow<'static, str>;

        fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
            match s {
                http::StatusCode::NO_CONTENT => Ok(DeleteUserFollows::Success),
                http::StatusCode::BAD_REQUEST => Ok(DeleteUserFollows::MissingQuery),
                http::StatusCode::UNPROCESSABLE_ENTITY => Ok(DeleteUserFollows::Success),
                other => Err(other.canonical_reason().unwrap_or("").into()),
            }
        }
    }

    impl helix::Request for DeleteUserFollowsRequest {
        type Response = DeleteUserFollows;

        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
        const PATH: &'static str = "users/follows";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditFollows];
    }

    impl helix::RequestDelete for DeleteUserFollowsRequest {}
}

/// Adds a specified user to the followers of a specified channel.
/// [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
pub mod create_user_follows {
    use std::convert::TryInto;

    use super::*;
    /// Query Parameters for [Create Users Follows](super::create_user_follows)
    ///
    /// [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct CreateUserFollowsRequest {}

    /// Body Parameters for [Create Users Follows](super::create_user_follows)
    ///
    /// [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct CreateUserFollowsBody {
        /// User ID of the follower
        #[builder(default, setter(into))]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from_id: Option<String>,
        /// ID of the channel to be followed by the user
        #[builder(default, setter(into))]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to_id: Option<String>,
    }

    /// Return Values for [[Create Users Follows](super::create_user_follows)
    ///
    /// [`create-user-follows`](https://dev.twitch.tv/docs/api/reference#create-user-follows)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub enum CreateUserFollows {
        // FIXME: Twitch docs....
        /// 204 or 200 - Successfully created follows
        Success,
        /// 400 - Missing Query Parameter
        MissingQuery,
        /// 422 - Entity cannot be processed
        ProcessingError,
    }

    impl std::convert::TryFrom<http::StatusCode> for CreateUserFollows {
        type Error = std::borrow::Cow<'static, str>;

        fn try_from(s: http::StatusCode) -> Result<Self, Self::Error> {
            match s {
                http::StatusCode::NO_CONTENT | http::StatusCode::OK => {
                    Ok(CreateUserFollows::Success)
                }
                http::StatusCode::BAD_REQUEST => Ok(CreateUserFollows::MissingQuery),
                http::StatusCode::UNPROCESSABLE_ENTITY => Ok(CreateUserFollows::Success),
                other => Err(other.canonical_reason().unwrap_or("").into()),
            }
        }
    }

    impl helix::Request for CreateUserFollowsRequest {
        type Response = CreateUserFollows;

        const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
        const PATH: &'static str = "users/follows";
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserEditFollows];
    }

    impl helix::RequestPost for CreateUserFollowsRequest {
        type Body = CreateUserFollowsBody;

        fn result<RE>(
            self,
            url: &url::Url,
            body: &str,
            response: http::Response<Vec<u8>>,
        ) -> Result<
            helix::Response<Self, <Self as helix::Request>::Response>,
            helix::RequestError<RE>,
        >
        where
            RE: std::error::Error + Send + Sync + 'static,
            Self: Sized,
        {
            let response = response.status().try_into().map_err(|_| {
                // This path should never be taken, but just to be sure we do this
                helix::RequestError::HelixRequestPostError {
                    status: response.status(),
                    url: url.clone(),
                    body: body.to_string(),
                    message: String::new(), // FIXME: None, but this branch should really never be hit
                    error: String::new(),
                }
            })?;
            Ok(helix::Response {
                data: vec![response],
                pagination: <_>::default(),
                request: self,
            })
        }
    }
}
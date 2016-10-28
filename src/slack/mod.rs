
pub mod events;
pub mod rtm;
pub mod utils;

const SLACK_BASE_URL: &'static str = "https://slack.com/api/";

#[derive(Clone, Display, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlackError {
    pub ok: bool,
    pub error: String,
}

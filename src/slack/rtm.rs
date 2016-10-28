
pub mod start {

    use hyper::Client;
    use slack::utils;
    use std::error::Error;
    
    #[derive(Debug, Display, Default, Serialize, Deserialize)]
    pub struct StartPayload {
        pub url: String,
    }

    const METHOD: &'static str = "rtm.start";
    
    pub fn call(token: &str) -> Result<StartPayload, String> {
        let client = Client::new();
        // make the url
        let url = format!("{}{}?token={}", ::slack::SLACK_BASE_URL, METHOD, token);
        match client.get(&*url).send() {
            Ok(mut r) => {
                match utils::from_body::<StartPayload, _>(&mut r) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(format!("error calling slack api: {}", e.description()))
        }
    }
}

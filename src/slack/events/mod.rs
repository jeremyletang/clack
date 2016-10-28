
use serde_json::{self, Value};

pub fn _match(s: String) {
    let data: Value = match serde_json::from_str(&*s) {
        Ok(d) => d,
        Err(_) => unreachable!(),
    };

    match data.find("type").unwrap().as_str().unwrap() {
        "hello" => hello(&*s).handle(),
        "reconnect_url" => redirect_url(&*s).handle(),
        _ => {/*unsupported event*/}
    }
}

pub trait HandleMessage {
    fn handle(&mut self) {}
}

#[derive(Serialize, Deserialize)]
pub struct Hello;

pub fn hello(_: &str) -> Hello {
    return  Hello;
}

impl HandleMessage for Hello {}


#[derive(Serialize, Deserialize)]
pub struct RedirectUrl {
    pub url: String
}

pub fn redirect_url(s: &str) -> RedirectUrl {
    serde_json::from_str(s).unwrap()
}

impl HandleMessage for RedirectUrl {
    fn handle(&mut self)  {
        //::start_ws(&*self.url)
    }
}

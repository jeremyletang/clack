// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::{Serialize, Deserialize};
use serde_json;
use std::io::Read;
use slack::SlackError;

pub fn from_body<T, B>(body: &mut B) -> Result<T, String>
    where T: Serialize + Deserialize + Default,
          B: Read
{
    let mut s = String::new();
    match body.read_to_string(&mut s) {
        Ok(_) => {}
        Err(e) => return Err(format!("unable read request body: {}", e))
    }
    
    match serde_json::from_str(&s) {
        Ok(t) => Ok(t),
        Err(_) => {
            // unable to deserialize the T type, let's try to deserialize a slack error
            // and get the error message
            match serde_json::from_str::<SlackError>(&s) {
                Ok(t) => Err(format!("error from slack: {}", t.error)),
                Err(e) => Err(format!("unable to deserialize payload: {}", e))
            }
        }
    }
}

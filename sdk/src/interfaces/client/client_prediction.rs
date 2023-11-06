use lazy_static::lazy_static;

use crate::interfaces::create_interface;

lazy_static! {
    pub static ref CLIENT_PREDICTION: &'static Source2ClientPrediction =
        create_interface!("client.dll", "Source2ClientPrediction0");
}

pub struct Source2ClientPrediction {}

use lazy_static::lazy_static;

use crate::interfaces::create_interface;

lazy_static! {
    pub static ref CLIENT_UI: &'static Source2ClientUI =
        create_interface!("client.dll", "Source2ClientUI0");
}

pub struct Source2ClientUI {}

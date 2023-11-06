use lazy_static::lazy_static;

use crate::interfaces::create_interface;

lazy_static! {
    pub static ref CLIENT: &'static Source2Client =
        create_interface!("client.dll", "Source2Client0");
}

pub struct Source2Client {}

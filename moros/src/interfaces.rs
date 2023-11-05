use encryption::x;
use lazy_static::lazy_static;
use sdk::{
    create_interface,
    interfaces::{
        client::{
            client::Source2Client, client_prediction::Prediction, client_ui::Source2ClientUI,
            entity_system::GameEntitySystem,
        },
        engine::{engine_client::EngineClient, resource_service::GameResourceService},
    },
};

lazy_static! {
    pub static ref CLIENT: &'static Source2Client =
        create_interface!("client.dll\0", "Source2Client0");
    pub static ref CLIENT_UI: &'static Source2ClientUI =
        create_interface!("client.dll\0", "Source2ClientUI0");
    pub static ref CLIENT_PREDICTION: &'static Prediction =
        create_interface!("client.dll\0", "Source2ClientPrediction0");
    pub static ref ENGINE: &'static EngineClient =
        create_interface!("engine2.dll\0", "Source2EngineToClient0");
    pub static ref RESOURCE_SERVICE: &'static GameResourceService =
        create_interface!("engine2.dll\0", "GameResourceServiceClientV0");
    pub static ref ENTITY_SYSTEM: &'static GameEntitySystem = RESOURCE_SERVICE
        .entity_system()
        .expect(&x!("could not get entity system"));
}

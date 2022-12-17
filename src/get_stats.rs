use reqwest;
use serde::Deserialize;
//use std::collections::HashMap;
use tabled::{Table, Tabled, Width};

#[derive(Tabled, Debug, Deserialize)]
pub struct Stats {
    #[tabled(rename = "orcs")]
    personnel_units: i32,
    #[tabled(rename = "artillery systems")]
    artillery_systems: i32,
    #[tabled(rename = "helicopters")]
    helicopters: i32,
    #[tabled(rename = "warships/cutters")]
    warships_cutters: i32,
    #[tabled(rename = "cruise missiles")]
    cruise_missiles: i32,
    #[tabled(rename = "special military equip")]
    special_military_equip: i32,
    #[tabled(rename = "vehicles and fuel tanks")]
    vehicles_fuel_tanks: i32,
    #[tabled(rename = "MLRS")]
    mlrs: i32,
    #[tabled(rename = "AA warfare systems")]
    aa_warfare_systems: i32,
    #[tabled(rename = "ATGM/SRBM systems")]
    atgm_srbm_systems: i32,
    #[tabled(rename = "tanks")]
    tanks: i32,
    #[tabled(rename = "UAV systems")]
    uav_systems: i32,
    #[tabled(rename = "planes")]
    planes: i32,
    #[tabled(rename = "AFV")]
    armoured_fighting_vehicles: i32,
}
#[derive(Debug, Deserialize)]
pub struct Data {
    day: i32,
    //stats: HashMap<String, i32>,
    pub stats: Stats,
    //increase: HashMap<String, i32>,
}
#[derive(Debug, Deserialize)]
pub struct Losses {
    //#[serde(flatten)]
    pub data: Data,
}
pub async fn fetch() -> Result<Losses, String> {
    let res = reqwest::get("https://russianwarship.rip/api/v1/statistics/latest")
        .await
        .unwrap()
        .text()
        .await;
    match res {
        Ok(r) => {
            let data: Losses = serde_json::from_str(
                //"{\"message\":\"The data were fetched successfully.\",\"data\":{\"day\":297,\"resource\":\"https:\\/\\/www.facebook.com\\/MinistryofDefence.UA\\/posts\\/pfbid0J6FyCXR4hbuQ1HDDvvCXdwhjU72eg7CJ9xUP47onFBvF2xGs6W5MaGtfBm2eas7Bl\",\"stats\":{\"personnel_units\":97690,\"tanks\":2985,\"armoured_fighting_vehicles\":5958,\"artillery_systems\":1947,\"mlrs\":410,\"aa_warfare_systems\":211,\"planes\":281,\"helicopters\":264,\"vehicles_fuel_tanks\":4577,\"warships_cutters\":16,\"cruise_missiles\":653,\"uav_systems\":1648,\"special_military_equip\":174,\"atgm_srbm_systems\":4},\"increase\":{\"personnel_units\":420,\"tanks\":5,\"armoured_fighting_vehicles\":6,\"artillery_systems\":1,\"mlrs\":0,\"aa_warfare_systems\":0,\"planes\":0,\"helicopters\":0,\"vehicles_fuel_tanks\":14,\"warships_cutters\":0,\"cruise_missiles\":61,\"uav_systems\":0,\"special_military_equip\":2,\"atgm_srbm_systems\":0}}}"
                &r,
            )
            .unwrap();
            Ok(data)
        }
        Err(_) => Err(String::from("Our service is unavailable for you now")),
    }
}

pub fn print_stats(data: Data) -> (String, i32) {
    let stats = Vec::from([data.stats]);
    (Table::new(stats).with(Width::wrap(300)).to_string(), data.day)
}

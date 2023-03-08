use reqwest;
use serde::Deserialize;
//use std::collections::HashMap;
use crate::text_to_image::draw;

#[derive(Debug, Deserialize)]
pub struct Stats {
    personnel_units: i32,
    artillery_systems: i32,
    helicopters: i32,
    warships_cutters: i32,
    cruise_missiles: i32,
    special_military_equip: i32,
    vehicles_fuel_tanks: i32,
    mlrs: i32,
    aa_warfare_systems: i32,
    atgm_srbm_systems: i32,
    tanks: i32,
    uav_systems: i32,
    planes: i32,
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
    /*
    let mock: Result<bool, &str> = Ok(true);
    match mock {
        Ok(_) => {
            let data: Losses = serde_json::from_str(
                "{\"message\":\"The data were fetched successfully.\",\"data\":{\"day\":297,\"resource\":\"https:\\/\\/www.facebook.com\\/MinistryofDefence.UA\\/posts\\/pfbid0J6FyCXR4hbuQ1HDDvvCXdwhjU72eg7CJ9xUP47onFBvF2xGs6W5MaGtfBm2eas7Bl\",\"stats\":{\"personnel_units\":97690,\"tanks\":2985,\"armoured_fighting_vehicles\":5958,\"artillery_systems\":1947,\"mlrs\":410,\"aa_warfare_systems\":211,\"planes\":281,\"helicopters\":264,\"vehicles_fuel_tanks\":4577,\"warships_cutters\":16,\"cruise_missiles\":653,\"uav_systems\":1648,\"special_military_equip\":174,\"atgm_srbm_systems\":4},\"increase\":{\"personnel_units\":420,\"tanks\":5,\"armoured_fighting_vehicles\":6,\"artillery_systems\":1,\"mlrs\":0,\"aa_warfare_systems\":0,\"planes\":0,\"helicopters\":0,\"vehicles_fuel_tanks\":14,\"warships_cutters\":0,\"cruise_missiles\":61,\"uav_systems\":0,\"special_military_equip\":2,\"atgm_srbm_systems\":0}}}",
            )
                .unwrap();
            Ok(data)
        }
        Err(_) => Err(String::from("Our service is unavailable for you now")),
    }
    */
}

pub fn print_stats(data: Data) {
    let mut table = Vec::new();
    table.push(("orcs", data.stats.personnel_units));
    table.push(("artillery systems", data.stats.artillery_systems));
    table.push(("helicopters", data.stats.helicopters));
    table.push(("warships/cutters", data.stats.warships_cutters));
    table.push(("cruise missiles", data.stats.cruise_missiles));
    table.push(("special military equip", data.stats.special_military_equip));
    table.push(("vehicles and fuel tanks", data.stats.vehicles_fuel_tanks));
    table.push(("MLRS", data.stats.mlrs));
    table.push(("AA warfare systems", data.stats.aa_warfare_systems));
    table.push(("ATGM/SRBM systems", data.stats.atgm_srbm_systems));
    table.push(("tanks", data.stats.tanks));
    table.push(("UAV systems", data.stats.uav_systems));
    table.push(("planes", data.stats.planes));
    table.push(("AFV", data.stats.armoured_fighting_vehicles));

    draw(
        [
            format!("The fascists' losses on the {} day", data.day),
            String::from("of the war amounted to:"),
        ]
        .to_vec(),
        table,
    );
}

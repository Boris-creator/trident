use colored::{control, Colorize};
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct District {
    title: String,
    enabled: bool,
    enabled_at: Option<String>,
    state: String,
}
#[derive(Debug, Deserialize)]
pub struct Region {
    enabled: bool,
    enabled_at: Option<String>,
    //disabled_at: Option<String>,
    districts: Option<HashMap<String, Region>>,
}
#[derive(Debug, Deserialize)]
pub struct Map {
    pub states: HashMap<String, Region>,
}
#[derive(Debug, Deserialize)]
pub struct Alerts {
    states: Vec<String>,
    districts: Vec<District>,
}
pub async fn fetch() -> Result<Map, String> {
    let res = reqwest::get("https://emapa.fra1.cdn.digitaloceanspaces.com/statuses.json")
        .await
        .unwrap()
        .text()
        .await;
    match res {
        Ok(r) => {
            let data: Map = serde_json::from_str(&r).unwrap();
            Ok(data)
        }
        Err(_) => Err(String::from("Our service is unavailable for you now")),
    }
}
pub fn filter_regions(regions: Map) -> Alerts {
    let mut state_alerts = Vec::new();
    let mut district_alerts = Vec::new();
    for (region, value) in regions.states {
        if value.enabled {
            state_alerts.push(String::from(&region))
        }
        let districts = value.districts;
        match districts {
            Some(d) => {
                for (key, value) in d {
                    if value.enabled {
                        district_alerts.push(District {
                            title: key,
                            enabled: true,
                            enabled_at: value.enabled_at,
                            state: String::from(&region),
                        })
                    }
                }
            }
            None => {}
        }
    }
    Alerts {
        states: state_alerts,
        districts: district_alerts,
    }
}
pub fn print_sirens(alerts: Alerts) {
    let mut output = String::new();
    for state in alerts.states {
        output.push_str(&state);
        output.push_str("\n")
    }
    for district in alerts.districts {
        output.push_str(&format!("{}, {}\n", district.state, district.title));
    }
    control::set_virtual_terminal(true).unwrap();
    println!("{}", output.red());
}

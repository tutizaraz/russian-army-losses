use chrono::Local;
use colored::*;
use prettytable::{format, Table, row};
use serde::{Deserialize, Serialize};
use url::Url;
use clap::{App, Arg};

#[derive(Serialize, Deserialize)]
struct Statistics {
    personnel_units: i32,
    tanks: i32,
    armoured_fighting_vehicles: i32,
    artillery_systems: i32,
    mlrs: i32,
    aa_warfare_systems: i32,
    planes: i32,
    helicopters: i32,
    vehicles_fuel_tanks: i32,
    warships_cutters: i32,
    cruise_missiles: i32,
    uav_systems: i32,
    special_military_equip: i32,
    atgm_srbm_systems: i32,
}

const URL: &str = "https://russianwarship.rip/api/v2/statistics/latest";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("shoporusni CLI")
        .version("1.0")
        .author("Dmytro Barabash <dbarabashdev@gmail.com>")
        .about("Displays the latest statistics from the Russian military.")
        .arg(
            Arg::with_name("shoporusni")
                .short('s')
                .long("shoporusni")
                .help("Displays the latest statistics from the Russian military."),
        )
        .get_matches();

    if matches.is_present("shoporusni") {
        let statistics = get_latest_statistics().await?;
        print_statistics(&statistics);
        print_donation_link();
        println!("Glory to Ukraine 🇺🇦");
    }

    Ok(())
}

async fn get_latest_statistics() -> Result<Statistics, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.get(URL).send().await?;
    let data = serde_json::from_str::<serde_json::Value>(&res.text().await?)?;
    let stats = data["data"]["stats"].clone().to_string();
    let statistics = serde_json::from_str::<Statistics>(&stats)?;

    Ok(statistics)
}

fn print_statistics(statistics: &Statistics) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["Statistic", "Value"]);
    table.add_row(row!["Personnel Units", statistics.personnel_units]);
    table.add_row(row!["Tanks", statistics.tanks]);
    table.add_row(row!["Armoured Fighting Vehicles", statistics.armoured_fighting_vehicles]);
    table.add_row(row!["Artillery Systems", statistics.artillery_systems]);
    table.add_row(row!["MLRS", statistics.mlrs]);
    table.add_row(row!["AA Warfare Systems", statistics.aa_warfare_systems]);
    table.add_row(row!["Planes", statistics.planes]);
    table.add_row(row!["Helicopters", statistics.helicopters]);
    table.add_row(row!["Vehicles Fuel Tanks", statistics.vehicles_fuel_tanks]);
    table.add_row(row!["Warships/Cutters", statistics.warships_cutters]);
    table.add_row(row!["Cruise Missiles", statistics.cruise_missiles]);
    table.add_row(row!["UAV Systems", statistics.uav_systems]);
    table.add_row(row!["Special Military Equip", statistics.special_military_equip]);
    table.add_row(row!["ATGM/SRBM Systems", statistics.atgm_srbm_systems]);

    println!("Latest Statistics as of {}: ", Local::now().to_string().green());
    table.printstd();
}

fn print_donation_link() {
    let link = "https://savelife.in.ua/en/donate-en";
    let url = Url::parse(link).unwrap();
    let formatted_link_come_back_alive = format!("{}", url);

    println!("Click here to donate: \x1b[32m{}\x1b[0m", formatted_link_come_back_alive);
}

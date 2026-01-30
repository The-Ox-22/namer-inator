use actix_web::{web, App, HttpServer};
use serde::Deserialize;
use std::collections::HashMap;

#[path = "requests.inator.rs"]
mod requests;

use requests::{random_inator, random_inator_by_season, random_inator_pure, AppState};

#[derive(Debug, Deserialize)]
struct InatorsList {
    season_1: Vec<String>,
    season_2: Vec<String>,
    season_3: Vec<String>,
    season_4: Vec<String>,
    season_5: Vec<String>,
    outside_main_series: Vec<String>,
    pure_inators: Vec<String>,
}

fn load_inators() -> HashMap<String, Vec<String>> {
    let content = std::fs::read_to_string("names-list.inator")
        .expect("Failed to read names-list.inator file");
    let inators: InatorsList = serde_json::from_str(&content)
        .expect("Failed to parse names-list.inator as JSON");

    let mut map = HashMap::new();
    map.insert("season_1".to_string(), inators.season_1);
    map.insert("season_2".to_string(), inators.season_2);
    map.insert("season_3".to_string(), inators.season_3);
    map.insert("season_4".to_string(), inators.season_4);
    map.insert("season_5".to_string(), inators.season_5);
    map.insert("outside_main_series".to_string(), inators.outside_main_series);
    map.insert("pure".to_string(), inators.pure_inators);
    map
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let inators = load_inators();
    let total: usize = inators.values().map(|v| v.len()).sum();
    log::info!("Loaded {} inators across {} categories", total, inators.len());

    let app_state = web::Data::new(AppState { inators });

    log::info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(random_inator)
            .service(random_inator_pure)
            .service(random_inator_by_season)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

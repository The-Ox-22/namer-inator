use actix_web::{HttpResponse, Responder, get, web};
use rand::seq::SliceRandom;
use serde::Serialize;
use std::collections::HashMap;

use crate::format::{FormatQuery, apply_format};

pub struct AppState {
    pub inators: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize)]
struct RandomInatorResponse {
    inator: String,
}

fn pick_random(inators: &[String]) -> Option<String> {
    let mut rng = rand::thread_rng();
    inators.choose(&mut rng).cloned()
}

#[get("/random-inator")]
pub async fn random_inator(
    data: web::Data<AppState>,
    query: web::Query<FormatQuery>,
) -> impl Responder {
    let all_inators: Vec<&String> = data.inators.values().flatten().collect();

    if all_inators.is_empty() {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": "No inators available"
        }));
    }

    let mut rng = rand::thread_rng();
    let random_inator = all_inators.choose(&mut rng).unwrap();

    HttpResponse::Ok().json(RandomInatorResponse {
        inator: apply_format(random_inator, &query),
    })
}

#[get("/random-inator/pure")]
pub async fn random_inator_pure(
    data: web::Data<AppState>,
    query: web::Query<FormatQuery>,
) -> impl Responder {
    match data.inators.get("pure") {
        Some(inators) if !inators.is_empty() => match pick_random(inators) {
            Some(inator) => HttpResponse::Ok().json(RandomInatorResponse {
                inator: apply_format(&inator, &query),
            }),
            None => HttpResponse::NotFound().json(serde_json::json!({
                "error": "No pure inators available"
            })),
        },
        _ => HttpResponse::NotFound().json(serde_json::json!({
            "error": "No pure inators available"
        })),
    }
}

#[get("/random-inator/{season}")]
pub async fn random_inator_by_season(
    data: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<FormatQuery>,
) -> impl Responder {
    let season = path.into_inner();

    match data.inators.get(&season) {
        Some(inators) if !inators.is_empty() => match pick_random(inators) {
            Some(inator) => HttpResponse::Ok().json(RandomInatorResponse {
                inator: apply_format(&inator, &query),
            }),
            None => HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("No inators available for {}", season)
            })),
        },
        _ => {
            let valid_seasons: Vec<&String> = data.inators.keys().collect();
            HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("Unknown season: {}", season),
                "valid_options": valid_seasons
            }))
        }
    }
}

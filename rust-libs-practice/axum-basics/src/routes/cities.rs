use axum::{
    Router, 
    routing::get, 
    extract::Json
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Cities {
    cities: Vec<City>,
}

#[derive(Serialize, Deserialize)]
struct City {
    name: String,
    details: Details,
}

#[derive(Serialize, Deserialize)]
struct Details {
    country: String,
    continent: String,
}

async fn get_cities_raw() -> Json<Cities> {
    let city_data = r#"{
        "cities": [
            { 
                "name": "Seoul",
                "details": { 
                    "country": "South Korea",
                    "continent": "Asia"
                }
            },
            {
                "name": "Paris",
                "details": { 
                    "country": "France",
                    "continent": "Europe"
                }
            },
            {
                "name": "New York",
                "details": {
                    "country": "USA",
                    "continent": "North America"
                }
            }
        ]
    }"#;
    let data: Cities = serde_json::from_str(city_data).unwrap();
    Json(data)
}

async fn get_cities_struct() -> Json<Cities> {
    let data = Cities {
        cities: vec![
            City {
                name: "Seoul".to_string(),
                details: Details {
                    country: "South Korea".to_string(),
                    continent: "Asia".to_string(),
                },
            },
            City {
                name: "Paris".to_string(),
                details: Details {
                    country: "France".to_string(),
                    continent: "Europe".to_string(),
                },
            },
            City {
                name: "New York".to_string(),
                details: Details {
                    country: "USA".to_string(),
                    continent: "North America".to_string(),
                },
            },
        ],
    };
    Json(data)
}

use serde_json::{json, Value};

async fn get_cities_json_macro() -> Json<Value> {    
    let data = json!({
        "cities": [
            {
                "name": "Seoul",
                "details": {
                    "country": "South Korea",
                    "continent": "Asia"
                }
            },
            {
                "name": "Paris",
                "details": {
                    "country": "France",
                    "continent": "Europe"
                }
            },
            {
                "name": "New York",
                "details": { 
                    "country": "USA",
                    "continent": "North America" 
                }
            }
        ]
    });
    Json(data)
}

pub fn create_router() -> Router {
    Router::new()
        .nest(
            "/cities",
            Router::new()
                .route("/raw", get(get_cities_raw))
                .route("/struct", get(get_cities_struct))
                .route("/json-macro", get(get_cities_json_macro))
        )
}
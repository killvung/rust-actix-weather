use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use w::domain::models::weather::{Weather, WeatherRequest};

#[get("/fetch_weather")]
async fn fetch_weather(web::Query(info): web::Query<WeatherRequest>) -> impl Responder {
    if let Some(city) = info.city {
        let weather_data = get_weather_data(&city).await;
        match weather_data {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(err) => HttpResponse::InternalServerError().body(err.without_url().to_string()),
        }
    } else {
        HttpResponse::BadRequest().body("Missing 'city' parameter")
    }
}

async fn get_weather_data(city: &str) -> Result<Weather, reqwest::Error> {
    let api_key = "[Use your own api key]";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    );

    let response = reqwest::get(&url).await?;
    response.error_for_status_ref()?;

    let json_data: serde_json::Value = response.json().await?;

    let main = json_data["main"]
        .as_object()
        .ok_or("Invalid JSON response")
        .unwrap();
    let weather = json_data["weather"][0]
        .as_object()
        .ok_or("Invalid JSON response")
        .unwrap();

    let low_temperature = main["temp_min"]
        .as_f64()
        .ok_or("Invalid temperature")
        .unwrap() as f32;
    let high_temperature = main["temp_max"]
        .as_f64()
        .ok_or("Invalid temperature")
        .unwrap() as f32;
    let humidity = main["humidity"].as_f64().ok_or("Invalid humidity").unwrap() as f32;
    let condition = weather["main"]
        .as_str()
        .ok_or("Invalid condition")
        .unwrap()
        .to_string();

    Ok(Weather {
        city: city.to_string(),
        low_temperature,
        high_temperature,
        humidity,
        condition,
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server on localhost:8080
    HttpServer::new(|| {
        App::new().service(
            web::scope("/weather") // Define a new domain for weather routes
                .service(fetch_weather),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

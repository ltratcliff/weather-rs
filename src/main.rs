use reqwest;
use chrono::{DateTime};
use chrono_tz::America::New_York;
mod weather;

const API_KEY: &str = "FILLMEIN";
const URL: &str = "https://api.openweathermap.org/data/2.5/weather?units=imperial&lang=en&mode=json";
const HOME: (f64, f64) = (27.855821243042406, -82.18882060556116);
//const TAMPA: (f64, f64) = (27.854942903688478, -82.48710218110561);
const FT_THOMAS: (f64, f64) = (39.06051727886154, -84.44961088792509);
const ORLAND: (f64, f64) = (39.75504677980524, -122.18453958789212);

const CITIES: [(f64, f64); 3] = [HOME, FT_THOMAS, ORLAND];

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let api_key = match std::env::var("OPEN_WEATHER_API_KEY") {
    //     Ok(v) => v,
    //     Err(_) => {
    //         eprintln!("OPEN_WEATHER_API_KEY not set");
    //         std::process::exit(1);
    //     },
    // };

    for city in CITIES {
        let url = format!("{}&lat={}&lon={}&appid={}", URL, city.0, city.1, API_KEY);
        let response = reqwest::Client::new().get(&url).send().await?;
        let data: weather::Root = response.json().await?;
        let sunrise = DateTime::from_timestamp(data.sys.sunrise, 0).unwrap()
            .with_timezone(&New_York);
        let sunset = DateTime::from_timestamp(data.sys.sunset, 0).unwrap()
            .with_timezone(&New_York);
        let mut rain = data.rain.unwrap_or(Default::default()).n1h.unwrap_or(0.0);
        if rain > 0.0 {
            rain = rain / 25.4
        }

        // println!("{:?}", data);
        println!("{:<12}: Temp: {:<5} Feels: {:<5} Min: {:<5}  Max: {:<5} Wind: {:<5?} Rain: {:<5.2}\
         Sunrise: {} Sunset: {}",
            data.name, data.main.temp, data.main.feels_like, data.main.temp_min, data.main.temp_max,
            data.wind.unwrap().speed, rain, sunrise.format("%I:%M:%S %p %Z"),
                 sunset.format("%I:%M:%S %p %Z"),
        )
    }

    Ok(())
}

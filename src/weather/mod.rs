use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind: Option<Wind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain: Option<Rain>,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(rename = "sea_level")]
    pub sea_level: i64,
    #[serde(rename = "grnd_level")]
    pub grnd_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rain {
    #[serde(rename = "1h")]
    pub n1h: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clouds {
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

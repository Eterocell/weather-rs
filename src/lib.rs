use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Input {
    pub city: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub coord: Coord,
    pub weather: Weather,
    pub main: Main,
    pub name: String,
    pub sys: Sys,
}

impl Response {
    pub async fn get(city: &String) -> Result<Self, ExitFailure> {
        let appid = String::from("");
        let url = format!(
            "https://api.openweathermap.org/data/2.5/\
    weather?q={}&appid={}",
            city, appid
        );
        let url = Url::parse(&*url)?;
        let respond = reqwest::get(url).await?.json::<Response>().await?;
        Ok(respond)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub weather_details: WeatherDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherDetails {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

pub fn print_response(response: Response) {
    println!(
        "\nCurrent Weather Information in {}, {}:\n\
    Weather: {}\n\
    Current Temperature: {:.2} °C\n\
    Highest Temperature: {:.2} °C\n\
    Lowest Temperature: {:.2} °C\n\
    Feels like: {:.2} °C\n\
    Air Pressure: {} kPa\n\
    Humidity: {}%\n",
        response.name,
        response.sys.country,
        response.weather.weather_details.main,
        response.main.temp - 273.15,
        response.main.temp_max - 273.15,
        response.main.temp_min - 273.15,
        response.main.feels_like - 273.15,
        response.main.pressure,
        response.main.humidity
    )
}

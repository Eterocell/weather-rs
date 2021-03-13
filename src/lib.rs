use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;
#[allow(unused_imports)]
use ansi_term::{Color, Style};
use ansi_term::ANSIGenericString;

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
        // Add your API Key here
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
    Current Temperature: {} °C\n\
    Highest Temperature: {} °C\n\
    Lowest Temperature: {} °C\n\
    Feels like: {:.2} °C\n\
    Air Pressure: {} kPa\n\
    Humidity: {}%\n",
        response.name,
        response.sys.country,
        response.weather.weather_details.main,
        paint_temp(response.main.temp - 273.15),
        paint_temp(response.main.temp_max - 273.15),
        paint_temp(response.main.temp_min - 273.15),
        paint_temp(response.main.feels_like - 273.15),
        response.main.pressure,
        response.main.humidity
    )
}

fn paint_temp<'a>(temp: f64) -> ANSIGenericString<'a, str> {
    #[allow(illegal_floating_point_literal_pattern)]
    let temp = match temp {
        30.0..=100.0 =>Color::Red.paint(format!("{:.2}", temp)),
        20.0..=30.0 => Color::Yellow.paint(format!("{:.2}", temp)),
        10.0..=20.0 => Color::Green.paint(format!("{:.2}", temp)),
        0.0..=10.0 => Color::Cyan.paint(format!("{:.2}", temp)),
        -273.15..=0.0 => Color::Blue.paint(format!("{:.2}", temp)),
        _ => Color::Red.paint(String::from("Numerous Error."))
    };
    temp
}

extern crate dotenv;
extern crate prettytable;
extern crate reqwest;
extern crate serde;
use prettytable::{row, Table};
use serde::Deserialize;
use std::error::Error;

#[cfg(test)]
pub mod test;

#[derive(Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}
#[derive(Deserialize, Debug)]
struct Weather {
    main: String,
}
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

#[derive(Deserialize, Debug)]
struct Rain {
    #[serde(rename = "1h")]
    volumn_1h: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct Snow {
    #[serde(rename = "1h")]
    volumn_1h: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct CurrentWeatherMain {
    temp: f64,
    feels_like: f64,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    id: u32,
    name: String,
    coord: Coord,
    weather: Vec<Weather>,
    main: CurrentWeatherMain,
    wind: Wind,
    rain: Option<Rain>,
    snow: Option<Snow>,
    visibility: Option<u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: weather <CITY>");
        return Ok(());
    }
    let city = &args[1];

    let weather: CurrentWeather = fetch_weather(city)?;
    // println!("weather = {weather:?}");
    forecast(&weather);

    Ok(())
}

fn fetch_weather(city: &str) -> Result<CurrentWeather, reqwest::Error> {
    dotenv::dotenv().unwrap();
    let mut api_key: Option<String> = None;
    for (key, value) in std::env::vars() {
        if key != "APIKEY" {
            continue;
        }
        api_key = Some(value)
    }
    if api_key.is_none() {
        panic!("APIKEY is not available")
    }
    let api_key: String = api_key.unwrap();
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units=metric"
    );
    let weather: CurrentWeather = reqwest::blocking::get(url)?.json()?;

    Ok(weather)
}

fn forecast(weather: &CurrentWeather) {
    let mut table = Table::new();
    table.add_row(row![bFw->"Weather in ", weather.name]);
    table.add_row(row![bFw->"City ID ", weather.id]);
    table.add_row(row![
        "Location",
        format!("{:.4}, {:.4}", weather.coord.lon, weather.coord.lat)
    ]);
    table.add_row(row!["Main Weather", weather.weather[0].main.clone()]);
    table.add_row(row![
        "Visibility (meters)",
        format!("{}", weather.visibility.unwrap_or(0)),
    ]);
    table.add_row(row!["Temperature (°C)", weather.main.temp]);
    table.add_row(row!["Feels Like (°C)", weather.main.feels_like]);
    table.add_row(row!["Wind Speed (m/s)", weather.wind.speed]);
    if let Some(rain) = &weather.rain {
        table.add_row(row![
            "Rain (last 1h)",
            format!("{:.1} mm", rain.volumn_1h.unwrap_or(0.0))
        ]);
    } else {
        table.add_row(row!["Rain (last 3h", "None"]);
    }
    if let Some(snow) = &weather.snow {
        table.add_row(row![
            "Snow (last 1h)",
            format!("{:.1} mm", snow.volumn_1h.unwrap_or(0.0))
        ]);
    } else {
        table.add_row(row!["Snow (last 1h)", "None"]);
    }

    table.printstd()
}

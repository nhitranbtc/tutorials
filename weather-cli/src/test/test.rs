use crate::*;
#[test]
fn test_weather_info() {
    // Test with a known city
    let city = "Aberdeen";
    let weather = fetch_weather(city).unwrap();

    // Assert city ID
    assert_eq!(weather.id, 2657832);

    // Assert city name
    assert_eq!(weather.name, "Aberdeen");

    // Assert longitude and latitude
    assert_eq!(weather.coord.lon, -2.0981);
    assert_eq!(weather.coord.lat, 57.1437);
}
#[test]
fn test_invalid_city() {
    // Test with an invalid city name
    let city = "InvalidCityName";
    let result = fetch_weather(city);

    // Verify that fetching weather data for an invalid city returns an error
    assert!(result.is_err());
}

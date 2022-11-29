use std::fmt::{self, Formatter, Display};

/* Demonstrates printing of a user defined struct using println! macro.*/
///City struct that has fields to represent a city.
/// * name: City name
/// * lat: Cities latitude
/// * lon: Cities longitude
struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}
/// Display implementation gives instructions on how to display this struct when printing.
impl Display for City {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}
///Color struct represents an RGB value with fields
/// * red: Red value out of 255
/// * green: Green value out of 255
/// * blue: Blue value out of 255
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
/// Implementation for Display trait on struct Color
impl Display for Color{
    /// Use the fmt method to build a string representation of our Color struct,
    /// and return a Result with the result or an error.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        write!(f, "red: {},green: {},blue: {}",self.red,self.green,self.blue)
    }
}

fn main() {
    for city in [
        City { name: "Glassboro", lat: 39.702892, lon: -75.111839 },
        City { name: "Mullica Hill", lat: 39.73928, lon: -75.224072 },
        City { name: "Swedesboro", lat: 39.747616, lon: -75.310463 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }
}

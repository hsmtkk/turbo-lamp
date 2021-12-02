use serde::Serialize;

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

impl City {
    fn new(name:&str, population:usize, latitude:f64, longitude:f64) -> City {
        City{name:name.to_string(), population, latitude, longitude}
    }
}

fn main() {
    let calabar = City::new("Calabar", 470000, 4.95, 8.33);
    let as_json = serde_json::to_string(&calabar).unwrap();
    println!("{}", as_json);
}

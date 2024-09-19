const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZING_POINT) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT
}

fn main() {
    let mut temp_fahrenheit: f64 = FREEZING_POINT;

    let mut temp_celsius = fahrenheit_to_celsius(temp_fahrenheit);
    println!("{}°F is equal to {:.2}°C",temp_fahrenheit, temp_celsius);

    for _i in 1..=5 {
        temp_fahrenheit += 1.0;
        temp_celsius = fahrenheit_to_celsius(temp_fahrenheit);
        println!("{}°F is equal to {:.2}°C",temp_fahrenheit, temp_celsius);
    }

    temp_celsius = 3.0;

    for _i in 1..=5 {
        temp_celsius += 1.0;
        temp_fahrenheit = celsius_to_fahrenheit(temp_celsius);
        println!("{}°C is equal to {:.2}°F", temp_celsius, temp_fahrenheit);
    }
}

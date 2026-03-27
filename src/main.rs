fn classify(temp_f: f64) -> &'static str {
    match temp_f {
        t if t < 32.0 => "cold",
        t if t >= 32.0 && t < 50.0 => "cold",
        t if t >= 50.0 && t < 70.0 => "mild",
        t if t >= 70.0 && t < 90.0 => "hot",
        _ => "hot",
    }
}

fn celsius_to_fahrenheit(celcius: f64) -> f64 {
    return (celcius * 9.0 / 5.0) + 32.0;
}

fn main() {
    // let temps = [0.0, 20.0, 35.0];

    // for temp in temps.iter() {
    //     let temp_f = celsius_to_fahrenheit(*temp);
    //     let classification = classify(temp_f);
    //     println!("{temp}°C is {temp_f}°F and is classified as {classification}");
    // }

    for temp in [0.0, 20.0, 35.0] {
        let temp_f = celsius_to_fahrenheit(temp);
        let classification = classify(temp_f);
        println!("{temp}°C is {temp_f}°F and is classified as {classification}");
    }
}

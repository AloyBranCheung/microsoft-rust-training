use std::fmt::Display;

struct Celsius(f64);
struct Fahrenheit(f64);
struct Kelvin(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Celsius> for Kelvin {
    fn from(c: Celsius) -> Self {
        Kelvin(c.0 + 273.15)
    }
}

impl TryFrom<f64> for Kelvin {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0.0 {
            // Kelvin cannot be negative scale starts at absolute zero
            Err("Temperature cannot be negative".to_string())
        } else {
            Ok(Kelvin(value))
        }
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} °C", self.0)
    }
}

impl Display for Fahrenheit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} °F", self.0)
    }
}

impl Display for Kelvin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} K", self.0)
    }
}

fn main() {
    let boiling = Celsius(100.0);
    let f: Fahrenheit = Celsius(100.0).into();
    let k: Kelvin = Celsius(100.0).into();
    println!("{boiling} = {f} = {k}");

    match Kelvin::try_from(-10.0) {
        Ok(k) => println!("{k}"),
        Err(e) => println!("Error: {e}"),
    }
}

// // Solution
// use std::fmt;

// struct Celsius(f64);
// struct Fahrenheit(f64);
// struct Kelvin(f64);

// impl From<Celsius> for Fahrenheit {
//     fn from(c: Celsius) -> Self {
//         Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
//     }
// }

// impl From<Celsius> for Kelvin {
//     fn from(c: Celsius) -> Self {
//         Kelvin(c.0 + 273.15)
//     }
// }

// #[derive(Debug)]
// struct BelowAbsoluteZero;

// impl fmt::Display for BelowAbsoluteZero {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "temperature below absolute zero")
//     }
// }

// impl TryFrom<f64> for Kelvin {
//     type Error = BelowAbsoluteZero;

//     fn try_from(value: f64) -> Result<Self, Self::Error> {
//         if value < 0.0 {
//             Err(BelowAbsoluteZero)
//         } else {
//             Ok(Kelvin(value))
//         }
//     }
// }

// impl fmt::Display for Celsius    { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:.2}°C", self.0) } }
// impl fmt::Display for Fahrenheit { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:.2}°F", self.0) } }
// impl fmt::Display for Kelvin     { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:.2}K",  self.0) } }

// fn main() {
//     let boiling = Celsius(100.0);
//     let f: Fahrenheit = Celsius(100.0).into();
//     let k: Kelvin = Celsius(100.0).into();
//     println!("{boiling} = {f} = {k}");

//     match Kelvin::try_from(-10.0) {
//         Ok(k) => println!("{k}"),
//         Err(e) => println!("Error: {e}"),
//     }
// }

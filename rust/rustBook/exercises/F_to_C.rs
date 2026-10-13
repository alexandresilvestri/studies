use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug)]
enum Unit { C, F }

impl FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "C" => Ok(Unit::C),
            "F" => Ok(Unit::F),
            other => Err(format!("'{other}' is not valid, Choose C or F.")),
        }
    }
}

fn convert_celsius_to_farenheit(in_celsius: i16) -> i16 {
    let in_farenheit: f32 = in_celsius as f32 * 9.0 / 5.0 + 32.0; 
    in_farenheit as i16
}

fn convert_farenheit_to_celsius(in_farenheit: i16) -> i16 {
   let in_celsius: f32 = (in_farenheit as f32 - 32.0) * 5.0 / 9.0;
   in_celsius as i16
}

fn main() {
    loop {
        println!("Put the measurement unit: ");  
            io::stdout().flush().unwrap();
        
        let mut unit = String::new();
            io::stdin() 
            .read_line(&mut unit)
            .expect("Failed to read the response");

        let unit: Unit = match unit.parse::<Unit>() {
            Ok(u) => u,
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        };

        println!("Put the value: ");  
        let mut value = String::new();
        io::stdin() 
            .read_line(&mut value)
            .expect("Failed to read the response");

        let value: i16 = match value.trim().parse::<i16>() {
            Ok(v) => v,
            Err(e) => { 
                eprintln!("Error: {e}");
                continue;
            }
        };

        match unit {
            Unit::C => {
                let temperature = convert_celsius_to_farenheit(value);
                println!("This temp in Farenheit is {temperature}")
            },
            Unit::F => {
                let temperature = convert_farenheit_to_celsius(value); 
                println!("This temp in Celsius is {temperature}");
            },
        }

        break;
    }
}


use std::io;
use std::str::FromStr;

fn convert_celsius_to_farenheit(in_celsius: i16) -> i16 {
   let mut in_farenheit: f32 = in_celsius - 32 * 5 / 9; 
   let in_farenheit = in_farenheit as i16;
}

fn convert_farenheit_to_celsius(in_farenheit: i16) -> i16 {
   let mut in_celsius: f32 = in_farenheit * 9 / 5 + 32;
   let in_celsius = in_celsius as i16;
}

enum unit {
    C,
    F,
}

main() {
    println!("Put the measurement unit: ");  
    let mut unit: char;
    io::stdin() 
        .readline(&mut unit)
        .expect("Failed to read the response");

    println!("Put the value: ");  
    let mut value: i16;
    io::stdin() 
        .readline(&mut value)
        .expect("Failed to read the response");

}

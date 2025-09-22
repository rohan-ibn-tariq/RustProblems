fn main() {

    let temp1 = temprature_converter(100.0, 'F');
    println!("Temperature in Fahrenheit: {temp1}");
    let temp2 = temprature_converter(212.0, 'C');
    println!("Temperature in Celsius: {temp2}");
}

fn temprature_converter(temp: f64 , scale: char) -> f64 {
    match scale {
        'C' | 'c' => (temp - 32.0) * 5.0/9.0,
        'F' | 'f' => (temp * 9.0/5.0) + 32.0,
        _ => {
            println!("Invalid scale provided. Use 'C' for Celsius or 'F' for Fahrenheit.");
            temp
        }
    }
}
use std::io;

fn main() {
    loop {
        let mut metric_system = String::new();

        println!("Celsius => Fahrenheit (input 1)");
        println!("Fahrenheit => Celsius (input 2)");

        io::stdin()
            .read_line(&mut metric_system)
            .expect("Failed to read the line!");

        let _metric_system: i32 = match metric_system.trim().parse() {
            Ok(metric_system) => metric_system,
            Err(_) => continue,
        };

        let mut temperature = String::new();
        println!("Input your temperature: ");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Please input a vaild charachter!");

        let _temperature: f32 = match temperature.trim().parse() {
            Ok(temperature) => temperature,
            Err(_) => continue,
        };
    }
}

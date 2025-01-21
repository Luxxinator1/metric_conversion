use std::io;

fn main() {
    loop {
        println!("Celsius => Fahrenheit (input 1)");
        println!("Fahrenheit => Celsius (input 2)");

        let mut metric_system = String::new();

        io::stdin()
            .read_line(&mut metric_system)
            .expect("Failed to read the line!");

        let metric_system: i32 = match metric_system.trim().parse() {
            Ok(metric_system) => metric_system,
            Err(_) => {
                println!("{metric_system} is not a number!");
                continue;
            }
        };

        println!("Input your temperature: ");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Please input a vaild charachter!");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(temperature) => temperature,
            Err(_) => {
                println!("{temperature} is not a number!");
                continue;
            }
        };

        let celsius: f32;

        if metric_system == 2 {
            // Celsius = (Fahrenheit - 32) / 1.8
            celsius = (temperature - 32.0) / 1.8;
            println!("{celsius}C");
            break;
        }

        let fahrenheit: f32;

        if metric_system == 1 {
            // Fahrenheit = (Celsius * 1.8) + 32
            fahrenheit = (temperature * 1.8) + 32.0;
            println!("{fahrenheit}F");
            break;
        }

        if metric_system != 1 && metric_system != 2 {
            println!("Please input a valid charachter!");
            continue;
        }
    }
}

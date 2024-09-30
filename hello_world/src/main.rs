use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};
use std::path::Path;

struct Car {
    make: String,
    model: String,
    year: u32,
    color: String,
}

impl Car {
    fn new() -> Car {
        let mut input = String::new();

        print!("Enter your car make? ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let make = input.trim().to_string();
        input.clear();
    
        print!("Enter your car model? ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let model = input.trim().parse().unwrap();
        input.clear();

        print!("Enter your car year? ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let year = input.trim().parse().unwrap();
        input.clear();

        print!("Enter your car color? ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let color = input.trim().parse().unwrap();
        
        Car {make, model, year, color}
    }

    fn save_to_file(&self, filename: &str){
        let mut file = File::create(filename).expect("Unable to create file");
        writeln!(file, "Make: {}", self.make).expect("Unable to write to file");
        writeln!(file, "Model: {}", self.model).expect("Unable to write to file");
        writeln!(file, "Year: {}", self.year).expect("Unable to write to file");
        writeln!(file, "Color: {}", self.color).expect("Unable to write to file");
    }
    
    fn read_from_file(filename: &str){
        if Path::new(filename).exists(){
            let file = File::open(filename).expect("Unable to open file");
            let reader = BufReader::new(file);
    
            for line in reader.lines(){
                println!("{}", line.unwrap());
            }
        } else {
            println!("File does not exist.")
        }
    }
}

fn main() {
    let car = Car::new();
    let filename = "user_info.txt";

    car.save_to_file(filename);

    println!("\nContent of the file '{}':", filename);
    Car::read_from_file(filename);
}

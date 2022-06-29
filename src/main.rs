use std::io; // Allows for input and output

fn main() {
    loop {  // Basic loop 
        println!("Enter Tempurature F=>C or C=>F");
        println!("Enter 1 for F=>C");
        println!("Enter 2 for C=>F");
        println!("Enter 3 for Quit");

        let mut entry = String::new(); // mutable variable able to be changed
        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read line");
        let selection: i32 = entry.trim().parse().expect("Input not an integer");

        println!("You entered: {}", entry);
        
        if selection == 1 {
            println!("Enter numeric temperature");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let temp: f32 = input.trim().parse().expect("Input not an integer"); // Conversion of input to float 32 bit integer
            fahtocelcius(temp); // Function call
        }
        else if selection == 2 {
            println!("Enter numeric temperature");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let temp: f32 = input.trim().parse().expect("Input not an integer"); // Conversion of input to float 32 bit integer
            celciustofah(temp); // Function call
        }
        else if selection == 3 {
            break;  // Ends the loop
        }
        else {
            println!("Invalid Entry");
        }
    } 
}

fn fahtocelcius(temp: f32) { // Fahrenheit to Celcuis function
    let result: f32 = (temp - 32.0) * (5.0 / 9.0);
    println!("Result {} degrees Celcius", result)
}
fn celciustofah(temp: f32) { // Celsius to Fahrenheit function
    let result: f32 = (temp * (5.0 / 9.0)) + 32.0;
    println!("Result {} degrees Fahrenheit", result);
}
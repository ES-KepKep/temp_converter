use dialoguer::Input;

fn main() {
    loop {
        println!("Quit by typing Q");

        let input: String = Input::new()
            .with_prompt("From (C)elsius or from (F)arenheit?")
            .interact_text()
            .unwrap();

        let input: char = input.trim().to_uppercase().parse().unwrap();

        match input {
            'C' => {
                println!("{:_^30}", "");
                println!("Selected Celsius");
                let input: String = Input::new()
                    .with_prompt("Input temp")
                    .interact_text()
                    .unwrap();

                let input: f32 = input.trim().parse().unwrap();
                let convertion = (input * 9.0 / 5.0) + 32.0;

                println!("{input} Celsius to Farenheit is: {convertion:.2}");
                println!("{:_^30}", "");
                println!();
            }
            'F' => {
                println!("{:_^30}", "");
                println!("Selected Farenheit");
                let input: String = Input::new()
                    .with_prompt("Input temp")
                    .interact_text()
                    .unwrap();

                let input: f32 = input.trim().parse().unwrap();
                let convertion = (input - 32.0) * 5.0 / 9.0;

                println!("{input} Farenheit to Celcius is: {convertion:.2}");
                println!("{:_^30}", "");
                println!();
            }
            'Q' => {
                break;
            }
            _ => {
                println!("Not Valid Input\n");
            }
        }
    }
}

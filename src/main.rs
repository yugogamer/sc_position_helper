use std::time::Duration;
use std::env;

use clipboard::{ClipboardProvider, ClipboardContext};
use colored::*;
use sc_position_helper::{Coordinate, is_valid, parse_coordinate};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
        println!("Usage: {} <x> <y> <z>", args[0]);
        return;
    }

    let x = args[1].parse::<f64>();
    let y = args[2].parse::<f64>();
    let z = args[3].parse::<f64>();
    
    if x.is_err() || y.is_err() || z.is_err() {
        println!("Usage: {} <x> <y> <z>", args[0]);
        return;
    }

    let saved_cordinate = Coordinate{
        x: x.unwrap(), 
        y: y.unwrap(), 
        z: z.unwrap()
    };
    script(saved_cordinate).await;
}

async fn script(saved_coordinates: Coordinate){
    let mut input: String;
    let mut last_input: String = "".to_string();
    let mut last_distance: f64 = 0.0;
    println!("-----------------------------");
    loop {
        let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
        let clip = clipboard.get_contents();
        if clip.is_err() {
            continue;
        }
        input = clip.unwrap();
        if input == last_input {
            continue;
        }
        last_input = input.clone();
        if is_valid(&input) {
            let coordinate = parse_coordinate(&input);
            match coordinate {
                Ok(coordinate) => {
                    let distance = saved_coordinates.get_distance_between(&coordinate);
                    if distance < last_distance {
                        println!("{}","You are getting closer to the selected point!".green());
                    }else {
                        println!("{}","You are getting further away from the selected point".red());
                    }
                    println!("Current distance : {:.2} Km", distance);

                    last_distance = distance;
                },
                Err(_) => {
                    println!("Coordinate not found");
                    continue;
                },
            }
            println!("-----------------------------");
        }
        //sleep
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
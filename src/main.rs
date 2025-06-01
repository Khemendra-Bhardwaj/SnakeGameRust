mod board;
mod snake;
mod game_engine;
mod food;
use crate::game_engine::GameEngine; 
use crate::snake::Snake;
use crate::board::Board;
use crate::food::Food;


use std::{thread};
use std::io::{self};
use std::sync::mpsc;


fn main() {

    // step1 : initialize board and print value 
    
    let mut classic_board:Board  =  Board::new(10,10);
    classic_board.print_board();

    // step2 : init snake and give movement and print snake positions 
    
    let mut cobra = Snake::new();

    let (sender, recevier) = mpsc::channel::<char> ();
    
    
    let game_engine_thread = thread::spawn(  move || {
        let mut game_engine = GameEngine::new(&mut classic_board, &mut cobra);
        GameEngine::start(&mut game_engine, recevier );
    });


    // main thread and game_engine thread would be communicating through channel 
    // main thread would only be sending inputs to receiver 

    let input_thread = thread::spawn(move || {
        loop{
        print!("Enter direction (U/D/L/R), or q to quit: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim whitespace and get first character
        if let Some(c) = input.trim().chars().next() {
            println!("User Input {}", c); 
            if c == 'q'{
                println!("Shutting Down Game...");
                break; 
            }else{
                sender.send(c).unwrap();
            }
        } else {
            println!("No character entered.");
        }
    }   
    });

    input_thread.join().unwrap();
    game_engine_thread.join().expect("Game Engine Shutting down");

    

}


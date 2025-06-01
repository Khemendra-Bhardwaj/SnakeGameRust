// game engine is contionus thread running in parallel to main thread 
// which  prints the board every 10ms 
// snake passes its corradiates to game engine and its updates the board accodinly 
// food creation and snake healthiness  is also handled by game engine 

use core::slice;
use std::{time::Duration};
use crate::board::Board;
use crate ::food::Food;

use crate::snake::{self, Position, Snake};
use std::sync::mpsc;
use rand::Rng;
use std::thread;
use rand::seq::SliceRandom;




pub struct GameEngine <'a> {
    pub board : &'a mut Board,
    pub snake : &'a mut Snake,
    pub last_direction : char , 
    pub score : i32   ,
    pub food : Food,  
    
}

impl <'a>  GameEngine <'a> {

    pub fn new(board: &'a mut Board, snake: &'a mut Snake) -> Self {
        Self {
            board,
            snake,
            last_direction: 'R',
            score: 0,
            food : Food{
                symbol : 'F',
                pos: Position{
                    x: 4,
                    y : 4 
                }
            }
        }
    }


    pub fn start(  &mut self, receiver : mpsc::Receiver<char>  ){

        loop{

            match receiver.recv_timeout(Duration::from_millis(1000)){
                Ok(c)=>{
                    // received input from user 
                    println!("Received User Input Direction {}", c); 
                    self.last_direction= c;
                }

                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // No input received, continue game logic
                    println!("No User Input Direction "); 
                }
                Err(e) => {
                    println!("Receiver error: {:?}", e);
                    break;
                }

            };  

                         
                    // check if snake eat food 
                    let head = self.snake.snake_body[0];
                    let mut is_grown = false;

                    if head.x  == self.food.pos.x && head.y == self.food.pos.y {
                        is_grown = true;
                        self.score += 1;
                        self.board.board[head.x as usize ][head.y  as usize ] = '*';
                        self.find_and_place_food();
                    }
                    
                    // move the snake
                    self.snake.movement(self.last_direction, is_grown );
                    
                    // print the board 
                    self.place_snake_on_board();
                    self.board.print_board();
                    self.snake.print_snake();


                    // check is snake is dead   
                    if self.is_game_over(){
                        println!("Game Over ! , score {}", self.score);
                        break; 
                    }

                    

            // frame update visible to user 
            thread::sleep(Duration::from_millis(1000) );

        }

    }

    
    fn is_game_over(&mut self) -> bool {
        // todo : run in parallel below fns 
        self.snake.hit_wall() || self.snake.eaten_himself() 
        
    } 
    
    fn place_snake_on_board(& mut self){
        // Clear 
         for row in self.board.board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = '-';
            }
        }
        // Place Snake 
       for pos in &self.snake.snake_body {
            self.board.board[pos.x as usize][pos.y as usize] = '*';
        }
        //  Place food
        let fp = &self.food.pos;
        self.board.board[fp.x as usize][fp.y as usize] = 'F';
    }


    fn find_and_place_food(& mut self){
        // iterate on all board cells store them in a cell and find 
        // a empty place to place food 

        let mut empty_cells : Vec<Position> = Vec::new();

        for (x, row) in self.board.board.iter().enumerate() {
        for (y, &col) in row.iter().enumerate() {
            if col == '-' {
                 empty_cells.push(Position {
                        x: x.try_into().unwrap(), y: y.try_into().unwrap()
                });
            }
        }
    }

    if !empty_cells.is_empty() {
        let idx = rand::thread_rng().gen_range(0..empty_cells.len());
        let pos = empty_cells[idx];
        self.food.pos = empty_cells[idx];
    }


}

    


}
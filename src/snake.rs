use std::collections::HashMap;



// first element of snake body would represent its head

pub struct Snake{
pub  snake_body : Vec<Position>, 
}

#[derive(Copy, Clone, Debug , PartialEq, Eq, Hash )] 
pub struct Position{
   pub  x : i8 ,  
   pub  y : i8 
}

impl Snake{
    pub fn new() -> Self  {
    
        // let board = get_board();
        let x = 4;
        let y = 4;


       let pos = Position{
            x: x ,
            y : y
        };

        let mut snake_body = Vec::new();
        snake_body.push(pos);

         Self{
            snake_body
         }
    }

    pub fn movement(  & mut self,  direction: char , is_grown: bool  ) {
            // remove snake last position 
            // and inserting new first position for snake  

            let head = self.snake_body[0];
            

            let new_head = match direction {
                'U' => Position { x: head.x - 1, y: head.y },
                'D' => Position { x: head.x + 1, y: head.y },
                'L' => Position { x: head.x, y: head.y - 1 },
                'R' => Position { x: head.x, y: head.y + 1 },
                _ => panic!("Invalid direction"),
            }; 

            self.snake_body.insert(0,new_head); 
            
            // removing tail only if we snake hasn't eaten food 
            if !is_grown{
                self.snake_body.pop(); 
            }

    }

    


    pub fn eaten_himself (&self) -> bool   {
        // is list contains duplicates then snake would have eaten himself 
        let mut cells :HashMap< Position  ,i8>  = HashMap::new();

        for pos in self.snake_body.iter() {
            if cells.contains_key(&pos){
                return true 
            }
            cells.insert(*pos,1); 
        }
        false 
    }


    pub fn hit_wall(&self) -> bool {
        let snake_head = self.snake_body[0];
        // todo : pass board configs as argument here 
        if snake_head.x <= 0 || snake_head.x >= 10{
            println!("Snake Has Hit Horizontal Wall");
            return true
        }
        else if  snake_head.y <= 0 || snake_head.y >= 10{
            println!("Snake Has Hit Vertical Wall");
            return true 
        }
        false
    }


    pub fn print_snake(&self){
        println!("Snake body: {:?}", self.snake_body);
    }




}
// 0,0 is top left of the board 
pub struct Board{
    length : i8 ,
    width : i8, 
    pub board : Vec<Vec<char >> 
}


impl Board{
    pub fn new( l  : i8 , w: i8 ) ->  Self {
        Board{
            length : l ,
            width : w, 
            board : vec![ vec!['-' ;  w as usize ] ;  l as usize   ]
        }
    }

    pub fn get_board(&self) -> &Vec<Vec<char>> {
        &self.board
    }

    pub fn print_board(&self){
        // iterate and print the board over here 
        for row in &self.board{
            println!("{:?}", row); 
        }
    }

}
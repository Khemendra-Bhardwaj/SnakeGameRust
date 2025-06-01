use crate::snake::Position;

pub struct Food{
    pub symbol : char, 
    pub pos : Position
}

impl Food{

    pub fn new(pos : Position) -> Food {
        Food{
            symbol: 'F',
            pos : pos   
        }
    }

}
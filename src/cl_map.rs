pub(crate) struct ClMap {
    
    cells: Vec<Vec<char>>,
}

impl ClMap {
    
    /// Creates a new ClMap with the given height and size and initializes it with the null 
    /// character
    pub(crate) fn new(width:usize, height:usize) -> ClMap {
        ClMap {
            cells: vec![vec![0 as char; width]; height]
        }
    }
    
    fn set_cell(&mut self, x: usize, y: usize, value:char) {
        self.cells[x][y] = value;
    }
    
    fn 
}
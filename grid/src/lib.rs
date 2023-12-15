#[derive(Debug,Clone)]
pub struct Grid {
    pub map: Vec<Vec<u8>>
}

impl Grid {
    /// Converts a multi line string into a 2D byte Vector
    pub fn new(inputs: &str) -> Self {
        let lines = inputs.lines();
    
        let mut input_map: Vec<Vec<u8>> = Vec::new();
        for line in lines {
            let bytes: Vec<u8> = line.bytes().collect();
            input_map.push(bytes);
        }
        return  Grid { map: input_map };
    }

    /// Up/Down Left/Right Neighbors, diagonals excluded
    /// 
    /// point: is (y, x)
    pub fn get_cardinal_neighbors(&self, point: &(usize,usize)) -> Vec<(usize,usize)> {
        let mut n: Vec<(usize, usize)> = Vec::with_capacity(4);
        let row_max = self.map.len() - 1;
        let col_max = self.map[0].len() - 1;
        //check up
        if point.0 > 0 {
            n.push((point.0 - 1, point.1));
        }
        //check right
        if point.1 < col_max {
            n.push((point.0, point.1 + 1));
        }
        //check down
        if point.0 < row_max {
            n.push((point.0 + 1, point.1));
        }
        //check left
        if point.1 > 0 {
            n.push((point.0, point.1 - 1));
        }
        return n;
    }

    /// Get Cardnial and Diagnal Neighbors
    /// 
    /// point: is (y, x)
    pub fn get_all_neighbors(&self, point: &(usize,usize)) -> Vec<(usize,usize)> {
        let mut n: Vec<(usize, usize)> = Vec::with_capacity(8);
        n.append(&mut self.get_cardinal_neighbors(point));
        let row_max = self.map.len() - 1;
        let col_max = self.map[0].len() - 1;

        //check up/eft
        if point.0 > 0  && point.1 > 0 {
            n.push((point.0 - 1, point.1 - 1));
        }
        //check up/right
        if point.0 > 0 && point.1 < col_max {
            n.push((point.0 - 1, point.1 + 1));
        }
        //check down/left
        if point.0 < row_max && point.1 > 0 {
            n.push((point.0 + 1,point.1 -1));
        }
        //check down/right
        if point.0 < row_max && point.1 < col_max {
            n.push((point.0 +1, point.1 + 1))
        }

        return n;
    }

    pub fn print_map(&self) {
        for row in self.map.iter() {
            for col in row.iter() {
                print!("{}", char::from(*col));
            }
            println!();
            // println!("{:?}", String::from_utf8(row.clone()).unwrap());
        }
    }

    /// Returns the first (y,x) point of item, None if item is not found in the grid
    /// 
    /// item: is &u8
    pub fn index_of(&self, item: &u8) -> Option<(usize,usize)> { 
        for (i, y) in self.map.iter().enumerate() {
            if let Some(j) = y.iter().position(|col| col == item) {
                return Some((i,j));
            }
        }

        return None;
    }

    /// Returns a a Vec of (y,x) pairs for a given item, Returns an empty Vec if no elements match. Assumes all rows are of equal length
    /// 
    /// item is &u8
    pub fn find_all(&self, item: &u8) -> Vec<(usize,usize)> {
        let width = self.map[0].len();
        return self.map.iter().flatten().enumerate().filter(|x| x.1 == item).map(|(i,_)| {
            ( (i / width), (i % width) )
        }).collect();
    }

    /// Inserts a row into the map at the given index. Consumes the row. Shifts all elements after to the right
    /// 
    /// row is Vec<u8>, index is usize
    pub fn insert_row(&mut self, row: Vec<u8>, index: usize) {
        self.map.insert(index, row);
    }

    /// Inserts the given element along the given column index, shifting all other elements to the right
    pub fn insert_col(&mut self, element: u8, index: usize) {
        for row in self.map.iter_mut() {
            row.insert(index, element);
        }
    }

    /// Returns the column width of the map, returns 0 if the map is empty.
    /// 
    /// Assumes all rows in the map are equal length
    pub fn width(&self) -> usize {
        if self.map.is_empty() {
            return 0;
        } else {
            return self.map[0].len();
        }
    }

    /// Returns the row height of the map, returns 0 if the map is empty.
    pub fn height(&self) -> usize {
        if self.map.is_empty() {
            return 0;
        } else {
            return  self.map.len();
        }
    }

    // Maybe makes more sense to go in a Math lib?
    /// Calculates the manhattan distance |x1 - x2| + |y1 - y2| of two (y,x) points.
    pub fn manhattan_distance(&self, a: &(usize,usize), b: &(usize,usize)) -> usize {
        return a.1.abs_diff(b.1) + a.0.abs_diff(b.0);
    }

    /// Rotates the grid counter clockwise by 90 degrees
    pub fn rotate_ccw(&mut self) {
        let mut new_map: Vec<Vec<u8>> = Vec::new();

        for column in (0..self.width()).into_iter().rev() {
            let row: Vec<u8> = self.map.iter().map(|r| r[column]).collect();
            new_map.push(row);
        }

        self.map = new_map;
    }

    /// Rotates the grid clockwise by 90 degrees
    pub fn rotate_cw(&mut self) {
        let mut new_map: Vec<Vec<u8>> = Vec::new();
        
        for column in 0..self.width() {
            let row: Vec<u8> = self.map.iter().rev().map(|r| r[column]).collect();
            new_map.push(row);
        }

        self.map = new_map;
    }
}

#![allow(warnings)]

mod board {
    pub struct Board {
        pub cells: [u16; 3*3*3*3],
    }

    impl Board {
        pub fn new() -> Board {
            return Board{cells: [0b111_111_111; 3*3*3*3]};
        }

        pub fn init(state: &str) -> Board {
            let mut rtn = Board::new();

            let mut index = 0;
            for c in state.chars() {
                if c == '-' {
                    index += 1;
                    continue;
                }
                if c.is_digit(10) {
                    rtn.cells[index] = (2 as u16).pow(c.to_digit(10).unwrap() - 1);
                    index += 1;
                }     
            }

            return rtn;
        }

        fn coords_to_cell_index(box_x: i32, box_y: i32, sub_x: i32, sub_y: i32) -> usize{
            return (box_y*3*3*3 + box_x*3 + sub_y*3*3 + sub_x) as usize;
        }

        pub fn cell_by_coords(&mut self, box_x: i32, box_y: i32, sub_x: i32, sub_y: i32) -> &mut u16 {
            return &mut (self.cells[Board::coords_to_cell_index(box_x, box_y, sub_x, sub_y)]);
        }

        fn is_cell_known_by_value(val: u16) -> bool {
            return val & (val-1) == 0;
        }

        pub fn is_cell_known(&self, index: usize) -> bool {
            return Board::is_cell_known_by_value(self.cells[index]);
        }

        fn val_to_string(val: u16) -> String {
            let num = (val as f32).log2();
            if Board::is_cell_known_by_value(val)
            { return format!(" {}", num + 1 as f32); }
            return "  ".to_string();
        }

        pub fn print(& self) {
            for box_y in 0..3 {
                for sub_y in 0..3 {
                    for box_x in 0..3 {
                        for sub_x in 0..3 {
                            print!("{}", Board::val_to_string(
                                self.cells[
                                    Board::coords_to_cell_index(box_x, box_y, sub_x, sub_y)
                                ]
                            ));
                        }
                        if box_x != 2 {
                            print!("|");
                        }
                    }
                    println!();
                }
                if box_y != 2 {
                    println!("------+------+------");
                }
            }     
        }
    }
}

mod solver {
    use crate::board::Board;

    fn get_relevant_positions(index: usize) -> [usize; 3*3 + 3*(3-1)*2 - 1] {
        let x = index%(3*3);
        let y = index/(3*3);
        let box_x = index%(3*3)/3;
        let box_y = index/(3*3*3);

        let mut rtn = [0; 3*3 + 3*(3-1)*2 - 1];

        let mut count: usize = 0;
        for i in 0..(3*(3-1)) {
            rtn[count] = y*3*3 + i + ((i/3 >= box_x) as usize)*3;
            count += 1;
        }
        for i in 0..(3*(3-1)) {
            rtn[count] = x + (i + ((i/3 >= box_y) as usize)*3) *3*3;
            count += 1; 
        }

        for i in 0..3*3 {
            let crnt = box_y*3*3*3 + box_x*3 + i%3 + i/3*3*3;
            if crnt != index {
                rtn[count] = crnt;
                count += 1;
            }
        }

        return rtn;
    }

    pub fn remove_non_possibilities(board: &mut Board, index: usize) {
        
        for check_index in get_relevant_positions(index).iter() {
            if board.is_cell_known(*check_index) {
                board.cells[index] = board.cells[index] & !board.cells[*check_index];
                println!("@{:b}", board.cells[index]);
            }
            // println!("#{}", *check_index);
        }
    }
}

use self::board::*;

fn main() {
    let mut board = Board::init(
        "
        6--|1--|--2
        8-1|-9-|---
        -75|-84|---
        
        43-|-2-|561
        518|7--|4-9
        -96|41-|3--
        
        ---|-7-|---
        -6-|-31|-5-
        7-2|54-|6-3");

    board.print();
    println!();
    solver::remove_non_possibilities(&mut board, 45);
    board.print();
    print!("{}, {}", board.is_cell_known(0), board.is_cell_known(1));

}

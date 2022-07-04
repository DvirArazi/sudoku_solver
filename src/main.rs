#![allow(warnings)]

use std::io;

mod boardLayouts {
    pub static easy_0 : &str = "
        6--|1--|--2
        8-1|-9-|---
        -75|-84|---
        
        43-|-2-|561
        518|7--|4-9
        -96|41-|3--
        
        ---|-7-|---
        -6-|-31|-5-
        7-2|54-|6-3";

    pub static medium_0 : &str = "
        64-|-3-|--7
        5-1|-7-|9--
        ---|---|-1-

        --4|9-8|-6-
        -8-|--3|-2-
        ---|4--|---

        4--|157|-3-
        2-8|3--|-4-
        75-|---|-96
        ";

    pub static hard_0 : &str = "
        --7|---|3-2
        2--|--5|-1-
        ---|8-1|4--

        -1-|-96|--8
        76-|---|-49
        ---|---|---

        ---|1-3|---
        8-1|-6-|---
        ---|7--|-63
        ";

    pub static expert_0 : &str = "
        5--|---|-36
        974|---|---
        6--|---|--8

        --2|--6|---
        ---|3--|2-1
        -4-|5--|---

        --5|-4-|-9-
        ---|-97|6--
        ---|---|-7-
        ";

    pub static expert_1 : &str = "
        5--|---|-36
        974|---|---
        6--|---|--8

        --2|--6|---
        ---|3--|2-1
        -4-|5--|---

        --5|-4-|-9-
        ---|-97|6--
        ---|---|---
        ";
}

mod board {
    #[derive(Clone)]
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

        pub fn is_full(& self) -> bool {
            for cell in self.cells.iter() {
                if !Board::is_cell_known_by_value(*cell) {
                    return false;
                }
            }

            return true;
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
    use std::io::{Read, stdin};

    use crate::board::{self, Board};

    fn get_best_guess(board: & Board) -> usize {
        
        let candidates: Vec<u16> = vec![];
        let mut min_count = 3*3;
        let mut rtn = 0;
        for i in 0..board.cells.len() {
            let cell_one_count = board.cells[i].count_ones();
            //yeah, I could check if the cell is known using boars.is_cell_known(i),
            //  but I think using cell_one_count > 1 is a bit faster here
            if (cell_one_count > 1 && cell_one_count < min_count) {
                min_count = cell_one_count;
                rtn = i;
            }
        }

        return rtn;
    }

    fn get_relevant_cells(index: usize) -> [usize; 3*3 + 3*(3-1)*2 - 1] {
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

    fn remove_non_possibilities(board: &mut Board, index: usize) -> bool {
        
        for rel_index in get_relevant_cells(index).iter() {
            //that check is not a must as doing the operation on known cells does nothing, but I think it saves time (maybe)
            if !board.is_cell_known(*rel_index) {            
                board.cells[*rel_index] &= !board.cells[index];
                
                if board.cells[*rel_index].count_ones() == 1 {
                    if !remove_non_possibilities(board, *rel_index) {
                        return false;
                    }
                }
            }
            else if board.cells[*rel_index] == board.cells[index] {
                return false;
            }
        }

        return true;
    }

    fn remove_all_non_possibilities(board: &mut Board) -> bool {
        for i  in 0..board.cells.len() {
            if board.is_cell_known(i) {
                if !remove_non_possibilities(board, i) {
                    return false;
                };
            }
        }

        return true;
    }

    fn get_guess_options(val: &u16) -> Vec<u16> {
        let mut rtn = Vec::new();
        
        for i in 0..(3*3) {
            let i = 2_u16.pow(i);
            if i == val & i {
                rtn.push(i);
            }
        }

        return rtn;
    }

    fn get_solutions(board: & Board) -> Vec<Board> {
        let mut rtn = Vec::new();

        let guess_index = get_best_guess(&board);
        
        for guess in get_guess_options(&board.cells[guess_index]).iter() {
            let mut guess_board = board.clone();
            guess_board.cells[guess_index] = *guess;

            if remove_non_possibilities(&mut guess_board, guess_index) {
                if guess_board.is_full() {
                    rtn.push(guess_board);
                }
                else {
                    rtn.append(&mut get_solutions(&guess_board));
                }
            }
        }

        return rtn;
    }

    pub fn solve(board: &Board) -> Vec<Board> {
        let mut start_board = board.clone();
        remove_all_non_possibilities(&mut start_board);

        return get_solutions(&start_board);
    }
}

use std::println;

use boardLayouts::easy_0;

use self::board::*;

fn main() {
    let mut board = Board::init(boardLayouts::expert_0);

    board.print();
    println!();

    for board in solver::solve(&mut board).iter() {
        board.print();
        println!();
    }

}

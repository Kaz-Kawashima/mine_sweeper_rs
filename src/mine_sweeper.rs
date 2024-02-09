use std::{io, process::exit};
use rand::Rng;
use crate::panel::{new_blank_panel, new_boarder_panel, new_bomb_panel, OpenResult, OpenState, Panel};

pub struct GameBoard
{    
    pub panel_mat :Vec<Vec<Box<dyn Panel>>>,
    pub size_x :usize,
    pub size_y :usize,
    pub field_size_x: usize,
    pub field_size_y: usize,
}

impl GameBoard {
    pub fn new(&mut self, y:usize, x:usize, num_bomb:usize) {
        self.size_x = x;
        self.size_y = y;
        self.field_size_x = x + 2;
        self.field_size_y = y + 2;
        // Fill Panel
        for row in 0 .. self.field_size_y {
            let mut panel_row:Vec<Box<dyn Panel>> = Vec::new();
            for col in 0 .. self.field_size_x {
                let b = Box::new(new_blank_panel());
                panel_row.push(b);
            }
            self.panel_mat.push(panel_row);
        }
        // Fill Boarder
        for row in 0 .. self.field_size_y{
            self.panel_mat[row][0] = Box::new(new_boarder_panel());
            self.panel_mat[row][self.field_size_x - 1] = Box::new(new_boarder_panel());
        }
        for col in 0 .. self.field_size_x{
            self.panel_mat[0][col] = Box::new(new_boarder_panel());
            self.panel_mat[self.field_size_y - 1][col] = Box::new(new_boarder_panel());
        }
        // SetBomb
        self.set_bomb(num_bomb);
        self.calc_bomb_value_all();
    }

    fn set_bomb(&mut self, num_bomb:usize) {
        let mut rng = rand::thread_rng();
        let mut counter = 0;
        while counter < num_bomb { 
            let row = rng.gen_range(1..self.size_y);
            let col = rng.gen_range(1..self.size_x);
            let p = &self.panel_mat[row][col];
            if !p.is_bomb() {
                self.panel_mat[row][col] = Box::new(new_bomb_panel());
                counter += 1;
            }
        }
    }

    fn calc_bomb_value(&mut self, y: usize, x: usize){
        let mut counter = 0;
        for row in (y - 1) ..= (y + 1) {
            for col in (x - 1) ..= (x + 1) {
                let p = &self.panel_mat[row][col];
                if p.is_bomb() {
                    counter += 1
                }
            }
        }
        self.panel_mat[y][x].set_bomb_value(counter);
    }

    fn calc_bomb_value_all(&mut self) {
        for row in 1 ..= self.size_y {
            for col in 1 ..= self.size_x {
                let p = &mut self.panel_mat[row][col];
                if !p.is_bomb() {
                    self.calc_bomb_value(row, col);                  
                }
            }
        }
    }

    pub fn print(&self) {
        let mut output = String::new();
        for panel_row in self.panel_mat.iter(){
            for panel in panel_row.iter(){
                output += &panel.to_str();
                output += " "
            }
            output += "\n"
        }
        print!("{}", output);
    }

    pub fn user_input(&self) -> (usize,usize) {
        let mut line = String::new();
        let mut x: usize;
        let mut y: usize;
        loop {
            println!("input y");
            io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            y = line.trim().parse().expect("bbb");
            line.clear();
            if 0 < y && y <= self.size_y{
                break;
            }
        }
        loop {
            println!("input x");
            io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            x = line.trim().parse().expect("aaa");
            line.clear();
            if 0 < x && x <= self.size_x{
                break;
            }
        }
        println!("input y={} x={}", y, x);
        (y, x)
    }

    pub fn open(&mut self, y:usize, x:usize) -> OpenResult {
        let p = &mut self.panel_mat[y][x];
        match p.get_flag_state() {
            crate::panel::FlagState::Flag => {
                OpenResult::Safe
            }
            crate::panel::FlagState::NoFlag => {
                p.open()
            }
        }
    }

    pub fn flag(&mut self, y: usize, x: usize){
        let p = &mut self.panel_mat[y][x];
        p.flag();
    }

    pub fn is_finished(&self) -> bool {
        for row in 1..=self.size_y{
            for col in 1..self.size_x{
                let p = &self.panel_mat[row][col];
                match p.get_open_state() {
                    crate::panel::OpenState::Open => {}
                    crate::panel::OpenState::Close => {
                        if !p.is_bomb(){
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn open_around(&mut self, y: usize, x: usize) -> u8 {
        let mut counter = 0;
        for row in (y - 1) ..= (y + 1){
            for col in (x - 1) ..= (x + 1){
                let p: &mut Box<dyn Panel> = &mut self.panel_mat[row][col];
                match p.get_open_state() {
                    OpenState::Open => {},
                    OpenState::Close => {
                        p.open();
                        counter += 1;
                    }
                }
            }
        }
        counter
    }

    fn cascade_open(&mut self) {
        loop {
            let mut new_open = 0;
            for row in 1 ..= self.size_y{
                for col in 1 ..= self.size_x{
                    let p = &self.panel_mat[row][col];
                    match p.get_open_state() {
                        OpenState::Close => {},
                        OpenState::Open => {
                            if p.get_bomb_value().unwrap() == 0 {
                                new_open += self.open_around(row, col);
                            }
                        },
                    }
                }
            }
            if new_open == 0 {
                break;
            }
        }
    }

    fn open_bombs(&mut self){
        for panel_row in self.panel_mat.iter_mut(){
            for panel in panel_row.iter_mut(){
                if panel.is_bomb() {
                    panel.open();
                }
            }
        }
    }
}

pub fn mine_sweeper_cui() {
    let mut gb = GameBoard{panel_mat:Vec::new(), size_y:0, size_x:0, field_size_y:0, field_size_x:0};
    gb.new(9, 9, 10);
    loop {
        gb.print();
        let (y, x) = gb.user_input();
        let open_result = gb.open(y, x);
        match open_result {
            OpenResult::Bomb => {
                gb.print();
                gb.open_bombs();
                println!("Game Ovaer!");
                exit(0);
            }
            OpenResult::Safe => {
                gb.cascade_open();
            }
        }
        if gb.is_finished() {
            break;
        }
    }
    gb.print();
    println!("You Win!");
}


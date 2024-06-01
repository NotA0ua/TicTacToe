use std::io::stdin;
use std::iter::Iterator;
use std::str::FromStr;
use rand::Rng;

pub fn build_game() -> Game {
    Game {
        field: [[None, None, None], [None, None, None], [None, None, None]],
        player_form: match rand::thread_rng().gen_range(1..=2) {
            1 => Forms::Cross,
            2 => Forms::Circle,
            _ => Forms::Cross,
        }
    }
}

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Forms {
    Cross,
    Circle,
}

pub struct Game {
    player_form: Forms,
    field: [[Option<Forms>; 3]; 3],
}

impl Game {
    pub fn run(&mut self) {
        if self.player_form == Forms::Cross{
            loop {
                self.draw();
                self.input();
                if self.win() { // remove bot turn if player win
                    break;
                };
                self.bot_turn();
                if self.win() { // check bot win
                    break;
                };
            }
        } else {
            loop {
                self.bot_turn();
                if self.win() { // remove player turn if bot win
                    break;
                };
                self.draw();
                self.input();
                if self.win() { // check bot win
                    break;
                };
            }
        }
    }

    fn draw(&self) {
        print!("{}[2J", 27 as char); // wipe terminal
        println!("{}", self.field());
        println!("\n\nВ какую клетку вы хотите поставить {}?", match self.player_form {
            Forms::Cross => "x",
            Forms::Circle => "o",
        });
    }

    fn input(&mut self) {
        let mut index: String = String::new();
        stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        if let Ok(num) = index.trim().parse(){
            let index: usize = num;
            if (index >= 1) && (index <= 9) {
                self.change_field(index, match self.player_form {
                    Forms::Cross => Some(Forms::Cross),
                    Forms::Circle => Some(Forms::Circle),
                }).unwrap();
            }
        };
    }

    fn bot_turn(&mut self) {
        loop {
            let index: usize = rand::thread_rng().gen_range(1..=9);
            let bot_form: Option<Forms> = match self.player_form {
                Forms::Cross => Some(Forms::Circle),
                Forms::Circle => Some(Forms::Cross),
            };
            match self.change_field(index, bot_form) {
                Ok(_) => {break},
                Err(_) => {continue},
            }
        }
    }

    fn win(&self) -> bool {
        let (result, player) = self.is_win();
        let player = match player {
            Forms::Cross => "x",
            Forms::Circle => "o",
        };
        if result {
            self.draw();
            println!("{player} player won!");
        }
        result
    }

    fn is_win(&self) -> (bool, Forms) {
        for player in [Forms::Cross, Forms::Circle]{
            for i in 0..3 {
                // Check rows and columns
                if self.field[i][0] == Some(player) && self.field[i][1] == Some(player) && self.field[i][2] == Some(player) {
                    return (true, player);
                } else if self.field[0][i] == Some(player) && self.field[1][i] == Some(player) && self.field[2][i] == Some(player) {
                    return (true, player);
                }
            }

            // Check diagonals
            if self.field[0][0] == Some(player) && self.field[1][1] == Some(player) && self.field[2][2] == Some(player) {
                return (true, player);
            }

            if self.field[0][2] == Some(player) && self.field[1][1] == Some(player) && self.field[2][0] == Some(player) {
                return (true, player);
            }
        }
        return (false, Forms::Cross);
    }

    fn field(&self) -> String {
        let mut formatted_field: String = String::new();
        for row in &self.field {
            for (i, cell) in row.iter().enumerate() {
                formatted_field.push(match cell {
                    Some(Forms::Circle) => char::from_str("o").unwrap(),
                    Some(Forms::Cross) => char::from_str("x").unwrap(),
                    None => " ".chars().next().unwrap(),
                });
                if i < 2{
                    formatted_field.push_str("|");
                }
            }
            formatted_field.push_str("\n")
        }
        formatted_field
    }

    fn change_field(&mut self, index: usize, form: Option<Forms>) -> Result<&str, &str>{
        let x: usize = (index - 1) % 3;
        let y: usize = (index - 1) / 3;
        if self.field[y][x].is_none() {
            self.field[y][x] = form;
            return Ok("Success");
        };
        return Err("Already taken");

    }
}

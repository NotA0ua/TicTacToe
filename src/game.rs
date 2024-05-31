use std::io::stdin;
use std::iter::Iterator;
use std::str::FromStr;
use rand::Rng;

pub fn build_game(form: Forms) -> Game {
    Game {
        field: [[None, None, None], [None, None, None], [None, None, None]],
        player_form: form
    }
}

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
        'main: loop {
            self.draw();
            self.input();
            // self.is_win(); // remove bot turn if player win
            self.bot_turn();
            // self.is_win(); // check bot win
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

    // fn is_win(&self) {
    //     for player in (Forms::Cross, Forms::Circle) {
    //         for i in 0..3 {
    //             if board[i][0] == player && board[i][1] == player && board[i][2] == player {
    //                 return true;
    //             }
    //         }
    //
    //         // Check columns
    //         for i in 0..3 {
    //             if board[0][i] == player && board[1][i] == player && board[2][i] == player {
    //                 return true;
    //             }
    //         }
    //
    //         // Check diagonals
    //         if board[0][0] == player && board[1][1] == player && board[2][2] == player {
    //             return true;
    //         }
    //
    //         if board[0][2] == player && board[1][1] == player && board[2][0] == player {
    //             return true;
    //         }
    //
    //         false
    //     }
    // }

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

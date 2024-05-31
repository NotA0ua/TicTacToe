use std::io::stdin;
use std::iter::Iterator;
use std::str::FromStr;

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
        loop {
            self.draw();
            self.input();
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
                self.change_field(index);
            }
        };
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

    fn change_field(&mut self, index: usize) {
        let x = (index - 1) % 3;
        let y = (index - 1) / 3;
        self.field[y][x] = match self.player_form {
            Forms::Cross => Some(Forms::Cross),
            Forms::Circle => Some(Forms::Circle),
        };
    }
}

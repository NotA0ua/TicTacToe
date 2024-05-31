use std::io::stdin;
use std::iter::Iterator;
use std::str::FromStr;

pub fn build_game() -> Game {
    Game {
        field: [[Some(Forms::Circle), None, None], [None, Some(Forms::Cross), None], [None, None, None]],
    }
}

pub enum Forms {
    Cross,
    Circle,
}

pub struct Game {
    field: [[Option<Forms>; 3]; 3],
}

impl Game {
    pub fn run(&mut self) {
        // loop {
            self.draw();
            // self.input();
        // }
    }

    fn draw(&self) {
        print!("{}[2J", 27 as char); // wipe terminal
        println!("{}", self.field());
        println!("\n\nВ какую клетку вы хотите поставить X:")
    }

    // fn input(&mut self) {
    //     let mut index = String::new();
    //     stdin().read_line(&mut index).expect("Неправильная строка!");
    //     let index: usize = index.parse().expect("Нужно ввести число!");
    //     if (1usize >= index) && (index <= 9usize) {
    //         self.change_field(index)
    //     } else {
    //         panic!("Плохой индекс!")
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

    // fn change_field(&mut self, index: usize) {
    //     let x = index & 3;
    //     let y = index / 3;
    //     self.field[y][x] = Option::from("x".chars().next().unwrap());
    // }
}

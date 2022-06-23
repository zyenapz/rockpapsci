use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Write},
    str::FromStr,
};

use rand::prelude::SliceRandom;

#[derive(Eq, Hash, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Rock => write!(f, "rock"),
            Move::Paper => write!(f, "paper"),
            Move::Scissors => write!(f, "scissors"),
        }
    }
}

impl Move {
    fn beats(&self, other: &Move) -> bool {
        let mut wins_against: HashMap<Move, Move> = HashMap::new();

        wins_against.insert(Move::Rock, Move::Scissors);
        wins_against.insert(Move::Paper, Move::Rock);
        wins_against.insert(Move::Scissors, Move::Paper);

        return wins_against[&self] == *other;
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "rock" => Ok(Move::Rock),
            "paper" => Ok(Move::Paper),
            "scissors" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

macro_rules! is_alphabetic {
    ($str: expr) => {
        $str.chars().all(|c| matches!(c, 'a'..='z'))
    };
}

macro_rules! alter_introduction {
    {$intro: expr, $tag: expr} => {
        $intro = format!("\n{} Again...rock, paper, scissors!", $tag);
    }
}

fn main() {
    // data
    let moves: Vec<Move> = vec![Move::Rock, Move::Paper, Move::Scissors];

    // variables
    let mut computer_move = moves.choose(&mut rand::thread_rng());
    let game_tag: String = String::from("[GAME]:");
    let mut introduction = format!("{} Let's go...rock, paper, scissors!", game_tag);

    loop {
        // Create new player move String
        let mut player_move = String::new();

        // println!("{}", computer_move.unwrap());

        // Get the player's move
        print!("{}\nEnter your move:  ", introduction);

        io::stdout().flush().expect("Can't flush stdout!");

        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line!");

        // Prettify the string. Really just pre-processing it.
        let pmove_pretty: String = player_move.trim().to_lowercase();

        // Do some validation
        match is_alphabetic!(pmove_pretty)
            & moves.iter().any(|i: &Move| i.to_string() == pmove_pretty)
        {
            true => {
                // Compare player's move against computer's move
                let does_beat: bool = Move::from_str(&pmove_pretty)
                    .unwrap()
                    .beats(computer_move.unwrap());

                match does_beat {
                    true => {
                        println!(
                            "\n{} Arghh...I picked {}. You won!",
                            game_tag,
                            &computer_move.unwrap()
                        );
                        break;
                    }
                    false => {
                        alter_introduction!(introduction, game_tag);

                        if pmove_pretty == computer_move.unwrap().to_string() {
                            println!(
                                "\n{} Looks like we picked the same move, let's try again!",
                                game_tag
                            );
                            computer_move = moves.choose(&mut rand::thread_rng());
                            continue;
                        } else {
                            println!(
                                "\n{} Ha! I picked {}. You lost!",
                                game_tag,
                                &computer_move.unwrap()
                            );
                            break;
                        }
                    }
                }
            }
            false => {
                println!("\n{} Hey, that's an invalid guess!", game_tag);
                alter_introduction!(introduction, game_tag);

                continue;
            }
        }
    }

    println!("\n{} Thank you for playing!", game_tag);
}

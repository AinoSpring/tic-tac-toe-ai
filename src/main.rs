mod ttt;

use ttt::*;
use std::io;

fn ttt_to_string(x: i8) -> String {
    String::from(match x {
        1 => "X",
        -1 => "O",
        _ => " ",
    })
}

fn input_coord() -> usize {
    loop {
        let mut i = String::new();
        io::stdin()
            .read_line(&mut i)
            .unwrap();
        let i = i.trim()
            .parse::<usize>();
        match i {
            Ok(x) => {
                if x < 3 {
                    break x;
                } else {
                    println!("Number has to be smaller than 3!");
                }
            },
            Err(_) => {
                println!("Not a valid number!");
            },
        };
    }
}


fn main() {
    let mut ttt = TTT::new();
    ttt.player = -1;
    ttt.apply_move(ttt.best_move(9));
    while ttt.evaluate() == 0 && !ttt.board_full() {
        println!("Deep evaluation: {}", ttt.deep_evaluate(9));
        for y in 0..3 {
            for x in 0..3 {
                print!("{}", ttt_to_string(ttt.board[x][y]));
            }
            println!();
        } 
        let m = if ttt.player == 1 {
            loop {
                let x = input_coord();
                let y = input_coord();
                if ttt.board[x][y] == 0 {
                    break (x, y);
                } else {
                    println!("Tile not empty!");
                }
            }
        } else {
            ttt.best_move(9)
        };

        ttt.apply_move(m);

        println!();
    }
    for y in 0..3 {
        for x in 0..3 {
            print!("{}", ttt_to_string(ttt.board[x][y]));
        }
        println!();
    }
    println!("{} won!", ttt_to_string(ttt.evaluate()));
}

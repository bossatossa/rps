mod element;

use rand::Rng;
use std::{io, process::exit};

use crate::element::Element;

fn print_game_result(player_int: u8, cpu_int: u8, result: bool) {
    if player_int == cpu_int {
        println!("It's a draw!")
    } else {
        if player_int == 0 {
            println!("Player used Rock");
        } else if player_int == 1 {
            println!("Player used Paper");
        } else if player_int == 2 {
            println!("Player used Scissors");
        }

        if cpu_int == 0 {
            println!("CPU used Rock");
        } else if cpu_int == 1 {
            println!("CPU used Paper");
        } else if cpu_int == 2 {
            println!("CPU used Scissors");
        }

        if result {
            println!("Player won, CPU lost")
        } else {
            println!("CPU won, Player lost")
        }
    }
}

fn main() {
    println!("Rock, Paper, Scissors!");
    println!("What's your hand? (R)ock, (P)aper or (S)cissors?");
    let result: bool;

    let rock: Element = Element {
        id: 0,
        wins_against: 2,
        loses_against: 1,
    };
    let paper = Element {
        id: 1,
        wins_against: 0,
        loses_against: 2,
    };
    let scissors = Element {
        id: 2,
        wins_against: 1,
        loses_against: 0,
    };

    loop {
        let cpu_hand = rand::thread_rng().gen_range(0..=2);
        let mut player_hand = String::new();
        println!("CPU hand: {}", cpu_hand);

        io::stdin()
            .read_line(&mut player_hand)
            .expect("Failed to read line");

        if player_hand.trim() == "R" {
            result = rock.game_result(cpu_hand);
            print_game_result(rock.id, cpu_hand, result);
            exit(0);
        } else if player_hand.trim() == "P" {
            result = paper.game_result(cpu_hand);
            print_game_result(paper.id, cpu_hand, result);
            exit(0);
        } else if player_hand.trim() == "S" {
            result = scissors.game_result(cpu_hand);
            print_game_result(scissors.id, cpu_hand, result);
            exit(0);
        } else {
            println!("Please choose (R)ock, (P)aper or (S)cissors")
        }
    }
}

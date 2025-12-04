extern crate rand;
use std::{io, vec};
use rand::{random, Rng};

fn get_player_move(player_num : i32) -> String {
    loop {
        let mut input = String::new();

        println!("player {}'s turn.", player_num);
        io::stdin().read_line(&mut input).expect("could not read input");
        let move_cleaned = input.trim().to_lowercase();

        if move_cleaned == "rock" || move_cleaned == "paper" || move_cleaned == "scissor" {
            return move_cleaned;

        } else {
            println!("Invalid move from Player {}. Try again.", player_num);
        }


    }
}

fn get_bot_move() -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..3);

    let moves = ["rock", "paper", "scissor"];
    moves[random_index].to_string()
}

fn play_with_friend() {
    let player1_move = get_player_move(1);
    let player2_move = get_player_move(2);

    if player1_move == player2_move {
        println!("it is a draw");
    } else if (player1_move == "rock" && player2_move == "scissor")
        || (player1_move == "paper" && player2_move == "rock")
            || (player1_move == "scissor" && player2_move == "paper")
    {
        println!("Player 1 won!");
    } else {
        println!("Player 2 won!");
    }
}

fn play_with_bot() {
    let player_move = get_player_move(1);
    let bot_move = get_bot_move(); 


    println!("Bot chose: {}", bot_move);
    if player_move == bot_move {
        println!("It is a draw");
    } else if (player_move == "rock" && bot_move == "scissor")
        || (player_move == "paper" &&  bot_move == "rock")
            || (player_move == "scissor" && bot_move == "paper")
    {
        println!("Player won!");
    } else {
        println!("Bot won");

    }
}

fn main() {
    let mut game_choose : String = String::new();
    println!("Write your choice \n1. Play with friend\t 2. Play with bot");
    io::stdin() 
        .read_line(&mut game_choose)
        .expect("couldn't read input");
    if game_choose.trim() == "1" {
        play_with_friend();
    } else {
        play_with_bot();
    }
}

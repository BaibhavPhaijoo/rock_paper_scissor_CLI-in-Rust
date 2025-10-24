use std::{io, string};

fn get_player_move(player_num : i32) -> String {
    loop {
        let mut input = String::new();

        println!("Player {}'s turn.", player_num);
        io::stdin().read_line(&mut input).expect("Could not read input");
        let move_cleaned = input.trim().to_lowercase();

        if move_cleaned == "rock" || move_cleaned == "paper" || move_cleaned == "scissor" {
            return move_cleaned;

        } else {
            println!("Invalid move from Player {}. Try again.", player_num);
        }


    }
}

fn play_with_friend() {
    let player1_move = get_player_move(1);
    let player2_move = get_player_move(2);

    if player1_move == player2_move {
        println!("It is a draw");
    } else if (player1_move == "rock" && player2_move == "scissor")
        || (player1_move == "paper" && player2_move == "rock")
            || (player1_move == "scissor" && player2_move == "paper")
    {
        println!("Player 1 won!");
    } else {
        println!("Player 2 won!");
    }
}

fn main() {
    play_with_friend();
}

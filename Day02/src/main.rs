use std::fs;

fn main() {
    let file_path = "G:\\003_PROGRAMOZAS\\aoc2023\\Day02\\src\\input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let parts = contents.split("\r\n");
    let balls_available = vec![12, 13, 14];
    let mut sum = 0;


    for line in parts {
        let games: Vec<_> = line.split("Game ").collect();
        let games: Vec<_> = games[1].split(": ").collect();
        let game_id_int = (games[0]).parse::<i32>().unwrap();

        let games: Vec<_> = games[1].split("; ").collect();
        let mut possible = true;
        for game in games {
            let mut ball_counts = vec![0, 0, 0];
            let balls: Vec<_> = game.split(", ").collect();

            for ball in balls {
                let ball: Vec<_> = ball.split(" ").collect();
                let ball_count = ball[0].parse::<i32>().unwrap();
                let ball_color = ball[1];
                if ball_color == "red" {
                    ball_counts[0] += ball_count;
                } else if ball_color == "green" {
                    ball_counts[1] += ball_count;
                } else if ball_color == "blue" {
                    ball_counts[2] += ball_count;
                }
            }
            if (ball_counts[0] > balls_available[0]) || (ball_counts[1] > balls_available[1]) || (ball_counts[2] > balls_available[2]) {
                println!("Game {} is not possible", game_id_int);
                possible = false;
                break;
            }
        }
        if possible {
            println!("Game {} is possible", game_id_int);
            sum += game_id_int;
        }
    }
    println!("Sum: {}", sum);
}

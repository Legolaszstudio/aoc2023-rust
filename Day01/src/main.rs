// My first ever rust program 🎉
use std::fs;
use std::collections::VecDeque;

fn main() {
    let file_path = "G:\\003_PROGRAMOZAS\\aoc2023\\Day01\\src\\input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let parts = contents.split("\r\n");
    let mut sum = 0;

    for part in parts {
        let chars = part.split("");
        let mut deque: VecDeque<&str> = VecDeque::new();
        for chara in chars {
            if chara.len() == 0 { continue; }
            let is_numeric = chara.chars().next().unwrap().is_ascii_digit();
            if is_numeric {
                deque.push_back(chara);
            }
        }
        if (deque.len() == 0) { continue; } //For testing, I added this, shouldn't hurt
        let lineStr = deque[0].to_owned() + deque[deque.len() - 1];
        sum += lineStr.parse::<i32>().unwrap();
    }

    println!("Sum: {}", sum);
    partTwo(contents.split("\r\n"));
}


fn partTwo(lines: std::str::Split<'_, &str>) {
    let mut sum = 0;
    for line in lines {
        let mut matches: Vec<_> = line.match_indices("zero").collect();
        matches.extend(line.match_indices("0"));
        matches.extend(line.match_indices("one"));
        matches.extend(line.match_indices("1"));
        matches.extend(line.match_indices("two"));
        matches.extend(line.match_indices("2"));
        matches.extend(line.match_indices("three"));
        matches.extend(line.match_indices("3"));
        matches.extend(line.match_indices("four"));
        matches.extend(line.match_indices("4"));
        matches.extend(line.match_indices("five"));
        matches.extend(line.match_indices("5"));
        matches.extend(line.match_indices("six"));
        matches.extend(line.match_indices("6"));
        matches.extend(line.match_indices("seven"));
        matches.extend(line.match_indices("7"));
        matches.extend(line.match_indices("eight"));
        matches.extend(line.match_indices("8"));
        matches.extend(line.match_indices("nine"));
        matches.extend(line.match_indices("9"));
        matches.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        
        let mut result = matches.get(0).unwrap().1.to_owned() + matches.get(matches.len() - 1).unwrap().1;
        result = result.replace("zero", "0");
        result = result.replace("one", "1");
        result = result.replace("two", "2");
        result = result.replace("three", "3");
        result = result.replace("four", "4");
        result = result.replace("five", "5");
        result = result.replace("six", "6");
        result = result.replace("seven", "7");
        result = result.replace("eight", "8");
        result = result.replace("nine", "9");

        sum += result.parse::<i32>().unwrap();
    }
    println!("Sum2: {}", sum);
}
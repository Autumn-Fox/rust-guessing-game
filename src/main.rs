fn main() {
    println!("Think of a number between 1 and 100");
    let answer = recursive_guess(1, 100);
    println! ("Your answer is {}.", answer);
}

fn recursive_guess(min:i32, max:i32) -> i32 {
    let difference = max - min;
    let guess = get_input(min, max);
    if difference <= 1 {

        if guess == "Y".to_string() {
            return min;
        } else if guess == "E".to_string() {
            return max;
        } else {
            return max;
        }

    } else if difference == 0 {
        return min;
    } else {
        if guess == "Y".to_string() {
            return recursive_guess(min, max - (difference + 1)/2);
        } else {
            return recursive_guess(min + difference/2 + 1, max);
        }
    }
    
}


fn get_input(m_bound: i32, max_bound: i32) -> String {
    return loop {
        if max_bound == m_bound {
            break "E".to_string();
        }

        if max_bound - m_bound > 1 {
            println!("Is your number between {} and {}? (Y/N)", m_bound, m_bound + (max_bound - m_bound)/2)
        } else {
            println!("Is your number {}? (Y/N)", m_bound)
        }

        let mut player_guess = String::new();
        std::io::stdin().read_line(&mut player_guess)
            .expect("Input error");

        let player_guess: String = player_guess.trim().to_string();
        
        if player_guess == "Y".to_string() || player_guess == "N".to_string() {
            break player_guess;
        } else {
            continue;
        }
    }
}
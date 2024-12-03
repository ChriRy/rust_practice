use std::io;

fn main() {
    const SECRET_WORD: &str = "polar"; // The word the user will be guessing
    const WORD_COUNT: usize = SECRET_WORD.len(); // The number of letters in the secret word
    let mut word_array: [char; WORD_COUNT] = ['_'; WORD_COUNT]; // An array to hold each letter in the secret word

    // Code to take each letter in the secret word and add it to the word_array
    let mut index = 0;
    for c in SECRET_WORD.chars() {
        word_array[index] = c;
        index += 1;
    }

    // Create an empty array that will hold the entries of the user.
    let mut guess_array: [char; SECRET_WORD.len()] = ['_'; SECRET_WORD.len()];

    let gameloop = true;
    let mut correct_guesses = 0;
    let mut guess_list: Vec<char> = Vec::new();

    while gameloop == true {
        clear_screen();

        println!("------------------------------------------------------------------------------");
        println!("\n");

        for entry in guess_array {
            // For loop to print out the empty spaces representing the word.
            print!("{entry}  ");
        }

        println!("\n");
        println!("------------------------------------------------------------------------------");
        println!("Guess Count: {}", guess_list.len());
        println!(">> ");

        if word_array == guess_array {
            println!("You win!");
            break;
        }

        let (guess, guess_list) = get_user_input(&mut guess_list); // Call the get_user_input function so the user can input their guess
        guess_list.push(guess);

        for i in 0..WORD_COUNT {
            // For loop to run through word_array and see if the user's guess matches
            if word_array[i] == guess {
                // any of the letters. If it does, it changes guess_array at that same index.
                guess_array[i] = guess;
                correct_guesses += 1;
            }
        }

        clear_screen();
    }
}

fn get_user_input(guess_list: &mut Vec<char>) -> (char, &mut Vec<char>) {
    loop {
        let mut guess = String::new();
        // Read the input from stdin
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim whitespace and get the first character
        let guess = guess.trim().chars().next();

        match guess {
            Some(first_char) if guess_list.contains(&first_char) => {
                println!("Already guessed, please enter a different letter.");
            }
            Some(first_char) => {
                return (first_char, guess_list);
            }
            None => {
                println!("No character entered. Please try again.");
            }
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

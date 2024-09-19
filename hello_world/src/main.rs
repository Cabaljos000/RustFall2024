fn check_guess(guess: i32, secret:i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main(){
    let secret_number = 61;

    let mut attempts = 0;

    let mut guess = 56;

    loop{
        attempts += 1;

        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("You guessed it! The secret number is {}", secret_number);
            break;
        } else if result == 1 {
            println!("Your guess of {} is too high!", guess);
            guess -= 1;
        } else {
            println!("Your guess of {} is too low!", guess);
            guess += 1;
        }        
    }

    println!("You found the number in {} attempts.", attempts);
}

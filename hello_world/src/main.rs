fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let nums = [10, 15, 23, 30, 42, 5, 60, 75, 12, 9];

    for &num in nums.iter(){
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }

        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    let mut sum = 0;
    let mut _i = 0;
    
    while _i < nums.len() {
        sum += nums[_i];
        _i += 1;
    }
    println!("The sum of all numbers is: {}", sum);

    let mut largest = nums[0];
    for &num in nums.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is: {}", largest);
}

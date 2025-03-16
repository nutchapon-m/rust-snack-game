use std::process::Command;
use std::thread;
use std::time::Duration;
use std::io::{self};
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (x, y) = (4, 4);
    let size = x * y;

    let mut rng = rand::rng();
    let mut k_x = rng.random_range(1..size - 1);
    let mut k_y = rng.random_range(1..size - 1);

    let mut input = String::new();

    // let mut count = 0;
    while input != "exit" {
        display_board(&mut k_x, &mut k_y, size);
        println!("Enter command: ");
        input_command(&mut input)?;
        change_direction(&mut k_x, &mut k_y, &mut input)?;
        thread::sleep(Duration::from_millis(50));
        input.clear();
        clear_screen()?;
    }
    println!("Good bye!!");
    Ok(())
}

fn input_command(input: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    io::stdin().read_line(input)?;
    Ok(())
}
 
fn clear_screen() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("clear").status()?;
    Ok(())
}

fn change_direction(d_x: &mut i32, d_y: &mut i32, input: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    let key_set = ["a", "w", "d", "s"];
    let k  = input.trim();
    if !key_set.contains(&k) {
        return Err(Box::from("Invalid input"));
    }

    if k == "d" {
        *d_x += 0;
        *d_y += 1;
    } else if k == "w" {
        *d_x += -1;
        *d_y += 0;
    } else if  k == "s" {
        *d_x += 1;
        *d_y += 0;
    } else {
        *d_x += 0;
        *d_y += -1;
    }

    Ok(())
}

fn display_board(k_x: &mut i32, k_y: &mut i32, size: i32) {
    for i in 0..size {
        for j in 0..size {
            if i == 0 || j == 0 || i == size - 1 || j == size - 1 {
                print!("*")
            } else if i == *k_x && j == *k_y {
                print!("o")
            } else {
                print!(" ")
            }
        }
        println!()
    }
}

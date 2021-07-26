use std::error::Error;
use std::io;
use std::str::FromStr;

// GOALS
// Setup three vector "stacks"
// Build a loop that accepts input from the user to move discs from one to another
// Logic to limit putting large discs on smaller discs
// Logic to limit the "winning scenario"

fn main() {
    // THREE STACKS
    let mut stack_1: Vec<i32> = vec![3, 2, 1];
    let mut stack_2: Vec<i32> = vec![];
    let mut stack_3: Vec<i32> = vec![];

    let vec_refs: [&mut Vec<i32>; 3] = [&mut stack_1, &mut stack_2, &mut stack_3];

    // INSTRUCTIONS
    println!("Goal: move all discs (numbers) from the 1st rod to the 3rd. You cannot put a larger number on a smaller number.");
    println!("Move a disc from one stack to another by typing [origin_stack] [destination_stack] example: 1 3");

    fn set_target(
        target: &mut i32,
        vec_refs: &[&mut Vec<i32>; 3],
        is_dest: bool,
        origin: i32,
    ) -> bool {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        match buf.trim_end().parse::<i32>() {
            Ok(input) => {
                if (input as usize) > vec_refs.len() || input == 0 {
                    println!("");
                    println!("***out of bounds***");
                    println!("");
                    return false;
                } else {
                    // CHECK IF THERE IS A LAST ELEMENT AND IF IT IS SMALLER
                    if is_dest {
                        if input == origin {
                            println!("");
                            println!("***distination is the same as origin***");
                            println!("");
                            return false;
                        }
                        let dest_vec = &vec_refs[(input as usize) - 1];
                        match dest_vec.last() {
                            Some(dest_disc) => {
                                if let Some(ori_disc) = &vec_refs[(origin as usize) - 1].last() {
                                    if dest_disc < ori_disc {
                                        println!("");
                                        println!(
                                            "***cannot place larger disc ({}) on smaller disc ({})***",
                                            input, dest_disc
                                        );
                                        println!("");
                                        return false;
                                    }
                                }
                            }
                            None => (),
                        }
                    }

                    *target = input;
                    true
                }
            }
            Err(_) => {
                println!("");
                println!("***incorrect input***");
                println!("");
                false
            }
        }
    }

    // GAME LOOP
    loop {
        // INPUT LOGIC
        let mut origin: i32 = 0;
        let mut destination: i32 = 0;

        // STACK INFO
        println!("current stacks");
        println!("1: {:?}", vec_refs[0]);
        println!("2: {:?}", vec_refs[1]);
        println!("3: {:?}", vec_refs[2]);
        println!("");

        // CHOOSE ORIGIN AND DESTINATION
        println!("move top item from...?");
        if !set_target(&mut origin, &vec_refs, false, 0) {
            continue;
        }
        println!("to the of... where...?");
        if !set_target(&mut &mut destination, &vec_refs, true, origin.clone()) {
            continue;
        }

        // POP AND PLACE
        let popped = &vec_refs[(origin - 1) as usize].pop().unwrap();
        vec_refs[(destination - 1) as usize].push(popped.clone());

        let final_vec: Vec<i32> = vec![3, 2, 1];
        if *vec_refs[2] == final_vec {
            println!("");
            println!("You did it!");
            println!("");
            break;
        }
    }
}

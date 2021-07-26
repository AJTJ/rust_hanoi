use std::io;

fn main() {
    // THREE "STACKS"
    let mut vec_1: Vec<i32> = vec![3, 2, 1];
    let mut vec_2: Vec<i32> = vec![];
    let mut vec_3: Vec<i32> = vec![];

    let vec_refs: [&mut Vec<i32>; 3] = [&mut vec_1, &mut vec_2, &mut vec_3];

    // INSTRUCTIONS
    println!("In the spirit of the tower of Hanoi, move all numbers (discs) from the 1st set (rod) to the 3rd. You can only move one number at a time and you are only able to move the top (right-most) number from any given set. You cannot put a larger number on a smaller number");

    fn set_target(target: &mut i32, vec_refs: &[&mut Vec<i32>; 3], origin: i32) -> bool {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        match buf.trim_end().parse::<i32>() {
            // input exists
            Ok(input) => {
                // check if input is out of bounds
                if (input as usize) > vec_refs.len() || input <= 0 {
                    println!("");
                    println!("***out of bounds***");
                    println!("");
                    return false;
                } else {
                    // origin will be greater than 0 while setting the destination
                    if origin > 0 {
                        // check if distination is also the origin
                        if input == origin {
                            println!("");
                            println!("***distination is the same as origin***");
                            println!("");
                            return false;
                        }
                        let dest_vec = &vec_refs[(input as usize) - 1];
                        match dest_vec.last() {
                            Some(dest_disc) => {
                                // check if the moved number is smaller than than any numbers it is landing on
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

        // VECTOR INFO
        println!("");
        println!("1: {:?}", vec_refs[0]);
        println!("2: {:?}", vec_refs[1]);
        println!("3: {:?}", vec_refs[2]);
        println!("");

        // CHOOSE ORIGIN AND DESTINATION
        println!("move top item from...?");
        if !set_target(&mut origin, &vec_refs, 0) {
            continue;
        }
        println!("to the of... where...?");
        if !set_target(&mut &mut destination, &vec_refs, origin.clone()) {
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

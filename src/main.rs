use std::io;

fn main() {
    // THREE "STACKS"
    let mut vec_1: Vec<usize> = vec![3, 2, 1];
    let mut vec_2: Vec<usize> = vec![];
    let mut vec_3: Vec<usize> = vec![];

    let vec_refs: [&mut Vec<usize>; 3] = [&mut vec_1, &mut vec_2, &mut vec_3];

    // INSTRUCTIONS
    println!();
    println!("In the spirit of the tower of Hanoi, move all numbers (discs) from the 1st set (rod) to the 3rd. You can only move one number at a time and you are only able to move the top (right-most) number from any given set. You cannot put a larger number on a smaller number");

    fn set_target(target: &mut usize, vec_refs: &[&mut Vec<usize>; 3], origin: usize) -> bool {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        match buf.trim_end().parse::<usize>() {
            // input is correct
            Ok(input) => {
                //check if input is in bounds
                if let None = input.checked_sub(1) {
                    println!();
                    println!("***out of bounds***");
                    println!();
                    return false;
                }
                match vec_refs.get(input - 1) {
                    Some(_) => {
                        if origin > 0 {
                            // origin will be greater than 0 while setting the destination
                            return check_destination(target, vec_refs, origin, input);
                        }
                        *target = input;
                        true
                    }
                    None => {
                        // Input out of bounds
                        println!();
                        println!("***out of bounds***");
                        println!();
                        return false;
                    }
                }
            }
            // incorrect input
            Err(_) => {
                println!();
                println!("***incorrect input***");
                println!();
                false
            }
        }
    }

    fn check_destination(
        target: &mut usize,
        vec_refs: &[&mut Vec<usize>; 3],
        origin: usize,
        input: usize,
    ) -> bool {
        // check if distination is also the origin
        if input == origin {
            println!();
            println!("***destination is the same as origin***");
            println!();
            return false;
        }
        let dest_vec = &vec_refs[input - 1];
        if let Some(dest_disc) = dest_vec.last() {
            // check if the moved number is smaller than than any numbers it is landing on
            if let Some(ori_disc) = &vec_refs[origin - 1].last() {
                if dest_disc < ori_disc {
                    println!();
                    println!(
                        "***cannot place larger disc ({}) on smaller disc ({})***",
                        input, dest_disc
                    );
                    println!();
                    return false;
                }
            }
        }
        *target = input;
        true
    }

    fn display_vectors(vec_refs: &[&mut Vec<usize>; 3]) {
        println!();
        println!("1: {:?}", vec_refs[0]);
        println!("2: {:?}", vec_refs[1]);
        println!("3: {:?}", vec_refs[2]);
        println!();
    }

    // GAME LOOP
    loop {
        // INPUT LOGIC
        let mut origin: usize = 0;
        let mut destination: usize = 0;

        display_vectors(&vec_refs);

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
        let popped = &vec_refs[origin - 1].pop().unwrap();
        vec_refs[destination - 1].push(popped.clone());

        let final_vec: Vec<usize> = vec![3, 2, 1];
        if *vec_refs[2] == final_vec {
            display_vectors(&vec_refs);
            println!();
            println!("You did it!");
            println!();
            break;
        }
    }
}

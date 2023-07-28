fn pattern2(size:i32){

    for _i in (1..size).rev() {
    
        for _j in 1..=_i {
            print!(" ") 
        }
        for _k in _i..size {
            print!("*")
        } println!()
    }
    for _i in 0..size {
        for _j in 0.._i {
            print!(" ")
        }
        for _k in _i..size {
            print!("*")
        } println!()
    }
    
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size_arg = if args.len() < 2 {""} else {&args[1]};
    let size: i32 = size_arg.parse().unwrap_or(0);
    
    pattern2(size);
}
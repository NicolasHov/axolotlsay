use clap::Parser;
enum UserEmotion {
    Happy,
    Sad,
}


impl ToString for UserEmotion {
    fn to_string(&self) -> String {
        match self {
            UserEmotion::Happy => String::from("User is logged in."),
            UserEmotion::Sad => String::from("User is logged out."),
            _ => String::from("User is banned"),
        }
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    message: String,

    /// State of mind
     emotion: UserEmotion;
    
}

fn main() {
    let args = Args::parse();
    //TODO: match on emotion
    // dbg!(args); // display with extended info
    println!("{}", UserEmotion::Happy);
    println!("                       ");
    println!("       | {:?} !|", args.message);
    println!("        / ");
    println!("       / ");
    println!("      / ");
    println!("  ___");
    println!(" >O_O<");
    println!(" (   )");
    println!(":(  ):");
    println!(" ( )");
    println!(" ( )");
    println!(" / |");
}

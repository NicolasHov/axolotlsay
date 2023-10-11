use clap::Parser;
enum UserEmotion {
    Happy,
    Sad,
}

impl ToString for UserEmotion {
    fn to_string(&self) -> String {
        match self {
            UserEmotion::Happy => String::from("I'm happy"),
            UserEmotion::Sad => String::from("The world is ending, yay!"),
            _ => String::from("I'm dead"),
        }
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    message: String,
}

fn main() {
    let args = Args::parse();
    //TODO: match on emotion
    let emotion = UserEmotion::Sad;
    let emotion_str = emotion.to_string();
    // dbg!(args); // display with extended info
    println!("                       ");
    println!("       | I'm {:?} and {} !|", args.message, emotion_str);
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

use std::{io, thread, time};

fn main() {
    let mut name = String::new();
    print!("{esc}[2J{esc}[1:1H", esc = 27 as char);

    println!(".");
    sleep_seconds(1);
    println!("..");
    sleep_seconds(2);
    println!("A wormhole and a GROM falls out.");
    println!("'Who are you?', the GROM says.");
    sleep_seconds(1);
    println!("You say to the GROM:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read user input.");

    let name = name.trim();

    if name == "" {
        sleep_seconds(1);
        println!("The GROM says 'Fine. Be that way.' and paddles out in to the metaverse.");
        sleep_seconds(2);
        println!("You spin in the GROM's wake and drift out into deepspace.");
        sleep_seconds(1);
        println!("end program.");
    } else {
        sleep_seconds(2);
        println!(
            "The GROM says 'Hello {}. Let's ride!' and tossed you a tow rope.",
            name
        );
        sleep_seconds(1);
        println!("The moment you touch the rope, time and space folds around you.");
        sleep_seconds(1);
        println!(".");
        sleep_seconds(1);
        println!("..");
        sleep_seconds(1);
        println!("Welcome to the metaverse.");
    }
    sleep_seconds(1);
}

fn sleep_seconds(secs: u64) {
    thread::sleep(time::Duration::from_secs(secs));
}

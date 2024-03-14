use std::{
    io,
    time::{Duration, Instant},
};

struct FishingGame {
    st: Instant,
    c_fish: u32,
}

impl FishingGame {
    fn count_fish(&mut self) -> bool {
        if self.c_fish >= 100 {
            print!("GG! ");
            return true;
        }

        println!(
            "You have collected {}, only {} to go!",
            self.c_fish,
            100 - self.c_fish
        );
        false
    }

    fn play(&mut self) -> bool {
        let mut complete: bool = false;
        while !complete {
            self.c_fish += get_fish_count();
            complete = self.count_fish();
        }

        true
    }
}

fn get_fish_count() -> u32 {
    let mut input: String = String::new();

    println!("How much fish have you collected?");
    io::stdin()
        .read_line(&mut input)
        .expect("failed reading stdin");
    let new_input = input.trim();

    let number: u32 = new_input.parse::<u32>().expect("couldn't parse the number");

    number
}

fn format_completion_time(t: Duration) -> String {
    let minutes = t.as_secs() / 60;
    let seconds = t.as_secs() % 60;

    let result: String = format!("{:?}m {:?}s", minutes, seconds);

    result
}

fn main() {
    let mut game = FishingGame {
        st: Instant::now(),
        c_fish: 0,
    };

    let mut complete = false;
    while !complete {
        complete = game.play();
    }

    let et: Duration = game.st.elapsed();
    println!("That took {}!", format_completion_time(et))
}

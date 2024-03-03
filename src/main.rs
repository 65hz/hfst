/*
    PLAN:
    one struct called FishingGame containing:
      st: Duration - the starting time, so i can then use it to get an elapsed time later on
      c_fish: i32 - collected fish, keeps track of how much fish i have
    
    methods of FishingGame:
      count_fish(k: u32) - completely unknown action
      play() - no fucking clue

    input will be its own function without any methods.
*/

use std::{io, time::{Duration, Instant}};

struct FishingGame {
    st: Instant,
    c_fish: i32,
}

impl FishingGame {
    fn count_fish(&mut self) -> bool {
        if self.c_fish >= 100 {
            print!("GG! ");
            return true
        }
        
        println!("You have collected {}, only {} to go!", self.c_fish, 100 - self.c_fish);
        false
    }
    
    fn play(&mut self) -> bool {
        let mut complete: bool = false;
        while complete == false {
            self.c_fish = self.c_fish + get_fish_count();
            complete = self.count_fish();
        };

        return true;
    }
}

fn get_fish_count() -> i32 {
    let mut input = String::new();
    
    println!("How much fish have you collected?");
    io::stdin().read_line(&mut input).expect("failed reading stdin");
    let new_input = input.trim();

    let number: i32 = new_input.parse::<i32>().expect("couldn't parse the number");

    if number < 0 {
        panic!("negative number spotted")
    }

    number
}

fn format_completion_time(t: Duration) -> String {
    let _minutes = t.as_secs() / 60;
    let _seconds = t.as_secs() % 60;

    let _result: String = format!("{:?}m {:?}s", _minutes, _seconds);

    _result
}

fn main() {
    let mut game = FishingGame {
        st: Instant::now(),
        c_fish: 0,
    };

    let mut complete = false;
    while complete == false {
        complete = game.play();
    }

    let et: Duration = game.st.elapsed();
    println!("That took {}!", format_completion_time(et))
}
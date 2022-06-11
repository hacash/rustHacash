use hacash::{cores::actions::*};
use hacash::interface::{ Field, Action};

/*
    cargo test --test action -- --nocapture
*/

#[test]
fn actions() {

    fn print_field(act: &dyn Field) {
        println!("print_field: {}", act.describe());
    }

    fn print_action(act: &dyn Action) {
        println!("print_action: {}", act.describe());
    }

    let act1 = Action1HacTrs::new();

    print_field(&act1);
    print_action(&act1);


}


use hacash::{cores::actions::*};
use hacash::interface::{ Field, Action};
use hacash::chain::state::*;

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

        let mut optr = StateOperate::new();
        _ = act.write_in_chain_state(&mut optr);
    }

    let act1 = Action1HacTrs::new();

    let mut optr = StateOperate::new();
    _ = act1.write_in_chain_state(&mut optr);

    print_field(&act1);
    print_action(&act1);



}

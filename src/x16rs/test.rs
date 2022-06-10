include!("./mod.rs");



fn main() {

        // input hash
        let input_data = [0u8; 32];

        // do hash
        let output_hash = x16rs_hash_wrap(1, &input_data);

        println!("{:?}", output_hash);

    
}

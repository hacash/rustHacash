

pub trait Action : Field {

    fn kind(&self) -> u16;
    fn request_sign_addresses(&self) -> Vec<Address> {
        vec![]
    }
    fn is_burning_90_persent_tx_fee(&self) -> bool {
        false
    }

}


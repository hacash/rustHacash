
// start with the 20001st diamond and enable the 32-bit MSG byte
pub const DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE: DiamondNumber = DiamondNumber::from(2_0000);

// define special field
pub_type_prefix_value_check_impl_Field_trait!(
    DiamondCreateExtendMsg, 
    number, DiamondNumber, Fixedbytes32,
    3+32+8+21, { number.get_value() as u64 > DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE.value() as u64 }
);


// create struct Action4DiamondCreate
action_create_struct_for_common_items!(
    ACTION_KIND_4, Action4DiamondCreate,

	diamond              , DiamondName                 ,   // diamond literal wtyuiahxvmekbszn
	number               , DiamondNumber               ,   // diamond serial number for difficulty check
	prev_hash            , Hash                        ,   // previous block hash containing diamond
	nonce                , Fixedbytes8                      ,   // random number
	address              , Address                     ,   // account
	// customer message                                   
	custom_message       , DiamondCreateExtendMsg<DiamondNumber, Fixedbytes32>      ,

);


impl_Action_trait_for_common_single!( Action4DiamondCreate, self, state, trs, {
   


    
    Ok(())
});


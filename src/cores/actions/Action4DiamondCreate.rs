
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


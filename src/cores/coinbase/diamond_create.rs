
// start with the 20001st diamond and enable the 32-bit MSG byte
pub const DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE: u32 = 2_0000;

// Starting from the 30001st diamond, destroy 90% of the bidding cost
pub const DIAMOND_ABOVE_NUMBER_OF_BURNING90_PERCENT_TX_FEES: u32 = 3_0000;

// The average bidding cost of 30001 ~ 40000 diamonds is adopted, and the previous setting is 10 diamonds
pub const DIAMOND_ABOVE_NUMBER_OF_STATISTICS_AVERAGE_BIDDING_BURNING: u32 = 4_0000;

// 40001 diamond, start with Sha3_ Hash (diamondreshash + blockhash) determines diamond shape and color matching
pub const DIAMOND_ABOVE_NUMBER_OF_VISUAL_GENE_APPEND_BLOCK_HASH: u32 = 4_0000;

// 41001 diamond, start with Sha3_ Hash (diamondreshash + blockhash + bidfee) includes the bidding fee to participate in the decision of diamond shape color matching
pub const DIAMOND_ABOVE_NUMBER_OF_VISUAL_GENE_APPEND_BIDDING_FEE: u32 = 4_1000;


// calculate diamond visual gene
pub fn calculate_diamond_visual_gene(number: &DiamondNumber, diamhash: &[u8;32], diamondstr: &[u8;16], pedding_block_hash: &Hash, diabidfee: &Amount) -> DiamondVisualGene {
    
    const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
    
    // start
    let dianum = number.value();
    let mut genehexstr = [0u8; 20];
    // cacl vgenehash
    let mut vgenehash = diamhash.clone();
    if dianum > DIAMOND_ABOVE_NUMBER_OF_VISUAL_GENE_APPEND_BLOCK_HASH {
        let mut vgenestuff = diamhash.to_vec();
        vgenestuff.append( &mut pedding_block_hash.to_vec() ); // add block hash
        if dianum > DIAMOND_ABOVE_NUMBER_OF_VISUAL_GENE_APPEND_BIDDING_FEE {
            vgenestuff.append( &mut diabidfee.serialize() ); // add bidfee
        }
        vgenehash = x16rs::calculate_hash(vgenestuff);
    }

    let mut idx = 2usize; // from index 2
    // step 1
    let searchgx = |x| {
        for (i, a) in DIAMOND_NAME_VALID_CHARS.iter().enumerate() {
            if *a == x {
                return HEX_CHARS[i]
            }
        }
        panic!("not supply diamond char!!!")
    };
    for i in 10..16 {
        genehexstr[idx] = searchgx( diamondstr[i] );
        idx += 1;
    }
    // step 2
    for i in 20..31 {
        let k = (vgenehash[i] as usize) % 16;
        genehexstr[idx] = HEX_CHARS[k];
        idx += 1;
    }
    // last bit of hash as shape selection
    let mut genehex = hex::decode(genehexstr).unwrap();
    genehex[0] = vgenehash[31];
    DiamondVisualGene::from(genehex.try_into().unwrap())
}


pub fn calculate_diamond_average_bid_burn(number: &DiamondNumber, total_bid_burn: f64) -> Uint2 {


    let diamond_number = number.value();
    // old
    if diamond_number <= DIAMOND_ABOVE_NUMBER_OF_STATISTICS_AVERAGE_BIDDING_BURNING {
        return Uint2::from(10)
    }

    // average
    let bsnum = diamond_number - DIAMOND_ABOVE_NUMBER_OF_BURNING90_PERCENT_TX_FEES;
    let bidfee = total_bid_burn / (bsnum as f64) + 0.9999_9999;
    let mut bidfee = bidfee as u16;
    if bidfee < 1 {
        bidfee = 1; // min is 1
    }
    // ok
    Uint2::from(bidfee)
}


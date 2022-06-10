use sha3::{Digest, Sha3_256};
use hacash::x16rs;


/*
    cargo test --test sha3 -- --nocapture
*/

#[test]
fn sha3_test_29347592347() {

    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();
    // write input message
    hasher.update(b"abc");
    // read hash digest
    let result = hasher.finalize();
    assert_eq!(result[..], hex::decode("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532").unwrap()[..]);


    /***********************************8 */


    // test 100

    let mut reshxs: Vec<String> = Vec::with_capacity(100);

    reshxs.push("9e6291970cb44dd94008c79bcaf9d86f18b4b49ba5b2a04781db7199ed3b9e4e".to_string());
    reshxs.push("3ffcf92d9c820def681c81ab1dffa44c3166539addb445c7731921af69bce8c7".to_string());
    reshxs.push("eb61fef1a47919434cfbddf1f4d96d492d6469462d636105c7c9c1fcd022da2c".to_string());
    reshxs.push("96b70dcd8b1ea711c3ab9e1bf6042066f41f2c41237f48dbf37bbdb35e64ab78".to_string());
    reshxs.push("9dc2b65b773683e42bc7e7c0e9415156724b27a048acf29e831475363ae0f723".to_string());
    reshxs.push("82cff3e7609fd2e89e27744deefc00876d3dc5b8ed1fe1ae7f986f2d8f21aa01".to_string());
    reshxs.push("420277f82b85f1d313a02033146357b6a28aa2801a59d27f85af016b62f54aeb".to_string());
    reshxs.push("9a2306f9ec63e368dfc8fd722f21962354a8ec409ecb786c42b705b4028e54fd".to_string());
    reshxs.push("f92480c216b68794da163ddf9e7bcfe086fddd7ef92a8aeedd6985421acf36b8".to_string());
    reshxs.push("8c0ea13e64d49e2aaf04a97b53a1494c9e5b88bdee23ba8a6f857e24f41c6de0".to_string());
    reshxs.push("f5efdb4a404dfe1d73f5e42f011e299a7deda2cc00d9b00e4eac460514d960aa".to_string());
    reshxs.push("72cc8a2c7fb503ac58b91ccfb9f09ccaede19519653c4a183b8c60a50115a636".to_string());
    reshxs.push("f421d3b2076324b8a4499a73f0aeef3d17996b023389729b38e88a26a5d82aa4".to_string());
    reshxs.push("30a8bf560db0003eb00104a663ad268b8e076a443684f01f62798e14ab795f0d".to_string());
    reshxs.push("7b792bb8b78d422eaae8748f1ec0655ca924e1a72e10029fd77c31bfa33fd8a6".to_string());
    reshxs.push("0da2512f465f984ed137d957f3c4fcd6f09fd485b77e313d4907c7c41974bf2e".to_string());
    reshxs.push("347383980bffd46dca4c4edd451e63a784be1ea589c96ab6cf555307762f6262".to_string());
    reshxs.push("ec0bffdcf136523fa7629b4fd41210c3bb32cb4bd46db304d366d73430022290".to_string());
    reshxs.push("b167bb656e6073732eeea0ebd5b434457b5fca20555fc7cce10a89b7ebe8a0ed".to_string());
    reshxs.push("6f47ad2ffe7156c911cefd4b49ee6e201d3fa8efd568fca0900c4a001a0a17b6".to_string());
    reshxs.push("0231e83990edaaab6d3de795bf795147030ae23165094c0a6a779b41d3933642".to_string());
    reshxs.push("8d66f9383a270e80202874b7a581171bf9ca0381d0d1c22bae08ee688cdc1125".to_string());
    reshxs.push("4977e21af4f673386d20859c40cf6fa836e4bb75a4399fa00cd7d631d0215ca2".to_string());
    reshxs.push("22e244c751f73c9cc1d33928819e2d836562673aaf7b71f33d5a4a4661d699e5".to_string());
    reshxs.push("6305efd1b96b4df2e417deb5d98cb4afc9bbaa8a303f112162c02d0f1b440872".to_string());
    reshxs.push("789e9053126f624698361c6fd1ffbc09b0795829b8b65d684b015599b0d14bc8".to_string());
    reshxs.push("faf38db53364d26a1ba943ad2007de7b931ff25f9925903c28e190171674d773".to_string());
    reshxs.push("e49be52c4c8c1f8e38715607394d25da1a0e2d3ef73e175139bbb484ee3917ea".to_string());
    reshxs.push("4d3315d3737c033b70b95ca09bfecdc1b383b49bb2425202812373ab3ffbcbaf".to_string());
    reshxs.push("8f5d06ab6e4a9525818fbd3231a2975d7a676c346405f7c33c3876ef8012fcd9".to_string());
    reshxs.push("6cb159a4366a575cf99344ad32c6b3134ce85fdf2d4239d57203874622506e6a".to_string());
    reshxs.push("6d8bb4942c233eab034af0270169ae2f3ab995bdc5d9db2caeadaeac86c1d461".to_string());
    reshxs.push("0939e91270c20f9c50371fe21fe3da694b9f32c6c268fb6dd8fe3d473db2140e".to_string());
    reshxs.push("726b4c42bf5f6c45650efd8daf07d090c6dd93f29fb05fa563ace84f17aed8e9".to_string());
    reshxs.push("8077d03ff0b0c0d843723bec06eb2e4c2a273aba9d4577be2d95f30c4314d108".to_string());
    reshxs.push("180604977733c34322c55794155ae2a83fa6faaec6af71d366b6f564078c515b".to_string());
    reshxs.push("9cab46dc2c4462084c46850c32989b2ac9860ac11cc02f9fc38707c7dcd7a8db".to_string());
    reshxs.push("56ef4e92f63cbcde1a79430789929c9b76a82f672e86df9215f2c383ac1e1ede".to_string());
    reshxs.push("9194ba009fa72cddf13b6346dc9f6d47a4d224f30fa96c565ee401bf25c0bd97".to_string());
    reshxs.push("6ea23667b019de3941cc7eb284805f2d1fc47443bfb3ce0a71212f58f224281f".to_string());
    reshxs.push("26e98317dee50d75aad03bd4fe26d33f2ae08752859571dba9ef7c29c8edc9e2".to_string());
    reshxs.push("942c80a446fc6998eff193c47d7775b5b0091c717be906e16514bd4b08ae25ff".to_string());
    reshxs.push("260c482de1ccf11a55c30af42166fe4defeac24a30fd82f171ddcd4eaf4b0eac".to_string());
    reshxs.push("1add756c4a8d149912d9847ca1cee207ea1f511e7a08186dbdc0f9c24043e82a".to_string());
    reshxs.push("35b20c428b5b9850864c399f32a4732e5f15279bb66885ebf7ed41b76e5dfe72".to_string());
    reshxs.push("c192431f58430b4b62c83f7e98372d60d11692be2ee19c485dbf48e6d0bca872".to_string());
    reshxs.push("5bb41ee475069eadaf251616fe37079ba238edaa1c844efe8c29dd2b0fecf9e2".to_string());
    reshxs.push("3c9afd1743a8c179866a0a1a0626137c0fa7fcb00787d1754019a2c6acda24be".to_string());
    reshxs.push("113cf1649315ef383b7e68c5c2320bd1a0ec6539464dedea510558b3ef99ad43".to_string());
    reshxs.push("7b103b59819fa63027d77f2c076f0e2228b105597aac512c2ab5cc0351eb2542".to_string());
    reshxs.push("290284aceaf5199f4680726b19bd69c71e93c61a4ae17288e2a2d6b5a48c772e".to_string());
    reshxs.push("65093eea82c167a049ce0176f03fe53ec6f9eace3479d560f7de7b48472de749".to_string());
    reshxs.push("33cc4e0d56e0617092fdd8d74c44f99687c90e8d5374eb3ed8a4e9d69316cbf6".to_string());
    reshxs.push("b4e00835b1925eda936f46917300de71d9b602144331dae8737c761e80ebc8eb".to_string());
    reshxs.push("554ba3c308f54966ccef87ff52acbb019e7a0435de618e9310d63b0fa693032f".to_string());
    reshxs.push("252920459be5e3b49b9458c06eb0cf913e9b89a708a18decca8ff85ec5542cd9".to_string());
    reshxs.push("3c9dd4a783b99dfdd5f8958c7ddf565b8ad4c9f1b876f48ba8a5076a24c6cea8".to_string());
    reshxs.push("fdb457a1597a0c269af7eb2f60b8c2bd9e85d4aacd897e00d4e4f1635bde2e29".to_string());
    reshxs.push("c94adb805bf850f85b8005baaecebca01b4475f9b6aab19cb17de4fb0df26e77".to_string());
    reshxs.push("5ec7485ffcd2d89e6ba97c4c1789c66d6686b6730f6ec862fc9b8ef8f360eced".to_string());
    reshxs.push("e716a6b8a9560a597e52f77e7909170eb8d5f764329695eb55d92d12e4fd289d".to_string());
    reshxs.push("cb81e5b216d0cad425a71c126dba1d85e94b26c852badbd11d16f1e714c0802a".to_string());
    reshxs.push("95706074efc49e715e162774e3fe644be9babbfb968d08c6d179773bf98a425e".to_string());
    reshxs.push("ff7aa3aaae6d4a496dd297412b1fae826152697fc1b0d9eea6b002e165237d42".to_string());
    reshxs.push("de15117cec0c435d986e4bf11ae7c19236834b20102d81ed3fa8b5e93c60bf66".to_string());
    reshxs.push("218f13afb9846efe5e20bd7a9976835c70f099169ffe4812e3abb97ecfe5841a".to_string());
    reshxs.push("32960fb9b792d72dd0cca5a9e2ee7e7de945e85f0823f06a632f8ad47ea20105".to_string());
    reshxs.push("56c30069ae65db12726c980aa2be0c327be0b74fdcc08ea3386a073d70ad0149".to_string());
    reshxs.push("d79dc5b922b99f64607c48f1fdec38ed3172069e2aca37bf7fceb33a877b8e84".to_string());
    reshxs.push("649a368b14a3ad4e8eaddeb4550c966d101ae45717ec33141be8dda9ed7116f3".to_string());
    reshxs.push("47a116d90dbc2ce43d2a5cb3cf824e9496477bd4e8486270d0e5921afa59de75".to_string());
    reshxs.push("20b5ef05fd9a025728a5d219affcc93c44d1264481a4f6979d023e46cf888ca2".to_string());
    reshxs.push("1e535d5d2b6787ad18ae2d799099aca3b89e404a26fe5a67a72cc118d89f08b3".to_string());
    reshxs.push("ad278a2b69f5e59a022d032187d12ab2a6b4df054590cf728e89b707ddd2b651".to_string());
    reshxs.push("61ee5f46271415cb4a84cf42dd077474f3c4766c18d7a015cb18467e523325f8".to_string());
    reshxs.push("9fb8b509b8fdf39ff8fc4b691f251b0861e8177f70f744fba242d20a54d6d5a3".to_string());
    reshxs.push("ce43880cee0d20d1290ece072a1e18ab651f1cc68d6318b28c6a29deec653f79".to_string());
    reshxs.push("3801c1ff3de0a694d8631bf9e0d118f596bc6b39601231ca89fddbba990f89ff".to_string());
    reshxs.push("a4f802da4e9049976edb1249a9f586023460bfd5bb4f4c1ecc03dad182913194".to_string());
    reshxs.push("0cf5427b0537820fac5cdfcc6889fd5a7eb1dce0ab743dba8f58bdd44fcf3aff".to_string());
    reshxs.push("b56bfe64c893e3b492fe79b9045e67bd1fde09abc954537dd1d44fff5b09b8cd".to_string());
    reshxs.push("e3ceddec34495df9367bbb66c3590284bd575dc59677730dba710673d9a141d0".to_string());
    reshxs.push("51206ccdf6ff56a0374cf02ed21c6481e7df53437b25c21f69cbf0614bbe2d03".to_string());
    reshxs.push("f9844a88f61da9ca479e4e66bf31a7cb1f7dd8b4778a1ec599437e0fc672b273".to_string());
    reshxs.push("ded479484dbd0e2c0d7c5dd4ad00e1bb331650d52619f8c0cb87f65f3f6a4bba".to_string());
    reshxs.push("d10d2bffd594627851d57e41c6e7e6ad134f1efad4b90e50cda7c56e650f89f4".to_string());
    reshxs.push("d99e7a9e0535608c9e03f2351d3077935784a2de67f5592be4cef66cba26c3cc".to_string());
    reshxs.push("66ed479a1d630fd7ad4b7498fcfc71343a963c0e2ec58966e607f889a54cb17e".to_string());
    reshxs.push("bb1cac569d9a5daefd6cd3bf6bc5117522ab8bc6427be037927e35fcfb5b4857".to_string());
    reshxs.push("489369cf2c9a13ec4f1a065f0eefa035b6fc4b99a92de6d2f74b0aa16a639493".to_string());
    reshxs.push("7195d0476e4b768ec4bd7d13b70899e68547702627754a1acae7c740e5821c3b".to_string());
    reshxs.push("38bf0285b0df1258b12c568ac39de4d68b79114454a1321159b5a2e01d9b5fd1".to_string());
    reshxs.push("449e12a918bfcaefbf867fa2b41b910fe27cde25d6aeed0cd1b0a641dfb8bf82".to_string());
    reshxs.push("668d68fc8b5826521accc2bd4998ca116fccdaa620b4d002291a1c68a910e744".to_string());
    reshxs.push("4226b22032e0c1369177b9454a4c9e20841083bcf335ec7534ea311537a2bc6f".to_string());
    reshxs.push("7d80567a8b0f6532d3f8ea5b2b13e959b61e081ef647f13174264413af323dcd".to_string());
    reshxs.push("e707f28b98c3d8ba6dfd90ca305ad369dcb08b8874ef82a56d84c2634b6a60d7".to_string());
    reshxs.push("fda0760162f859007a527e48fc327839ad978ada44ee7a921fc2c1bf15e9db21".to_string());
    reshxs.push("b9129e6c9499d33abb894fbe23358b2d97611ee8180abb804d6dfd6f67e9495c".to_string());
    reshxs.push("988054f9b3ee3d48b4586296b2881fdd8f549f16b192b1c11bc93678bbe54be1".to_string());

    let mut hxdata = [0u8; 32];
    for i in 0..100usize {

        let result: [u8; 32] = x16rs::sha3(hxdata.to_vec());
        let resstr = hex::encode(result);
        // println!("{} : {}", reshxs[i], resstr);
        assert_eq!(reshxs[i], resstr);
        hxdata = result;

    };

}
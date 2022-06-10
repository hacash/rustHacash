use hacash::x16rs::x16rs_hash_wrap;

/*
    cargo test --test x16rs -- --nocapture
*/

#[test]
fn x16rs() {


    let mut reshxs: Vec<String> = Vec::with_capacity(100);

    reshxs.push("6fe2a4b96f71518b7603e5c63702588ba816885aa1ce5908de31335e11473460".to_string());
    reshxs.push("c32f086c22c5fa6cc51196c46ded007e186f20be8717089c6abe47806493c35c".to_string());
    reshxs.push("c6a40468628d854643910449ceaba1845e31a2effc88833c4a186a2fc0445b24".to_string());
    reshxs.push("819b6b79617fd19e7a77a112ce0446852b8bc87a286141da91314dfba7b3952f".to_string());
    reshxs.push("8ead8b0842fb850a0ce0b8339268991528251e5956626fca4ff7e1760c705f31".to_string());
    reshxs.push("a8898bd9ab0cc07f4bdd506f8f7abeda65fe13c7d0309a2a25b3e5c3c2b4767f".to_string());
    reshxs.push("7e41b5bdfc1dd219c828c5f2e7678d9250fe370b3c22019e81efba5075e56338".to_string());
    reshxs.push("f6d48246bfe1335ee2634ce8389661c704e4114085dac45ba9fb90812d7eea47".to_string());
    reshxs.push("07df1f6a2622c891523ef49d2843c57f12060362d7c73de078288d9805dc66eb".to_string());
    reshxs.push("fd0decd4cddc04f36c01466d0a5f946734cc77ece951381168973c657fd66b02".to_string());
    reshxs.push("68145e572e8e0ec22e5e761ac071f3c6a86d8979e64cbb3cc6e379266c294253".to_string());
    reshxs.push("87171e0791795a5cf7911ca09d2520bea34b9d4329f3fb088d700a2555be5412".to_string());
    reshxs.push("3430c1629ae7fd8b0c2f0d0f3936da1db8589965cbc6080e3f45780c58441adc".to_string());
    reshxs.push("4bab26db94f224072d7632d03ae11e8f0b1e8eaf3d8a8ab8698fea6d72029d8d".to_string());
    reshxs.push("a8527fafaf48583350a5d50c33712bdfdc7db61fbdfa0d848e6caa870432c3d2".to_string());
    reshxs.push("466405795a9630c800ca1fe951c98eae640ce7de666cf5131a7ff157a8af61ea".to_string());
    reshxs.push("e8b8c0ef2912e3d44e9beed9c8482bd69a954ae86fd3cf7e68b5988380e22407".to_string());
    reshxs.push("3820cc031f91c499f75845f72c127741f7c2f79a418155639fd7e9475f187e79".to_string());
    reshxs.push("07b4ad143f2e66afff2d05d18a0521309a15763722b8816433e025b566609ff9".to_string());
    reshxs.push("76f83bdfa495c75132f5643500c1570fdcb884b36ee26faf0a952ab392f4230b".to_string());
    reshxs.push("303a46de3d4f8e6df5c46e622cd18891bd8da6b4dd8fb95b541a8a65632e841f".to_string());
    reshxs.push("cea089ebb6b1840289f50119ae7ec7ab42b57dde9237b8c7b62879add78a2585".to_string());
    reshxs.push("9b59ccbc8f07ef80071cf1b1c5ad135b4f6350c15d0c5873e8f9a12ff8f021f6".to_string());
    reshxs.push("285020b2dfe41c986a59dd67488a15cb4f425cebc2dfe995198b19687b950d23".to_string());
    reshxs.push("2708ae7fd119543ee30d6aeb9900e7f0111489c016e76df279a2d52f20fa389e".to_string());
    reshxs.push("cb1895877be3303142827319dd6e493636c2ea9130f2b613101af95b9ab837f5".to_string());
    reshxs.push("36005911632d06048f1d2010185b2793f39cc22e2f0ac862aa105a5981191ac2".to_string());
    reshxs.push("7c08d382ec104afc950deba2170eaef622c73331917994d4d64750893d0eaa55".to_string());
    reshxs.push("c26a298ae9fad4239b0fcfe9a84d599b996dc2ff229786e0f649ed3c9e05a05c".to_string());
    reshxs.push("0b816562dd1a548e74d86867500477645f2037bd1826e87f7b2e05c54487d49d".to_string());
    reshxs.push("64b32d59748646894c84141c67635c3259611b8b86473f1b71ed7fe3cf6c640b".to_string());
    reshxs.push("0ea24cda360a06e98803b1b387c6239a5e69cf5211f77fca59abf611d8cd05e3".to_string());
    reshxs.push("12c1ac4b96d19b18315cd091950113cd886e59a5510e80096f578f2185a80aaa".to_string());
    reshxs.push("4eade4452dc8398a2ae96597d3b20985100465ccbb0b6306f4981022410e5d24".to_string());
    reshxs.push("ca7429cba5cab05a8ac4ceadf3b65ff4fa66d4c26057a8e3dc205a329528b120".to_string());
    reshxs.push("6b42fdfcdde338f6bc6bd6c263f7c7f358e36d81b812703024799f29d717dacb".to_string());
    reshxs.push("1ad18f31443643f78966c6a2285eb29292f930f46b675080eed1ea2a1e689689".to_string());
    reshxs.push("bc353a397104f76dbc1c75501d2f10e3ba92e4a95754899cd55f1253fce6aae3".to_string());
    reshxs.push("66527934bbcabf17c6d888f2f01bfa3236c5004c5bdfbb769fb8e5e948da316f".to_string());
    reshxs.push("00d50ee4f39fb8a6ea73884754cf79bb13628e71778d0ed96ce0f23abbfeb928".to_string());
    reshxs.push("c76ea6196204d133a7df6ac16b1f0ff26aa80ac3a1f90e41740da3ae8f379ed9".to_string());
    reshxs.push("dd662830f0e4c0176759d38436a54e1c25373a11fad2eff52504151c35094a31".to_string());
    reshxs.push("7cca9373a16a5b123bd817c419e222a3f8f501f91aed4b054933b9a262ffc6bb".to_string());
    reshxs.push("0334e3382ed51a79501b71e1133e8c3c753816a7ca6d675638ee3d99f58b0dd7".to_string());
    reshxs.push("272c530ef158c100f9fdef8a51966f4e4a71acb52f8ca12ab6d247b1a5571026".to_string());
    reshxs.push("cf9623118bbfabffc7e5f8c0c921c4a2d6b844600fbdb29f3674be19fd7443cc".to_string());
    reshxs.push("24527c3b6c76630081ea8f64983ffed6994188a74a392976716bac0b51a6b14f".to_string());
    reshxs.push("025893c28331a50e31ade6139341a98df6b184dba4aaf573195f0c00ae1f5fe8".to_string());
    reshxs.push("eb6f3586d167d662d4baa02d248aa05158904fdfada43e8c809bd728e8209508".to_string());
    reshxs.push("35bd1b482077062ec3ff9c84511071b9c0f4bce1b3cf4e2c3d87a5f88cdb0e64".to_string());
    reshxs.push("1e07afd466beea2dd817737d98b66ba0d582ad4d55403aa5efb554c5ab0d3e7a".to_string());
    reshxs.push("4bdb269ec125e7e0aaa19a4bbf30ba6f3aa609e0713de573d30de94298b4ea6c".to_string());
    reshxs.push("bdb29a6d2f30af8c990daaeed6a72c703613a6bed891b5a2bbcec8264312b2f3".to_string());
    reshxs.push("ba428dd308aea5b6b115cb5cb49213e9da7a8a1d84d27d63c698233ba65dc49e".to_string());
    reshxs.push("c5bc9e5e745f8a5b15b844368e47e0002ff23c9c731e94300d290f39741ec431".to_string());
    reshxs.push("abb2216a9b159595413cb59f3e3fb5f5efdf3cb433d640b5aebf250d1175c3cd".to_string());
    reshxs.push("84702fc62d8aca746e28108fdf9d3e08196ce7224fc7178f298df40038c6a0e5".to_string());
    reshxs.push("d5a32ef67a818618f4fad2304065f65b0463bd9e855e4bb322bff17fd5a88dc9".to_string());
    reshxs.push("503bdbe0204d382deae0a59b60bac1cb389d0c2997909bca3d27ed1c9a927ffd".to_string());
    reshxs.push("6c47cdfb8ffa6ee61bb4b413b0079192e397b6349732dbbe7632739a6d9690a7".to_string());
    reshxs.push("3862aed1f0c69f07dbde902d8fc4f294afec8a3225ebc07aa90b876152f80b4e".to_string());
    reshxs.push("5f584e87185640ea3e5e7c985f8bc643b9ed0f31fd994d0eb2c6c0a813c3891c".to_string());
    reshxs.push("635f6766a8c450d97783b9aca2001388f5565989123e1dba9376df997fd2cec1".to_string());
    reshxs.push("688ba4f6ee7b1e9d4ef37d87d1372723762da15e8cf7f46b534443d7dee87d38".to_string());
    reshxs.push("2e4fa6c29dc551d2f90d2c66ca2f201080cb99b47faa97b950c937291459023a".to_string());
    reshxs.push("5a42ad942e923c3eec00a5c0bfa59aac0c22154dfcbbe5cc1f0816b096dd8728".to_string());
    reshxs.push("f1fde0846f3f042b8f4360e24a2a8814e278317a1a7dd62521baddc3f3256834".to_string());
    reshxs.push("c6eb44b469f49995d560fda6f8ce91b6fb89f394c14e7fff1e4b0d0816af86b4".to_string());
    reshxs.push("053c723c59534ebdb28164eee873320ab83e8f985b474a339225d99279c40d67".to_string());
    reshxs.push("223030c5a859cfc2e989c28554ff729211a2054ffa37f49d365974093ecd2431".to_string());
    reshxs.push("1c5e963e9a389efdf72e1867dfbc2909c55c3290226bb6a030e7e3fd8a3ab2dd".to_string());
    reshxs.push("0fc209201006fd1c661cd2a23d9b8a59199c5ad915575ee0d9a7b9b20d6dbae4".to_string());
    reshxs.push("9324ff3c65a91a70f9a66b98eef776b15026e3b07d0e2e42acb7bff29136b855".to_string());
    reshxs.push("5ccbf3a19f1e3e2a41e391285ff41bbc92b3c0e3c5962f5fd83f07cf1b1e8c18".to_string());
    reshxs.push("b6b5cfcd52f8059b740a4239a9136f39ab34bde19b7b12539fe0e5591bb3bfd0".to_string());
    reshxs.push("e796e94b43b3664f0298c91df00abaedf13094f7285eef3ef30bb321fe3b05ce".to_string());
    reshxs.push("179cb88ccd2bfe2abbf744390b174bfc49df48d33ea562772fcb6ec0edd416d5".to_string());
    reshxs.push("72246d209d490bb563911085f0c229788b023d099aa8187ff988434034cc411e".to_string());
    reshxs.push("aa5d4b55d836031b88c902ee13e2d436377d4767c88591542d4aba1340eb186a".to_string());
    reshxs.push("967dd6ac8e87c40566c56b573a5ffd0b9e7ac328c096c76d096574e35ab4d948".to_string());
    reshxs.push("2f88ba744a90b0f97904c87e6366f7acdb32529d33f3478ba208efc6e2bda247".to_string());
    reshxs.push("1d28bcc6316fa54968ce4b383eddcde7f9751a282bafd041ed40e49f3714665f".to_string());
    reshxs.push("fe50c419a674c856fc4f6ec07c6c64463fd7e09f4cb8c9fb1012c515d90edd13".to_string());
    reshxs.push("245159adbbb88f5cd0381ff4d2740cb489af10f4973168a0d9f17065da1b7cdb".to_string());
    reshxs.push("c5e43e373a20f60d6fc516eb2e496f8960ded22b2df430c951d5aadefc20c2b9".to_string());
    reshxs.push("2c1a4008f8f8a5a8170ccb38bdfdda3c5a8da38ca05372ea826ac1cb0db2d8aa".to_string());
    reshxs.push("38e6ffa05e9b6e29dab382459e4c20c60766d2405c01b3d9996b868b661a986e".to_string());
    reshxs.push("7a28bf759ae395755337136fc7f11d8760ed153a55488124a9fc4db3472ea46f".to_string());
    reshxs.push("b21bbaaef4020c773b8b7eafaf6b09c978d6e0681782171dfb0a297d95a5e45d".to_string());
    reshxs.push("1955c99a62a7e6469137ed151ec1750336fd4f0be061a17961923c1dd6aac755".to_string());
    reshxs.push("c2ebb6df665dc5d5bb747bf5e7b8800d10d00cbe2410159fecbd86ad1e746831".to_string());
    reshxs.push("b11f8266f9273bd033df0bc172d3fdd46c7110ec4867eb47bfdb87e8b4634e02".to_string());
    reshxs.push("7432e2777f3e5ebe85837810629d261657148325806bb6b8d86a95756e9bbd90".to_string());
    reshxs.push("4a6f0786f4a53f86decb5663de2cec7415a33e1577b728d6bd99ddf6c0fbef28".to_string());
    reshxs.push("9684f32b2385288c57cc9b29ade136640eebda488dfbf08f4b5621cfeb447164".to_string());
    reshxs.push("cff670c9908983e7113e2da4c42bf732b1d25879d53e799e2f4507482fe07374".to_string());
    reshxs.push("97110fdd30eb9d864600549c33eae5ea2daf7b02c8da9d2a1968205d48f29ad6".to_string());
    reshxs.push("3942e1c5dd1ad431051b2f436519219dda3c280841e7471d3a4192ecfd5f7b9d".to_string());
    reshxs.push("b6abd1a8193b3a9cab951f3ecbcf7e70a59bfcc97b7ee58334903b8c86e28c7d".to_string());
    reshxs.push("c43438c9126c2c60d19756c87e35b1e4e1ebe1f873815c4ddecd1a2f613cd3a3".to_string());



    let indata = [0u8; 32];
    // indata[0] = 1;

    for i in 0..100usize {
        let resd = x16rs_hash_wrap(i as i32 + 1, &indata);
        let resstr = hex::encode(resd);
        // println!("{} : {}", reshxs[i], resstr);
        assert_eq!(reshxs[i], resstr)
    }



}
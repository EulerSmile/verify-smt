use smt::{
    blake2b::Blake2bHasher,
    merkle_proof::CompiledMerkleProof,
    h256::H256,
};
use hex;

#[test]
fn verify_proof() {
    /*
    proof: 4C4FFA4C4FFA484C4FFB484C4FFA51FA8A2D4F231623D87D7283993E36751FCBA64105322669BCBED8609221121260A87EBBFB3D7FD98BF0684FA83006ABD9BFF58C4C369957FE27D89D309BE922E1004C4FF84C4FF8484F0248484C4FFB4C4FF74C4FF7484F014C4FF9484F01484C4FFB4C4FFB4848484C4FFC4C4FF751F7CD9F8D483766F2A51066CFB69D2E0773E246476F167E9F2D7CB64D4005120DDC6CFA5D47A387DCD0B3DCC8E5C3BD9B5270DBC3E73F0FECAD48B94D5319D73E004F034C4FF851F8D75B5BF5012812E8D12EBE69C41912360CAFC046778D445F29E6571CF3DE140425E18E85C232082C1E1D803EC37CAD9A3EA7FD60C2EAE98938FAF65C4E81DF004F0248484C4FF84C4FF8484C4FF9484C4FFA4851FBC813908757E46041AC2C0BE5826EDE37C9F10341909B2590E06B39C729ED967FF2BF38FE0F01B2D44843B132F42F231E49A5F45A504CB39A160D2A07707C26004C4FFC4848484C4FF851F80193064BE6DBE3E832848FF547E69472F6813AA1AF08826BBDEE27DC491D18584133E65047FFBA6E69829931593A50CFC341E4FC1918C8A8DB7F24FFCF0451004C4FF9484C4FFA484C4FF94C4FF9484F01484F014C4FFB4C4FFA4C4FFA48484C4FFC48484C4FFB4C4FF951F9B5296C1C2BFCE6660CA7D361D6093AE1EAD530D7A4ACFFB98D6C22F8A16EB90262A73A4569754E7501E334304AB3D011458812061FC7568ED3AFE192D13212004F01485101BDEF443A707B6FCFF2612EEA44A11FA0865AC6954546804457976A4F81EA1BE100000000000000000000000000000000000000000000000000000000000000084C4FF851F8AC8C8F6E201DEC1F5A47C9E45A3E3E4EA103070F147EFD8FABAE064787572C805F6172FD7DFFD605D981993F15E04A8CCE70FDF230FA66AF3C944F4E216A8E0051F9E87D953DCF69BAF8D05E84910F0ABD2E2D9154ACBDC9FA0556FCDB0A2C67BF49857A56890BB175275F36A5B9BBC975203191BF4DFAC430A31F4C5FA3D2416A004C4FF74C4FF7484F024851FB5F83B59E60FCB3CFF94945B9566811BB90A254F4EA0CEA5EE448EB47B752A9325DBACD156FC9BE60E727D40F22FA9E4F484CAF59C2F323757BD7084BC3F30C074C4FFB4C4FFA4C4FFA484848484848
    root: 0A6B1AC4FE13B31D8EC8F044401F4B6885287BE470EE187EFA4F4CA9AF0F50BE
    leaves: [("D426F88BB5ED2D32A82620FD07C52B0EDE8A5CC58358A502D8952891291D3429", "22A7A095E0D5315E6D26785356455980D6DFE58C0D2DEF2776B1EC65B4036BA1"), ("9BC4BABBB757253FB62D479F48426C72037BA937E49D62D3D7E2423BEB9D3119", "D1C7E9BAE8D5F0332B1FF35BC7953ED52804D5DE64B5FC3D60A21A87674E72C2"), ("66E74B21BA413CE72F231EF522A63AB59DE61357FCB3C35C01E002021CE00B7A", "A14901EACB9DA705BB393B9426D7B5108A46E8A8BF85A46FD3E062AE45BC83C0"), ("5D3425095D80434A55092DA9D3F38FEE9A23B3EBF35C9718C50CC58676674C67", "D0AA56DD81C7492D2FEB91F8F882A6B1C994695D289C97B7422F73190C18191C"), ("5328FD88A935FCBAC10B5179CEAAC6014A65FCB455064AED690D7DE2FB54EAEE", "4507D335BA0C068C57F5CCA3B328943D0FCB32BD4F73EF28D9B45EA960394145"), ("BEA3777F3F81D54F5FE6A3ABBCE83F7BA71C34BDD6CE20FF28FAD18622816FF8", "EB73F11C3DC287CAF3067DF9693D15E8241C4B00AD51E1F8FA9A33D92841183F"), ("AC95A9B4CB0606BEDAD04A84D74FD29BC0F2E4732394C2461AB0B16129FAE95C", "350B2F865E7EE1CB0EA31C22437B7EC7A25A30FCD3788D909BEFAEF69A132C5D"), ("A07943E48E7334215F591190BE988A55DDC16FD18F3CEB05738AD7E2948DC333", "AADBBB59D8810D95ACD23117E04B98CFB22669A15EA746602109FFEBB6764A2D"), ("A967C4671207A5FFF7AE152DAF1B1320A283E4E06AAFA0D7AF20C348234AB7CC", "A297FB06312C39C998394CB334961A1BF7B4971ACCA5825494B7172B2FF1D008"), ("B711D0BC7EACF53CC55BBBA64ECE77D2E9D05399C34C332823F33EDE4D5BE9BE", "9BD46B90EB14CEAE67FC03A99207599C470B91B96B5591D1517139E08C81AB2A"), ("A3925B0A90060F896585F0EF08A5141C957EA3366299261D97F3871EF749A218", "8DCDA5F050F8DBDD91573DBA6CB02943D1F3EB7E0BAB4011094DB164F08B232E"), ("CD153C09F92FD52AA741F28FE814FB91777681769E7BDEEA0D5BB05636438EC3", "D35097A2197A75C15918A611BADE780A39D10ECDA409075B194DF70207CE6D5E"), ("46C5CF93BA4E54CFA37E50DD86C45C07EBF106D3ADC11ED8D1DAE03042559403", "A426785443043D9F8AF61F7047A543A7340A298EE48D8D024DBA094E6BD1807B"), ("445EAA116F45BDC2DDCB7D2CBC1371BCB7555F8F378E9BAD574D4733471A5A91", "A5DB24B1E8FAB36E175E21B0F8662BA986160FBDD159FFB85F75F31081D24DDA"), ("5B0248F5546B2BF059E62E71578498BFFF661FB6A5FD2F6E1B7A58E97F745B62", "37C216F158FA1431A31C6FAE1658D2C3F8A2F7806F6135F11B76A2D64752B028"), ("BF1A28900E937F1DE32A9C18745F3492B663326DF1F9EC6A51566206742E5960", "3A6245DB276675B27B4F8EA76F4177856FC8893FFEB6E6421CFC63D52478C01B"), ("42FF175EEECFC3AE7CA9F57E6C18A4755FCEB575700B248F7E6C6B177A5EBB99", "0866C1CA8894D04DA629F85347146FE6E16BB1998B5DB3CB3D00BBCFEA529756"), ("B53E4EB2046A01B44595A62D5F138483257131E6194E669E03E55738890FF62A", "18812EADA49C5A3412F5E0E983BFDF7825F5F9AB54A20BE013097473D84D833C"), ("3A17FF0F3DBF00F28C0BB9F8DC8BCAB6BF80FBDB00A2459630E8333430388761", "07ACAD9E117DD7BBAB780C6EE97466AF436B647024B8F308C92964C48786ED42"), ("633D168F092C618F7D6601C52B3DC5B00B8833E2EFD181A2ACB8DC781F67F026", "ECC2F5E35099B0868245CA1D25DC1B4CD5FC6030F04B1EF0BBD2937BF497AFF8"), ("491C7B3CB4A271B5B031815921B75C14C491BC9C2BA9E981C821CDE0DE31E10B", "7A06BBA73C86F6E156B6BADE64363068E9B37ABE1F1A7CFAEA64B25E1573F94B"), ("449BC17D5883B34755D72FF74D827E905DB1CA0431567DCE56DA71BA60E8B2AD", "C85C11FE8F8FDCCFD8C2E7F53E62875493223E82D9FCD1F876931477D965DFFF"), ("E1882C813BCD2D3FC6A271E97911C0323705E562EE04F5DE568791EBAA8706A0", "8D8B785FE0E87AE372669A21785F68060EC006E480DDFEEBBAEC98B3D7A1EFE0"), ("30D9BFD03338DC86004B79200BC0D134EA5AE97B7CB77B14EB82D281FB182C3B", "C9FC13BEBC415279208D895D6071702FC7B1E28288AFF21E22857DE47DEEEF67"), ("7C18C2FB8EA12304D0062903E103607621DD2D4C48025D69328A63B5C05F65EE", "B06A0C9CDFCE99C0DBF0A073C7A1D732E62ACFDF479D3D84BF66441300E81173"), ("E6327088A95A4399C6ECC0FFEB1D9C1494CE57BB6C6AE9352DA47FFFDF93819B", "8D58155BA53EED929BBB3C2665778849AD53502CD30BCD08285743FF993D6CBC"), ("0B18F051724BFBC0A6D331CF774C26CE80C61EAF2A4151CB04DACE044DD38792", "877AA0398D69125B949D156A6529CEF859506E7F622CA3C1696B73987B0D50AD"), ("0111997FFC3C0C07B9D73327B168CFCA4EB1922912D7B6AF14400C7001D0AE4A", "679BFF3E5674A48F6152EFD02739E9F41A0F49103BB3A71B16250A0929449A42"), ("8829708ED2BC94C6E806231BBA808CF7F9FE13653E3B7786FBEB279B9D11C7A9", "80BC4930E1E2490F382072AE601621A3CB390F080F0084807057F9E60E6768C3"), ("59A7ADEB7F39931E54B4C50D0E7ADE0E01D44C71DD96FFC1FBE3F646F52C2596", "939C55B22F2F014A739EE7F7FAFE4877D014AAAE41B048C506CCD2BE17491E6C"), ("7483E51287156C0A9A3FB413C1C2601B172B3FEDCB758B780A98E28F43F0A0F5", "CFE31C569DA066BB904A8696F71516366A8B7B8B5441E0828C91A3BF5E84A29C"), ("2B8927D345FCAD11054068D9F45EE1BC1140F3E9946E03746DE59E06DA267614", "9941AEDE21CA157BCA03D3E437164C34E22ABF3FA3117CC190791C501FD9DB96"), ("C4563D1B8D815FF4548345E8B2A490E4AC506D868B7CC1302306C28A5398ABE8", "E8411791FB0BFC17B2A8AEB82CB0298BB4F5877488462130A008CFEB8D348D1F"), ("70971A06AEE6F7E0538B9C566117BA0A4CA9573C7F15E86DF5720708B5423F56", "A3381E00E3EB0D74DF525DCE42D651BE749C998FFD4CFC5430271591AED21464"), ("18A0B2640A87D84DA83B7D89817D95157FEFD0FE273E6C51A32D56F2C20A5704", "4840A10FB89FC158A6F57B725916A3CC9EA74158423A65AB775EB9A5C3B48B55"), ("CC681898EA5CA2BD17C3E292197AFC480085598BF2A2982AE2AB6FDCD6E7B4FF", "8604DF7BC1A8F7DABCEFF20B9A339710BEBAC442B7DDCB247E8334DA70BF0CA9"), ("A6C629FA641D0529B178CCFC2101F9C6D597C0FCE3DFC717F011675B8223BC29", "84E1C3D490C0752F1BBA69FB8AC7E416AF7528348C5031552EBD278EC23B76C7")]

    proof: 4C4FFD51FD7AA76A162CE575B291463C5C580504925B2E7854529D42E28BB5366C1849D59FB9AE1A718347B7589254B864D584DB841BF7E68E219EEA2461BB9180E6E4A018510417C0A3186D3D15D4CD130CDB43C153EA809EFB6EC67C45EDF11AED09EF6765A00000000000000000000000000000000000000000000000000000000000000000507B020292F4C5482E3846F6BC7EBC6B9F1D66E12813089BE9A26741623DE77482
    root: DBD8683EE6F305B91A3E6224CE9D64F39049EDAF3F8D0176B0C21C36497923A6
    leaves: [("9E1A2665F327BCE7D003E460DBFB441C99629E0294C860D8AE66A6970A5B1167", "3D0A4B726BB0AB18EB5E0243DE30973FFEE0EA268EC561819564A126C1C66C19")]
    */

    let proof = "4C4FFD51FD7AA76A162CE575B291463C5C580504925B2E7854529D42E28BB5366C1849D59FB9AE1A718347B7589254B864D584DB841BF7E68E219EEA2461BB9180E6E4A018510417C0A3186D3D15D4CD130CDB43C153EA809EFB6EC67C45EDF11AED09EF6765A00000000000000000000000000000000000000000000000000000000000000000507B020292F4C5482E3846F6BC7EBC6B9F1D66E12813089BE9A26741623DE77482";
    let root = "DBD8683EE6F305B91A3E6224CE9D64F39049EDAF3F8D0176B0C21C36497923A6";
    let leaves = vec![("9E1A2665F327BCE7D003E460DBFB441C99629E0294C860D8AE66A6970A5B1167", "3D0A4B726BB0AB18EB5E0243DE30973FFEE0EA268EC561819564A126C1C66C19")];

    let compiled_proof = hex::decode(proof).unwrap();
    let root: [u8; 32] = hex::decode(root).unwrap().try_into().expect("failed");
    let leaves = leaves.into_iter().map(|l|{
        let left: [u8; 32] = hex::decode(l.0).unwrap().try_into().expect("failed");
        let right: [u8; 32] = hex::decode(l.1).unwrap().try_into().expect("failed");
        (H256::from(left), H256::from(right))
    }).collect::<Vec<_>>();

    let proof = CompiledMerkleProof(compiled_proof);
    let root = H256::from(root);
    let result = proof.verify::<Blake2bHasher>(&root, leaves).unwrap();
    // assert!(result.is_ok());
    println!("result: {}", result);
}
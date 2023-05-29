use verify_smt::h256::H256;
use verify_smt::merkle_verify::MerkleVerify;

#[test]
fn test_verify_proof() {
    /*
    proof: 4C4FFB4C4FFB484F0251FE702D1A70F4725733065D0DFFDA44ECAE822EE1CE769C11156636F0CDA502B965470D040FAAB9BE0C0487D0EF98B49776ADB757ED05BA06386B973F88BEDBB3174C4FFF48
    root: B4DC9545003863D6452CFF308DBCBFD187A9BA3EE23A16E492B3C960DBDD4613
    leaves: [("C99E35BAAF5FFE6498C151BD2CD4CEFBAA5A3F391B8D5C4877FB563E9754360D", "0369F81A383176F2771F70FFE0B83FECE38652F2D639A62E3D750D7C45CA4224"), ("8E6E6C29288A092B54FB22276B5C60ABF55E5F61BA50B03E6F10764AAD97A400", "2980DDE5970B55A6030125AC3CB5B74BB9225A317A23F6B3E074511A04E7C1EB"), ("05267A33D8CBBCEDB9CFF79F5A0B01A9CB51F57484758A5C710E29406E99C789", "CD808D9534D7507FEFF2CD5241D6CF6FACEA4D84BACE1C53B3B79ECA5837F45C")]

    proof: 4C4FFE51FE32D72DD0B7036C66440E54F19A45E59D7810C3742F7832D6EDE9EA57635FE2A8B8D9D132B4568CF01A4732C5AD72EDB365718BF285C8B255683BB27160D516074F01
    root: 5733D4F3FD12C4D47772E0F681E385C7F49FC1442009DEFA005B5CA371CC1D2A
    leaves: [("4C4D606D481FED7FBCFF67C6D9186C2F07156C777A1F8B3F19333F74DBF5A2DA", "178D1D3B66FB6B7670BB0438A4F9152FC7A8875E94CDF1391169A0F4BF7ADFC8")]
    */

    let proof0: &str = "4C4FFB4C4FFB484F0251FE702D1A70F4725733065D0DFFDA44ECAE822EE1CE769C11156636F0CDA502B965470D040FAAB9BE0C0487D0EF98B49776ADB757ED05BA06386B973F88BEDBB3174C4FFF48";
    let root0: &str = "B4DC9545003863D6452CFF308DBCBFD187A9BA3EE23A16E492B3C960DBDD4613";
    let leaves0: Vec<(&str, &str)> = vec![
        ("C99E35BAAF5FFE6498C151BD2CD4CEFBAA5A3F391B8D5C4877FB563E9754360D",
         "0369F81A383176F2771F70FFE0B83FECE38652F2D639A62E3D750D7C45CA4224"),
        ("8E6E6C29288A092B54FB22276B5C60ABF55E5F61BA50B03E6F10764AAD97A400",
         "2980DDE5970B55A6030125AC3CB5B74BB9225A317A23F6B3E074511A04E7C1EB"),
        ("05267A33D8CBBCEDB9CFF79F5A0B01A9CB51F57484758A5C710E29406E99C789",
         "CD808D9534D7507FEFF2CD5241D6CF6FACEA4D84BACE1C53B3B79ECA5837F45C")];
    let result0: bool = MerkleVerify::verify_proof(proof0, root0, &leaves0);
    println!("result0: {}", result0);
    assert!(result0);

    let proof1: &str = "4C4FFE51FE32D72DD0B7036C66440E54F19A45E59D7810C3742F7832D6EDE9EA57635FE2A8B8D9D132B4568CF01A4732C5AD72EDB365718BF285C8B255683BB27160D516074F01";
    let root1: &str = "5733D4F3FD12C4D47772E0F681E385C7F49FC1442009DEFA005B5CA371CC1D2A";
    let leaves1: Vec<(&str, &str)> = vec![
        ("4C4D606D481FED7FBCFF67C6D9186C2F07156C777A1F8B3F19333F74DBF5A2DA",
         "178D1D3B66FB6B7670BB0438A4F9152FC7A8875E94CDF1391169A0F4BF7ADFC8")
    ];
    let result1: bool = MerkleVerify::verify_proof(proof1, root1, &leaves1);
    println!("result1: {}", result1);
    assert!(result1);
}

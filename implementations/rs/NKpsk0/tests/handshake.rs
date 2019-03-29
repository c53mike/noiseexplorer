#![allow(non_snake_case, non_upper_case_globals)]

use NKpsk0;
use hex;

fn decode_str(s: &str) -> Vec<u8> {
    if let Ok(x) = hex::decode(s) {
        x
    } else {
        panic!("{:X?}", hex::decode(s).err());
    }
}

fn decode_str_32(s: &str) -> [u8; 32] {
	if let Ok(x) = hex::decode(s) {
		if x.len() == 32 {
			let mut temp: [u8; 32] = [0u8; 32];
			temp.copy_from_slice(&x[..]);
			temp
		} else {
			panic!("Invalid input length; decode_32");
		}
	} else {
		panic!("Invalid input length; decode_32");
	}
}

#[test]
fn test() {
    let prologue = decode_str("4a6f686e2047616c74");
	let initStaticA: NKpsk0::Keypair = NKpsk0::Keypair::new_k(NKpsk0::EMPTY_KEY);
	let respStatic: NKpsk0::Keypair = NKpsk0::Keypair::new_k(decode_str_32("4a3acbfdb163dec651dfa3194dece676d437029c62a408b4c5ea9114246e4893"));
	let temp_psk1: [u8; 32] =
	decode_str_32("54686973206973206d7920417573747269616e20706572737065637469766521");
	let temp_psk2: [u8; 32] =
	decode_str_32("54686973206973206d7920417573747269616e20706572737065637469766521");
	let mut initiatorSession: NKpsk0::NoiseSession =
	NKpsk0::NoiseSession::InitSession(true, &prologue, initStaticA, respStatic.pk.0, temp_psk1);
	let mut responderSession: NKpsk0::NoiseSession =
	NKpsk0::NoiseSession::InitSession(false, &prologue, respStatic, NKpsk0::EMPTY_KEY, temp_psk2);
	initiatorSession.set_ephemeral_keypair(NKpsk0::Keypair::new_k(decode_str_32(
		"893e28b9dc6ca8d611ab664754b8ceb7bac5117349a4439a6b0569da977c464a"
	)));
	responderSession.set_ephemeral_keypair(NKpsk0::Keypair::new_k(decode_str_32(
		"bbdb4cdbd309f1a1f2e1456967fe288cadd6f712d65dc7b7793d5e63da6b375b"
	)));
	let payloadA = decode_str("4c756477696720766f6e204d69736573");
	let mut messageA: NKpsk0::MessageBuffer = initiatorSession.SendMessage(&payloadA);
	let mut validA: bool = false;
	if let Some(_x) = responderSession.RecvMessage(&mut messageA) {
		validA = true;
	}
	let tA: Vec<u8> = decode_str("ca35def5ae56cec33dc2036731ab14896bc4c75dbb07a61f879f8e3afa4c794427635ede06947b2d3acd77a36788aaaf17e9f5a8ac252e560fb421ba161a2cf8");
	let payloadB = decode_str("4d757272617920526f746862617264");
	let mut messageB: NKpsk0::MessageBuffer = responderSession.SendMessage(&payloadB);
	let mut validB: bool = false;
	if let Some(_x) = initiatorSession.RecvMessage(&mut messageB) {
		validB = true;
	}
	let tB: Vec<u8> = decode_str("95ebc60d2b1fa672c1f46a8aa265ef51bfe38e7ccb39ec5be34069f144808843d682eb9cf4fee6816c8c8cfd34c15774321e234e3a426d7cfd3f13e5e84d04");
	let payloadC = decode_str("462e20412e20486179656b");
	let mut messageC: NKpsk0::MessageBuffer = initiatorSession.SendMessage(&payloadC);
	let mut validC: bool = false;
	if let Some(_x) = responderSession.RecvMessage(&mut messageC) {
		validC = true;
	}
	let tC: Vec<u8> = decode_str("b6645684db57679aa08f0b3352d58f32ec7f1e1a02083d5bd54277");
	let payloadD = decode_str("4361726c204d656e676572");
	let mut messageD: NKpsk0::MessageBuffer = responderSession.SendMessage(&payloadD);
	let mut validD: bool = false;
	if let Some(_x) = initiatorSession.RecvMessage(&mut messageD) {
		validD = true;
	}
	let tD: Vec<u8> = decode_str("473a9a4109eba0939e934640d318984df8d0900aa922f0195a09ad");
	let payloadE = decode_str("4a65616e2d426170746973746520536179");
	let mut messageE: NKpsk0::MessageBuffer = initiatorSession.SendMessage(&payloadE);
	let mut validE: bool = false;
	if let Some(_x) = responderSession.RecvMessage(&mut messageE) {
		validE = true;
	}
	let tE: Vec<u8> = decode_str("c8c44a16fff728f83e61272382149feadd3eb0ee1bab6313f84c72fe1581225236");
	let payloadF = decode_str("457567656e2042f6686d20766f6e2042617765726b");
	let mut messageF: NKpsk0::MessageBuffer = responderSession.SendMessage(&payloadF);
	let mut validF: bool = false;
	if let Some(_x) = initiatorSession.RecvMessage(&mut messageF) {
		validF = true;
	}
	let tF: Vec<u8> = decode_str("21354f87158ac5e357529e87e8c84cfcdb49c8a080550c8f908d05ef7ea82ca525e3d1398e");
	if validA && validB && validC && validD && validE && validF {
		println!("Sanity check PASS for NKpsk0_25519_ChaChaPoly_BLAKE2s.");
	} else {
		println!("Sanity check FAIL for NKpsk0_25519_ChaChaPoly_BLAKE2s.");
	}
	let mut cA: Vec<u8> = Vec::new();
	cA.append(&mut Vec::from(&messageA.ne[..]));
	cA.append(&mut messageA.ciphertext);
	let mut cB: Vec<u8> = Vec::new();
	cB.append(&mut Vec::from(&messageB.ne[..]));
	cB.append(&mut messageB.ciphertext);
	let mut cC: Vec<u8> = Vec::new();
	cC.append(&mut messageC.ciphertext);
	let mut cD: Vec<u8> = Vec::new();
	cD.append(&mut messageD.ciphertext);
	let mut cE: Vec<u8> = Vec::new();
	cE.append(&mut messageE.ciphertext);
	let mut cF: Vec<u8> = Vec::new();
	cF.append(&mut messageF.ciphertext);
	if tA == cA {
		println!("Test A: PASS");
	} else {
		println!("Test A: FAIL");
		println!("Expected: {:X?}", tA);
		println!("Actual:   {:X?}", cA);
	}
	if tB == cB {
		println!("Test B: PASS");
	} else {
		println!("Test B: FAIL");
		println!("Expected: {:X?}", tB);
		println!("Actual:   {:X?}", cB);
	}
	if tC == cC {
		println!("Test C: PASS");
	} else {
		println!("Test C: FAIL");
		println!("Expected: {:X?}", tC);
		println!("Actual:   {:X?}", cC);
	}
	if tD == cD {
		println!("Test D: PASS");
	} else {
		println!("Test D: FAIL");
		println!("Expected: {:X?}", tD);
		println!("Actual:   {:X?}", cD);
	}
	if tE == cE {
		println!("Test E: PASS");
	} else {
		println!("Test E: FAIL");
		println!("Expected: {:X?}", tE);
		println!("Actual:   {:X?}", cE);
	}
	if tF == cF {
		println!("Test F: PASS");
	} else {
		println!("Test F: FAIL");
		println!("Expected: {:X?}", tF);
		println!("Actual:   {:X?}", cF);
	}
	assert_eq!(tA, cA);
	assert_eq!(tB, cB);
	assert_eq!(tC, cC);
	assert_eq!(tD, cD);
	assert_eq!(tE, cE);
	assert_eq!(tF, cF);
}
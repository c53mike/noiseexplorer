/* ---------------------------------------------------------------- *
 * PRIMITIVES                                                       *
 * ---------------------------------------------------------------- */

use crate::{consts::{BLOCKLEN, DHLEN, EMPTY_HASH, HASHLEN, MAC_LENGTH},
utils::{from_slice_hashlen, prep_nonce}};
use blake2_rfc::blake2s::Blake2s;
use chacha20_poly1305_aead;


pub(crate) fn encrypt(k: [u8; DHLEN], n: u64, ad: &[u8], plaintext: &[u8], ciphertext: &mut [u8], mac: &mut [u8; MAC_LENGTH]) {
	let mut cipher_text = Vec::with_capacity(plaintext.len());
	let tag = chacha20_poly1305_aead::encrypt(&k, &prep_nonce(n), ad, plaintext, &mut cipher_text).unwrap();
	mac.copy_from_slice(&tag[..16]);
	ciphertext.copy_from_slice(&cipher_text[..]);	
}

pub(crate) fn decrypt(k: [u8; DHLEN], n: u64, ad: &[u8], plaintext: &mut [u8], ciphertext: &[u8], mac: &[u8; MAC_LENGTH]) -> bool {
	let mut plain_text = Vec::with_capacity(ciphertext.len());
	if let Ok(()) = chacha20_poly1305_aead::decrypt(&k, &prep_nonce(n), ad, ciphertext, &mac[..], &mut plain_text) {
		plaintext.copy_from_slice(&plain_text[..]);
		return true;
	}
	false
}

pub(crate) fn hash(data: &[u8]) -> [u8; HASHLEN] {
	let mut context = Blake2s::new(HASHLEN);
	context.update(data);
	let hash = context.finalize();
	from_slice_hashlen(&hash.as_bytes()[..])
}

pub(crate) fn hash_with_context(con: &[u8], data: &[u8]) -> [u8; HASHLEN] {
	let mut context = Blake2s::new(HASHLEN);
	context.update(con);
	context.update(data);
	let hash = context.finalize();
	from_slice_hashlen(&hash.as_bytes()[..])
}

pub(crate) fn hmac(key: &[u8], data: &[u8], out: &mut [u8]) {
	let mut context = Blake2s::new(HASHLEN);
	let mut ipad = [0x36_u8; BLOCKLEN];
	let mut opad = [0x5c_u8; BLOCKLEN];
	for count in 0..key.len() {
		ipad[count] ^= key[count];
		opad[count] ^= key[count];
	}
	context.update(&ipad[..BLOCKLEN]);
	context.update(data);
	let inner_output = context.finalize();
	context = Blake2s::new(HASHLEN);
	context.update(&opad[..BLOCKLEN]);
	context.update(&inner_output.as_bytes()[..HASHLEN]);
	out.copy_from_slice(context.finalize().as_bytes());
}

pub(crate) fn hkdf(
	chaining_key: &[u8], input_key_material: &[u8], outputs: usize, out1: &mut [u8],
	out2: &mut [u8], out3: &mut [u8],
) {
	let mut temp_key = EMPTY_HASH;
	hmac(chaining_key, input_key_material, &mut temp_key);
	hmac(&temp_key, &[1_u8], out1);
	if outputs == 1 {
		return;
	}
	let mut in2 = [0_u8; HASHLEN + 1];
	copy_slices!(&out1[0..HASHLEN], &mut in2);
	in2[HASHLEN] = 2;
	hmac(&temp_key, &in2[..=HASHLEN], out2);
	if outputs == 2 {
		return;
	}
	let mut in3 = [0_u8; HASHLEN + 1];
	copy_slices!(&out2[0..HASHLEN], &mut in3);
	in3[HASHLEN] = 3;
	hmac(&temp_key, &in3[..=HASHLEN], out3);
}
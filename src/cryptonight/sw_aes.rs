
use u64x2::u64x2;
use cryptonight::sse;

pub fn gen_round_keys(input0: u64x2, input1: u64x2) -> [u64x2;10] {

/*
    let mut k : [u8;16] = [0;16];
    input0.write(&mut k);
    let aes = aesti::Aes::with_key(&mut k).unwrap();
    println!("key_enc={:?}", aes.key_enc[0]);
*/
    let r : [u64x2;10] = [u64x2(0,0);10];
    return r;
}

pub fn aes_round(block: u64x2, key: u64x2) -> u64x2 {
    return u64x2(0, 0);
}

fn transmute_u8(u: u32) -> [u8; 4] {
    unsafe { ::std::mem::transmute(u) }
}

fn transmute_u32(u: [u8;4]) -> u32 {
    unsafe { ::std::mem::transmute(u) }
}

pub fn sub_word(w: u32) -> u32 {
    let a = transmute_u8(w);
    let r = [sbox(a[0]), sbox(a[1]), sbox(a[2]), sbox(a[3])];
    return transmute_u32(r);
}

pub fn rotr(value: u32, amount: u32) -> u32 {
    return u32::rotate_right(value, amount);
}

//TODO Test for sl_xor
//TODO impl _mm_xor_si128
pub fn sl_xor(value: u64x2) -> u64x2 {

    /*
    /*printf("x-or-input: ");
	printm128i(tmp1);*/
	__m128i tmp4;
	tmp4 = _mm_slli_si128(tmp1, 0x04);
	tmp1 = _mm_xor_si128(tmp1, tmp4);
	/*printf("xor-1: ");
	printm128i(tmp1);*/
	tmp4 = _mm_slli_si128(tmp4, 0x04);
	tmp1 = _mm_xor_si128(tmp1, tmp4);
	/*printf("xor-2: ");
	printm128i(tmp1);*/
	tmp4 = _mm_slli_si128(tmp4, 0x04);
	tmp1 = _mm_xor_si128(tmp1, tmp4);
	/*printf("xor-3: ");
	printm128i(tmp1);*/
	return tmp1;
    */

    return u64x2(0, 0);
}

pub fn aes_keygenassist(key: u64x2, rcon: u8) -> u64x2 {
    let w1 = sub_word(sse::_mm_cvtsi128_si32(sse::_mm_shuffle_epi32_0x55(key)));
    let w2 = sub_word(sse::_mm_cvtsi128_si32(sse::_mm_shuffle_epi32_0xff(key)));

    let p2 = ((rotr(w2, 8) ^ rcon as u32) as u64) << 32 | w2 as u64;
    let p1 = ((rotr(w1, 8) ^ rcon as u32) as u64) << 32 | w1 as u64;
    return u64x2(p1, p2);
}

pub fn aes_keygenassist_sub(key0: u64x2, key1: u64x2, rcon: u8) -> (u64x2, u64x2) {
    let mut out1 = aes_keygenassist(key1, rcon);
    out1 = sse::_mm_shuffle_epi32_0xff(out1);

    let mut out2 = aes_keygenassist(key1, 0x00);
    out2 = sse::_mm_shuffle_epi32_0xaa(out2);

    return (out1, out2);
/*
    __m128i xout1 = soft_aeskeygenassist(*xout2, rcon);
	xout1 = _mm_shuffle_epi32(xout1, 0xFF); // see PSHUFD, set all elems to 4th elem
	*xout0 = sl_xor(*xout0);
	*xout0 = _mm_xor_si128(*xout0, xout1);

	xout1 = soft_aeskeygenassist(*xout0, 0x00);
	xout1 = _mm_shuffle_epi32(xout1, 0xAA); // see PSHUFD, set all elems to 3rd elem
	*xout2 = sl_xor(*xout2);
	*xout2 = _mm_xor_si128(*xout2, xout1);
*/
}

#[inline(always)]
fn sbox(i: u8) -> u8{
    return SBOX[i as usize];
}

// Code taken from https://github.com/jmesmon/aesti

const SBOX: [u8;256] = [
0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5,
0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0,
0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc,
0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a,
0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0,
0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b,
0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85,
0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17,
0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88,
0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c,
0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9,
0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6,
0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e,
0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94,
0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68,
0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

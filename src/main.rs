mod aes_ciy {
    #[derive(PartialEq, Eq)]
    pub struct AESByte {
        val: u8,
    }
    use std::fmt;
    impl fmt::Display for AESByte {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.val)
        }
    }
    impl fmt::Debug for AESByte {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AESByte: 0x{:x}", self.val)
        }
    }
    impl Copy for AESByte { }
    impl Clone for AESByte {
        fn clone(&self) -> AESByte {
            *self
        }
    }
    use std::ops::BitXor;
    impl BitXor for AESByte {
        type Output = Self;
        fn bitxor(self, rhs: Self) -> Self::Output {
            let xor = self.val ^ rhs.val;
            let output = AESByte::new(xor);
            output
        }
    }
    use std::ops::BitXorAssign;
    impl BitXorAssign for AESByte {
        fn bitxor_assign(&mut self, rhs: Self) {
            self.val ^= rhs.val;
        }
    }
    use std::ops::Shl;
    impl Shl<usize> for AESByte {
        type Output = Self;
        fn shl(self, rhs: usize) -> Self::Output {
            let u = self.get() << rhs;
            AESByte::new(u)
        }
    }
    impl AESByte {
        pub fn new(val: u8) -> AESByte {
            AESByte {
                val,
            }
        }
        pub fn get(&self) -> u8 {
            self.val
        }
        pub fn set(& mut self, val: u8) {
            self.val = val;
        }
        pub fn sub_bytes(&mut self) {
            let b = sub_bytes::sub_bytes(self.get());
            self.set(b);
        }
    }

    mod sub_bytes {
        const SBOX: [u8; 256] = [
            0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
            0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
            0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
            0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
            0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
            0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
            0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
            0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
            0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
            0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
            0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
            0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
            0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
            0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
            0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
            0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
        ];
        pub fn sub_bytes(input: u8) -> u8 {
            SBOX[input as usize]
        }
    }

    pub struct AESKey {
        key: [AESByte; 16],
    }
    impl AESKey {
        pub fn new(key: u128) -> AESKey {
            let bytes = key.to_be_bytes();
            let mut key = [AESByte::new(0); 16];
            for (i, kb) in bytes.iter().enumerate() {
                key[i].set(*kb);
            }
            AESKey {
                key,
            }
        }
        pub fn expand(&mut self, round: u8) {
            let tmp = AESKey::gi_function(&self.key[12..16], round);
            for (i, b) in tmp.iter().enumerate() {
                self.key[i] ^= *b;
            }
            // TODO: Iterator::zip for pairing both iterators
            for i in 4..16 {
                self.key[i] ^= self.key[i-4];
            }
        }
        fn gi_function(input: &[AESByte], round: u8) -> [AESByte; 4] {
            let mut out = [AESByte::new(0); 4];
            out[0] = input[1];
            out[1] = input[2];
            out[2] = input[3];
            out[3] = input[0];
            for b in out.iter_mut() {
                b.sub_bytes();
            }
            let rc = AESKey::get_round_const(round);
            out[0] ^= rc;
            out
        }
        fn get_round_const(round: u8) -> AESByte {
            let constant: u8 = match round {
                1 => 0x01,
                2 => 0x02,
                3 => 0x04,
                4 => 0x08,
                5 => 0x10,
                6 => 0x20,
                7 => 0x40,
                8 => 0x80,
                9 => 0x1B,
                10 => 0x36,
                _ => panic!("key round out of range"),
            };
            AESByte::new(constant)
        }
    }
    // TODO: Rename AESData
    // TODO: Same struct for AESBlock and AESKey?
    pub struct AESBlock {
        pub data: [AESByte; 16],
    }
    impl AESBlock {
        pub fn new(plaintext: u128) -> AESBlock {
            let bytes = plaintext.to_be_bytes();
            let mut data = [AESByte::new(0); 16];
            for (i, db) in bytes.iter().enumerate() {
                data[i].set(*db);
            }
            AESBlock {
                data,
            }
        }
        // TODO: define bitxor trait for AESBlock and AESKey; or use same struct
        pub fn add_round_key(&mut self, key: &AESKey) {
            for (i, kb) in key.key.iter().enumerate() {
                self.data[i] ^= *kb;
            }
        }
        pub fn substitute_bytes(&mut self) {
            for b in self.data.iter_mut() {
                b.sub_bytes();
            }
        }
        pub fn shift_rows(&mut self) {
            // first row: shift one column left
            let b10 = self.data[1];
            self.data[1] = self.data[5];
            self.data[5] = self.data[9];
            self.data[9] = self.data[13];
            self.data[13] = b10;
            // second row: shift two columns left
            let b20 = self.data[2];
            let b21 = self.data[6];
            self.data[2] = self.data[10];
            self.data[6] = self.data[14];
            self.data[10] = b20;
            self.data[14] = b21;
            // third row: shift three columns left
            let b30 = self.data[3];
            self.data[3] = self.data[15];
            self.data[15] = self.data[11];
            self.data[11] = self.data[7];
            self.data[7] = b30;
        }
        pub fn mix_columns(&mut self) {
            AESBlock::mix_column(&mut self.data[0..4]);
            AESBlock::mix_column(&mut self.data[4..8]);
            AESBlock::mix_column(&mut self.data[8..12]);
            AESBlock::mix_column(&mut self.data[12..16]);
        }
        fn mix_column(input: &mut [AESByte]) {
            let in_cp = [input[0], input[1], input[2], input[3]];
            input[0] = AESBlock::xtime(in_cp[0]) ^ AESBlock::xtime(in_cp[1]) ^ in_cp[1] ^ in_cp[2] ^ in_cp[3];
            input[1] = in_cp[0] ^ AESBlock::xtime(in_cp[1]) ^ AESBlock::xtime(in_cp[2]) ^ in_cp[2] ^ in_cp[3];
            input[2] = in_cp[0] ^ in_cp[1] ^ AESBlock::xtime(in_cp[2]) ^ AESBlock::xtime(in_cp[3]) ^ in_cp[3];
            input[3] = AESBlock::xtime(in_cp[0]) ^ in_cp[0] ^ in_cp[1] ^ in_cp[2] ^ AESBlock::xtime(in_cp[3]);
        }
        fn xtime(input: AESByte) -> AESByte {
            if input.get() & 0x80 != 0x00 {
                input.shl(1) ^ AESByte::new(0x1b)
            }
            else {
                input.shl(1)
            }
        }
    }
    // TODO: clean up pub / private fields
    pub struct AES {
        key: AESKey,
        pub data: AESBlock,
        round: u8,
        pub cipher: Option<u128>,
    }
    impl AES {
        pub fn new(plaintext: u128, key: u128) -> AES {
            let key = AESKey::new(key);
            let data = AESBlock::new(plaintext);
            let cipher = None;
            let round: u8 = 0;
            AES {
                key,
                data,
                round,
                cipher,
            }
        }
        pub fn encrypt(&mut self) {
            self.first_round();
            for _ in 0..9 {
                self.normal_round();
            }
            self.last_round();
            self.write_cipher();
        }
        fn first_round(&mut self) {
            self.data.add_round_key(&self.key);
        }
        fn normal_round(&mut self) {
            self.data.substitute_bytes();
            self.data.shift_rows();
            self.data.mix_columns();
            self.round += 1;
            self.key.expand(self.round);
            self.data.add_round_key(&self.key);
        }
        fn last_round(&mut self) {
            self.data.substitute_bytes();
            self.data.shift_rows();
            self.round += 1;
            self.key.expand(self.round);
            self.data.add_round_key(&self.key);
        }
        fn write_cipher(&mut self) {
            let mut ciph: u128 = 0x0;
            for (i, aes_b) in self.data.data.iter().enumerate() {
                ciph |= (aes_b.get() as u128) << (15-i)*8;
            }
            self.cipher = Some(ciph);
        }
    }
    // TODO: add more unit tests
    #[cfg(test)]
    mod tests {
        #[test]
        fn encryption_1() {
            let key: u128 = 0x9D5BFF851B0B81F841E7196736524BBD;
            let plaintext: u128 = 0x4F816B7C87A0563D0D84BDE984A33D03;
            let mut aes = super::AES::new(plaintext, key);
            aes.encrypt();

            let cipher = match aes.cipher {
                Some(t) => t,
                None => panic!("houston, we fucked up!"),
            };
            println!("Cipher: {:x}", &cipher);

            const CORR_CIPH: u128 = 0xBACF80FA05DF776E90CBF0E7D13335B4;
            assert_eq!(cipher, CORR_CIPH, "wrong cipher computed!");
            println!("Cipher is correct!");
        }
        #[test]
        fn xtime_1() {
            let input = super::AESByte::new(0x7F);
            let result = super::AESBlock::xtime(input);
            let correct = super::AESByte::new(0xFE);
            assert_eq!(result, correct, "xtime computed wrong result; MSB not set");
        }
        #[test]
        fn xtime_2() {
            let input = super::AESByte::new(0xFF);
            let result = super::AESBlock::xtime(input);
            let correct = super::AESByte::new(0xE5);
            assert_eq!(result, correct, "xtime computed wrong result; MSB set");
        }
        #[test]
        fn mix_column_1() {
            use super::AESByte;
            use super::AESBlock;

            let mut input = [
                AESByte::new(0xdb),
                AESByte::new(0x13),
                AESByte::new(0x53),
                AESByte::new(0x45),
            ];
            let corr = [
                AESByte::new(0x8e),
                AESByte::new(0x4d),
                AESByte::new(0xa1),
                AESByte::new(0xbc),
            ];

            AESBlock::mix_column(&mut input);
            assert_eq!(input, corr, "mix_column computed wrong result!");
        }
        #[test]
        fn mix_column_2() {
            use super::AESByte;
            use super::AESBlock;

            let mut input = [
                AESByte::new(0xf2),
                AESByte::new(0x0a),
                AESByte::new(0x22),
                AESByte::new(0x5c),
            ];
            let corr = [
                AESByte::new(0x9f),
                AESByte::new(0xdc),
                AESByte::new(0x58),
                AESByte::new(0x9d),
            ];

            AESBlock::mix_column(&mut input);
            assert_eq!(input, corr, "mix_column computed wrong result!");
        }
    }
}

use aes_ciy::AES;

fn main() {
    println!("AES CIY (Code It Yourself)");

    let key: u128 = 0x9D5BFF851B0B81F841E7196736524BBD;
    let plaintext: u128 = 0x4F816B7C87A0563D0D84BDE984A33D03;
    let mut aes = AES::new(plaintext, key);
    aes.encrypt();

    if let Some(cipher) = aes.cipher {
        println!("Cipher: {:x}", &cipher);
    } else {
        println!("No cipher!");
    }
}

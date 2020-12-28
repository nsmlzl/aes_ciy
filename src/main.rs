
mod aes_ciy {
    pub struct AESByte {
        val: u8,
    }
    // TODO: traits needed:
    // [ ] xor
    use std::fmt;
    impl fmt::Display for AESByte {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.val)
        }
    }
    impl Copy for AESByte { }
    impl Clone for AESByte {
        fn clone(&self) -> AESByte {
            *self
        }
    }
    impl AESByte {
        pub fn new() -> AESByte {
            AESByte {
                val: 0,
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
        pub fn xor (&mut self, input: &AESByte) {
            let b = self.get() ^ input.get();
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
        round: u8,
    }
    impl AESKey {
        pub fn new(key: u128) -> AESKey {
            let bytes = key.to_be_bytes();
            let mut key = [AESByte::new(); 16];
            for (i, kb) in bytes.iter().enumerate() {
                key[i].set(*kb);
            }
            let round: u8 = 0;
            AESKey {
                key,
                round,
            }
        }
        pub fn expand(&mut self) {
            self.round += 1;
            let tmp = AESKey::gi(&self.key[12..16], self.round);
            for (i, b) in tmp.iter().enumerate() {
                self.key[i].xor(b);
            }
            for i in 4..16 {
                let mut t = AESByte::new();
                t.set(self.key[i-4].get());
                self.key[i].xor(&t);
            }
        }
        fn gi(input: &[AESByte], round: u8) -> [AESByte; 4] {
            let mut tmp = [AESByte::new(),  AESByte::new(), AESByte::new(), AESByte::new()];
            tmp[0].set(input[1].get());
            tmp[1].set(input[2].get());
            tmp[2].set(input[3].get());
            tmp[3].set(input[0].get());
            for b in tmp.iter_mut() {
                b.sub_bytes();
            }
            let mut rc = AESByte::new();
            let u: u8 = match round {
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
            rc.set(u);
            tmp[0].xor(&rc);
            tmp
        }
    }
    pub struct AESBlock {
        pub data: [AESByte; 16],
    }
    impl AESBlock {
        pub fn new(plaintext: u128) -> AESBlock {
            let bytes = plaintext.to_be_bytes();
            let mut data = [AESByte::new(); 16];
            for (i, db) in bytes.iter().enumerate() {
                data[i].set(*db);
            }
            AESBlock {
                data,
            }
        }
        pub fn add_round_key(&mut self, key: &AESKey) {
            for (i, kb) in key.key.iter().enumerate() {
                self.data[i].xor(kb);
            }
        }
        pub fn substitute_bytes(&mut self) {
            for b in self.data.iter_mut() {
                b.sub_bytes();
            }
        }
        pub fn shift_rows(&mut self) {
            // first row: shift one column left
            let b10 = self.data[1].get();
            self.data[1].set(self.data[5].get());
            self.data[5].set(self.data[9].get());
            self.data[9].set(self.data[13].get());
            self.data[13].set(b10);
            // second row: shift two columns left
            let b20 = self.data[2].get();
            let b21 = self.data[6].get();
            self.data[2].set(self.data[10].get());
            self.data[6].set(self.data[14].get());
            self.data[10].set(b20);
            self.data[14].set(b21);
            // third row: shift three columns left
            let b30 = self.data[3].get();
            let b31 = self.data[7].get();
            let b32 = self.data[11].get();
            self.data[3].set(self.data[15].get());
            self.data[7].set(b30);
            self.data[11].set(b31);
            self.data[15].set(b32);
        }
        pub fn mix_columns(&mut self) {
            AESBlock::mix_column(&mut self.data[0..4]);
            AESBlock::mix_column(&mut self.data[4..8]);
            AESBlock::mix_column(&mut self.data[8..12]);
            AESBlock::mix_column(&mut self.data[12..16]);
        }
        fn mix_column(input: &mut [AESByte]) {
            let i0 = input[0].get();
            let i1 = input[1].get();
            let i2 = input[2].get();
            let i3 = input[3].get();
            input[0].set(AESBlock::xtime(i0) ^ AESBlock::xtime(i1) ^ i1 ^ i2 ^i3);
            input[1].set(i0 ^ AESBlock::xtime(i1) ^ AESBlock::xtime(i2) ^ i2 ^i3);
            input[2].set(i0 ^ i1 ^ AESBlock::xtime(i2) ^ AESBlock::xtime(i3) ^ i3);
            input[3].set(AESBlock::xtime(i0) ^ i0 ^ i1 ^ i2 ^ AESBlock::xtime(i3));
        }
        fn xtime(input: u8) -> u8 {
            if input & 0x80 != 0x00 {
                input << 1 ^ 0x1b
            }
            else {
                input << 1
            }
        }
    }
    pub struct AES {
        key: AESKey,
        pub data: AESBlock,
    }
    impl AES {
        pub fn new(plaintext: u128, key: u128) -> AES {
            let key = AESKey::new(key);
            let data = AESBlock::new(plaintext);
            AES {
                key,
                data,
            }
        }
        pub fn encrypt(&mut self) {
            self.first_round();
            for _ in 0..9 {
                self.normal_round();
            }
            self.last_round();
        }
        fn first_round(&mut self) {
            self.data.add_round_key(&self.key);
        }
        fn normal_round(&mut self) {
            self.data.substitute_bytes();
            self.data.shift_rows();
            self.data.mix_columns();
            self.key.expand();
            self.data.add_round_key(&self.key);
        }
        fn last_round(&mut self) {
            self.data.substitute_bytes();
            self.data.shift_rows();
            self.key.expand();
            self.data.add_round_key(&self.key);

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

    for (i, d) in aes.data.data.iter().enumerate() {
        println!("{}: 0x{:x}", i, d.get());
    }
}

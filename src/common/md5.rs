pub struct MD5;

impl MD5 {
    const STATE_INIT: [u32; 4] = [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476];

    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, // round 1
        5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, // round 2
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, // round 3
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, // round 4
    ];

    const K: [u32; 64] = [
        // round 1
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, // round 2
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681,
        0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8,
        0x676f02d9, 0x8d2a4c8a, // round 3
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60,
        0xbebfbc70, 0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5,
        0x1fa27cf8, 0xc4ac5665, // round 4
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d,
        0x85845dd1, 0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235,
        0x2ad7d2bb, 0xeb86d391,
    ];

    fn str_to_bytes(s: &str) -> Vec<u8> {
        s.bytes().collect()
    }

    fn pad(mut message: Vec<u8>) -> Vec<u8> {
        let initial_len = (message.len() as u64) * 8;
        message.push(0x80);
        let k = (56 - (message.len() % 64) + 64) % 64;
        message.extend_from_slice(&vec![0; k]);
        message.extend_from_slice(&initial_len.to_le_bytes());
        message
    }

    fn chunks(message: &[u8]) -> Vec<[u32; 16]> {
        message
            .chunks_exact(64)
            .map(|chunk| {
                let mut words = [0; 16];
                for (i, chunk) in chunk.chunks_exact(4).enumerate() {
                    words[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                }
                words
            })
            .collect()
    }

    fn md5(message: &[u8]) -> [u32; 4] {
        let mut state = Self::STATE_INIT;
        for chunk in Self::chunks(message) {
            let mut a = state[0];
            let mut b = state[1];
            let mut c = state[2];
            let mut d = state[3];
            for i in 0..64 {
                let (mut f, g) = match i {
                    0..=15 => ((b & c) | (!b & d), i),
                    16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                    32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                    48..=63 => (c ^ (b | !d), (7 * i) % 16),
                    _ => unreachable!(),
                };
                f = f
                    .wrapping_add(a)
                    .wrapping_add(Self::K[i])
                    .wrapping_add(chunk[g]);
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(f.rotate_left(Self::S[i]));
            }
            state[0] = state[0].wrapping_add(a);
            state[1] = state[1].wrapping_add(b);
            state[2] = state[2].wrapping_add(c);
            state[3] = state[3].wrapping_add(d);
        }
        state
    }

    pub fn hash(s: &str) -> u128 {
        let bytes = Self::str_to_bytes(s);
        let padded = Self::pad(bytes);
        let result = Self::md5(&padded);

        u128::from_be_bytes(
            result
                .iter()
                .flat_map(|x| x.to_le_bytes())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_empty() {
        assert_eq!(MD5::hash(""), 0xd41d8cd98f00b204e9800998ecf8427e);
    }

    #[test]
    fn test_hash_abc() {
        assert_eq!(MD5::hash("abc"), 0x900150983cd24fb0d6963f7d28e17f72);
    }

    #[test]
    fn test_hash_123() {
        assert_eq!(MD5::hash("123"), 0x202cb962ac59075b964b07152d234b70);
    }
}

use aoc_common::Options;


fn md5(input: &[u8]) -> String {
    let s: [u32; 64] = [ 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17,
    22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23,
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15, 21, 6, 10, 15, 21,
    6, 10, 15, 21, 6, 10, 15, 21 ];

    let k: [u32; 64] = [ 0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501, 0x698098d8, 0x8b44f7af,
    0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453,
    0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681,
    0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 
    0x1fa27cf8, 0xc4ac5665, 0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1, 0x6fa87e4f, 0xfe2ce6e0,
    0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391 ];

    let mut a0: u32 = 0x67452301; // A
    let mut b0: u32 = 0xefcdab89; // B
    let mut c0: u32 = 0x98badcfe; // C
    let mut d0: u32 = 0x10325476; // D

    let mut original_bit_len: Vec<u8> = (input.len() as u64)
    .wrapping_mul(8)
    .to_le_bytes()
    .to_vec();

    let mut data: Vec<u8> = input.to_vec();
    data.push(0x80); // Append a 1
    // Append 0s until we're 8 bytes short of the next 64.
    while data.len().rem_euclid(64) != 56 {
        data.push(0x00);
    }
    data.append(&mut original_bit_len);

    for chunk in data.chunks_exact(64) {
        let m: [u32; 16] = std::array::from_fn(|i| {
            let slice = &chunk[i * 4..i*4 + 4];
            u32::from_le_bytes(slice.try_into().unwrap())
        });

        let mut a: u32 = a0;
        let mut b: u32 = b0;
        let mut c: u32 = c0;
        let mut d: u32 = d0;

        for i in 0_u32..64_u32 {
            let (mut f, g) = if i <= 15 {
                ((b & c) | ((!b) & d),
                i)
            } else if i >= 16 && i <= 31 {
                ((d & b) | ((!d) & c),
                (5*i + 1).rem_euclid(16))
            } else if i >= 32 && i <= 47 {
                (b ^ c ^ d,
                (3*i + 5).rem_euclid(16))
            } else {
                (c ^ (b | (!d)),
                (7*i).rem_euclid(16))
            };
            f = f.wrapping_add(a)
                .wrapping_add(k[i as usize])
                .wrapping_add(m[g as usize]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(s[i as usize]));
        }

        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let a0_str = a0
        .to_le_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    let b0_str = b0
        .to_le_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    let c0_str = c0
        .to_le_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    let d0_str = d0
        .to_le_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    
    format!("{}{}{}{}", a0_str, b0_str, c0_str, d0_str)
}


/// Generate an 8 character password by using the 6th character of an MD5 digest
/// that begins with 5 0s, which is calculated using an input with an 
/// incrementing digit appended to it.
pub fn part1(input: &str, _opts: &Options) -> Result<String, String> {

    let mut password = String::new();

    let mut i = 0_usize;
    while password.len() < 8 {
        let attempt = format!("{}{}", input, i);
        let digest = md5(attempt.as_bytes());
        if digest.starts_with("00000") {
            password.push(digest.chars().nth(5).unwrap_or('_'));
        }
        i += 1;
    }
    
    Ok(password)
}


/// Generate an 8 character password by using the 6th character of an MD5 digest
/// that begins with 5 0s as an index to place the 7th character at, which is
/// calculated using an input with an incrementing digit appended to it.
/// Note: Do not overwrite characters that have already been assigned.
pub fn part2(input: &str, _opts: &Options) -> Result<String, String> {

    // Prefill vec with _ to indicate unassigned indicies
    let mut password = vec!['_'; 8];

    let mut i = 0_usize;
    // Continue until all 8 characters have been assigned.
    while password.iter().any(|c| *c == '_') {
        let attempt = format!("{}{}", input, i);
        let digest = md5(attempt.as_bytes());
        if digest.starts_with("00000") {
            let idx = digest
            .chars()
            .nth(5)
            .unwrap()
            .to_digit(16)
            .unwrap() as usize;
            if  idx > 7 {
                i += 1;
                continue;
            }
            if password[idx] == '_' {
                let ch = digest.chars().nth(6).unwrap();
                password[idx] = ch;
            }
        }
        i += 1;
    }

    let password: String = password.iter().collect();
    
    Ok(password)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE101: &str = "abc";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(EXAMPLE101, &Options{ debug: false, iterations: Some(8) })
                .unwrap(),
            "18f47a30"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part1(EXAMPLE101, &Options{ debug: false, iterations: Some(8) })
                .unwrap(),
            "05ace8e3"
        );
    }
}
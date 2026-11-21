use aoc_common::Options;
use std::{
    collections::HashMap,
    cmp::Ordering
};


#[derive(Debug)]
struct Room {
    encrypted_name: String,
    sector_id: usize,
    checksum: String
}
impl Room {
    /// Parse a Room from a string. Lazy - will just ignore any errors
    fn from_str(input: &str) -> Self {
        let mut encrypted_name = String::new();
        let mut sector_id = String::new();
        let mut checksum = String::new();
        let mut parsing_checksum = false;
        for ch in input.chars() {
            match ch {
                'a'..='z' | '-' => {
                    if parsing_checksum && ch != '-' {
                        checksum.push(ch);
                    } else {
                        encrypted_name.push(ch);
                    }
                },
                '0'..='9' => {
                    sector_id.push(ch);
                },
                '[' => {
                    parsing_checksum = true;
                },
                _ => { continue; }
            }
        }

        if encrypted_name.ends_with('-') {
            encrypted_name.pop();
        }
        let sector_id = sector_id.parse::<usize>().unwrap_or(0);

        Room {
            encrypted_name,
            sector_id,
            checksum,
        }
    }

    /// Check if a room is real by comparing the checksums
    fn is_real(&self) -> bool {
        let computed_sum = self.compute_checksum();
        
        computed_sum == self.checksum
    }

    /// Compute the checksum of a room by counting the encrypted_name char freq
    /// And then sorting them - first in reverse by their frequency (most common
    /// first) and then normally by the char itself if the counts are equal
    fn compute_checksum(&self) -> String {
        let mut ch_count: HashMap<char, usize> = HashMap::with_capacity(self.encrypted_name.len());
        for ch in self.encrypted_name.chars() {
            if ch == '-' {
                continue;
            }
            let mut count = 1 as usize;
            if ch_count.contains_key(&ch) {
                count += ch_count.get(&ch).unwrap();
            }
            ch_count.insert(ch, count);
        }
        let mut ch_count: Vec<(usize, char)> = ch_count
        .iter()
        .map(|(ch, count)| (count.to_owned(), ch.to_owned()))
        .collect();
        ch_count.sort_by(
            |a, b|
            match b.0.cmp(&a.0) {
                Ordering::Equal => {
                    a.1.cmp(&b.1)
                },
                o => o
            }
        );
        
        ch_count[0..5].iter().map(|(_, ch)| ch).collect::<String>()
    }

    /// Decrypt the encrypted name 
    fn decrypt(&self) -> String {
        self.encrypted_name
        .chars()
        .map(|c| {
            if c == '-' {
                ' '
            } else {
                let base = 'a' as u8;
                let offset = c as u8 - base;
                ((offset as usize + self.sector_id).rem_euclid(26) as u8 + base) as char
            }
        })
        .collect()
    }
}


pub fn part1(input: &str, opts: &Options) -> Result<String, String> {

    let mut sum: usize = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let room = Room::from_str(line);
        if opts.debug { eprintln!("Parsed {:?} - Computed Checksum: {}", room, room.compute_checksum()); }
        if room.is_real() {
            sum = sum.wrapping_add(room.sector_id);
            if opts.debug { eprintln!("Sum: {}", sum)}
        }
    }

    Ok(sum.to_string())
}


pub fn part2(input: &str, opts: &Options) -> Result<String, String> {
    let mut sector_id = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let room = Room::from_str(line);
        if opts.debug { 
            eprintln!(
                "Parsed {:?} - Computed Checksum: {}",
                room,
                room.compute_checksum()
            );
        }
        if room.is_real() {
            let decrypted_name = room.decrypt();
            if opts.debug { eprintln!("Decrypted {}", decrypted_name); }
            if decrypted_name.trim().eq_ignore_ascii_case("northpole object storage") {
                sector_id = room.sector_id;
            }
        }
    }

    Ok(sector_id.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE101: &str = "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]";
    const EXAMPLE201: &str = "qzmt-zixmtkozy-ivhz-343";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(EXAMPLE101, &Options{ debug: false, iterations: None }).unwrap(),
            "1514"
        );
    }

    #[test]
    fn part2_example() {
        let room = Room::from_str(EXAMPLE201);
        assert_eq!(
            room.decrypt(),
            "very encrypted name"
        );
    }
}
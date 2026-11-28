use aoc_common::Options;
use std::collections::HashSet;


struct IPv7 {
    address: String
}
impl IPv7 {
    fn supports_tls(&self, opts: &Options) -> bool {
        let mut square_brackets = false;
        let mut supports_tls = false;

        //if opts.debug { eprintln!("{}", &self.address); }

        for i in 0..self.address.chars().count() {
            //if opts.debug {
            //    eprintln!(
            //        "At index {} - {}",
            //        i,
            //        &self.address
            //            .chars()
            //            .nth(i)
            //            .unwrap()
            //    );
            //}
            
            // We need at least 4 chars remaining to make a pattern
            let Some(remaining_chars) = self.address
                .chars()
                .count()
                .checked_sub(i)
            else {
                continue;
            };
            if remaining_chars < 4 {
                continue;
            }

            if self.address.chars().nth(i).unwrap() == '[' {
                square_brackets = true;
                continue;
            } else if self.address.chars().nth(i).unwrap() == ']' {
                square_brackets = false;
                continue;
            }

            let chs: Vec<char> = self.address.chars().skip(i).take(4).collect();

            // Must match abba - where a and b are different
            if chs[0] != chs[1] && chs[0] == chs [3] && chs[1] == chs [2] {
                if opts.debug { eprintln!("Match! {:?} - In[]={}", chs, square_brackets); }
                if square_brackets {
                    return false
                }
                supports_tls = true;
            }
        }

        supports_tls
    }

    fn supports_ssl(&self) -> bool {
        // Save a list of all discovered ABA supernet (outside square brackets)
        // and the BAB hypernets (inside the square brackets) and then check the
        // list of ABA againbst the BAB
        let mut supernet_list: HashSet<String> = HashSet::new();
        let mut hypernet_list: HashSet<String> = HashSet::new();
        let mut square_brackets = false;

        for i in 0..self.address.chars().count() {
            // We need at least 3 chars remaining to make a pattern
            let Some(remaining_chars) = self.address
                .chars()
                .count()
                .checked_sub(i)
            else {
                continue;
            };
            if remaining_chars < 3 {
                continue;
            }

            if self.address.chars().nth(i).unwrap() == '[' {
                square_brackets = true;
                continue;
            } else if self.address.chars().nth(i).unwrap() == ']' {
                square_brackets = false;
                continue;
            }

            let chs: Vec<char> = self.address.chars().skip(i).take(3).collect();

            // Must match aba - where a and b are different
            if chs[0] != chs[1] && chs[0] == chs [2] {
                if square_brackets {
                    hypernet_list.insert(chs.iter().collect::<String>());
                    continue;
                }
                supernet_list.insert(chs.iter().collect::<String>());
            }
        }

        for aba in supernet_list {
            let bab = [
                aba.chars().nth(1).unwrap(),
                aba.chars().nth(0).unwrap(),
                aba.chars().nth(1).unwrap()
            ].to_vec().iter().collect::<String>();
            if hypernet_list.contains(&bab) {
                return true;
            }
        }

        false
    }
}


pub fn part1(input: &str, opts: &Options) -> Result<String, String> {

    let mut count: usize = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let ipv7_address = IPv7{ address: line.to_string()};
        if ipv7_address.supports_tls(opts) {
            count += 1;
        }
    }
    

    Ok(count.to_string())
}


pub fn part2(input: &str, _opts: &Options) -> Result<String, String> {
    
    let mut count: usize = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let ipv7_address = IPv7{ address: line.to_string()};
        if ipv7_address.supports_ssl() {
            count += 1;
        }
    }
    

    Ok(count.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE101: &str = "abba[mnop]qrst\nabcd[bddb]xyyx\naaaa[qwer]tyui\nioxxoj[asdfgh]zxcvbn\n";
    const EXAMPLE201: &str = "aba[bab]xyz\nxyx[xyx]xyx\naaa[kek]eke\nzazbz[bzb]cdb";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(EXAMPLE101, &Options::default())
                .unwrap(),
            "2"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(EXAMPLE201, &Options::default())
                .unwrap(),
            "3"
        );
    }
}
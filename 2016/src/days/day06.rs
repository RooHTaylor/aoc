use aoc_common::Options;
use std::collections::HashMap;


pub fn part1(input: &str, opts: &Options) -> Result<String, String> {
    
    let number_cols: usize = input.lines().nth(0).unwrap().len();
    let mut char_frequencies: Vec<HashMap<char, usize>> = vec![
        HashMap::new();
        number_cols
    ];

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if opts.debug { eprintln!("Line: {}", line); }
        for i in 0_usize..number_cols {
            if opts.debug { eprintln!("Col {} - {}", i, line.chars().nth(i).unwrap()); }
            char_frequencies[i]
                .entry(line.chars().nth(i).unwrap())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            if opts.debug { eprintln!("Col {} - {:?}", i, char_frequencies); }
        }
    }

    let message: String = char_frequencies
        .iter()
        .map(|cf| {
            cf
                .iter()
                .max_by_key(|&(_, count)| count)
                .map(|(&ch, _ )| ch)
                .unwrap()
        })
        .collect();

    Ok(message.to_string())
}


pub fn part2(input: &str, opts: &Options) -> Result<String, String> {
    let number_cols: usize = input.lines().nth(0).unwrap().len();
    let mut char_frequencies: Vec<HashMap<char, usize>> = vec![
        HashMap::new();
        number_cols
    ];

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if opts.debug { eprintln!("Line: {}", line); }
        for i in 0_usize..number_cols {
            if opts.debug { eprintln!("Col {} - {}", i, line.chars().nth(i).unwrap()); }
            char_frequencies[i]
                .entry(line.chars().nth(i).unwrap())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            if opts.debug { eprintln!("Col {} - {:?}", i, char_frequencies); }
        }
    }

    let message: String = char_frequencies
        .iter()
        .map(|cf| {
            cf
                .iter()
                .min_by_key(|&(_, count)| count)
                .map(|(&ch, _ )| ch)
                .unwrap()
        })
        .collect();

    Ok(message.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE101: &str = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar\n";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(EXAMPLE101, &Options{ debug: false, iterations: None })
                .unwrap(),
            "easter"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part1(EXAMPLE101, &Options{ debug: false, iterations: None })
                .unwrap(),
            "advent"
        );
    }
}
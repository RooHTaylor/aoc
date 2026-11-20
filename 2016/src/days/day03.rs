use aoc_common::Options;


#[derive(Debug, Clone)]
struct Triangle {
    x1: usize,
    x2: usize,
    x3: usize
}
impl Triangle {
    fn is_valid(&self) -> bool {
        self.x1.wrapping_add(self.x2) > self.x3 && 
        self.x1.wrapping_add(self.x3) > self.x2 &&
        self.x2.wrapping_add(self.x3) > self.x1
    }

    fn set_index(&mut self, idx: usize, value: usize) {
        match idx {
            0 => {
                self.x1 = value;
            },
            1 => {
                self.x2 = value;
            },
            _ => {
                self.x3 = value;
            }
        }
    }
}


fn parse_triangles(input: &str) -> Result<Vec<Triangle>, String> {
    let mut triangles = vec![];

    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }
        let side_lengths: Vec<usize> = line
        .split_whitespace()
        .map(|n| {
            n
            .parse::<usize>()
            .map_err(|_e| format!("Invalid triangle side length! {}", n))
        })
        .collect::<Result<Vec<usize>, String>>()?;

        if side_lengths.len() != 3 {
            return Err(format!("Invalid triangle! {}", line))
        }
        triangles.push(Triangle {
            x1: side_lengths[0],
            x2: side_lengths[1],
            x3: side_lengths[2]
        });
    }

    Ok(triangles)
}


pub fn part1(input: &str, _opts: &Options) -> Result<String, String> {
    let triangles = parse_triangles(input)?;
    let mut valid_count: usize = 0;

    for triangle in triangles {
        if !triangle.is_valid() {
            continue;
        }
        valid_count += 1;
    }

    Ok(valid_count.to_string())
}


fn parse_triangles_pt2(input: &str) -> Result<Vec<Triangle>, String> {
    let mut triangles = vec![];

    let mut ta = Triangle {x1: 0, x2: 0, x3: 0};
    let mut tb = Triangle {x1: 0, x2: 0, x3: 0};
    let mut tc = Triangle {x1: 0, x2: 0, x3: 0};
    let mut i = 0 as usize;
    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        let side_lengths: Vec<usize> = line
        .split_whitespace()
        .map(|n| {
            n
            .parse::<usize>()
            .map_err(|_e| format!("Invalid triangle side length! {}", n))
        })
        .collect::<Result<Vec<usize>, String>>()?;

        if side_lengths.len() != 3 {
            return Err(format!("Invalid line! {}", line))
        }

        ta.set_index(i, side_lengths[0]);
        tb.set_index(i, side_lengths[1]);
        tc.set_index(i, side_lengths[2]);

        if i == 2 {
            i = 0;
            triangles.push(ta.clone());
            triangles.push(tb.clone());
            triangles.push(tc.clone());
            continue;
        }

        i += 1;
    }

    Ok(triangles)
}


pub fn part2(input: &str, _opts: &Options) -> Result<String, String> {
    let triangles = parse_triangles_pt2(input)?;
    let mut valid_count: usize = 0;

    for triangle in triangles {
        if !triangle.is_valid() {
            continue;
        }
        valid_count += 1;
    }

    Ok(valid_count.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE101: &str = "5 10 25";
    const EXAMPLE201: &str = "101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 602\n203 403 603\n";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(EXAMPLE101, &Options{ debug: false, iterations: None }).unwrap(),
            "0"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(EXAMPLE201, &Options{ debug: false, iterations: None }).unwrap(),
            "6"
        );
    }
}
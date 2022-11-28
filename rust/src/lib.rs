use std::io::{Write, Read, Error};
use std::fmt::{Display, Formatter};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printing_in_new_lines_test() {
        let vec = NumVec(vec![0,1,2]);
        assert_eq!(format!("{}", vec), "0\n1\n2");
    }

    #[test]
    fn test_of_single_series_001_test() {
        let res = compute_max_inequality(3, "001");
        assert_eq!(res, 1);
    }

    #[test]
    fn test_of_single_series_1100_test() {
        let res = compute_max_inequality(4, "1100");
        assert_eq!(res, 2);
    }

    #[test]
    fn test_of_single_series_11111_test() {
        let res = compute_max_inequality(5, "11111");
        assert_eq!(res, 0);
    }


    #[test]
    fn string_new_test() {
        let s1 = String::new();
        assert_eq!(s1, "".to_string());
    }

    #[test]
    fn max_inequality_001_test() {
        let mut out: Vec<u8> = Vec::new();

        max_inequality(&mut "1
3
001".as_bytes(), &mut out).unwrap();

        assert_eq!(out, b"1\n");
    }
}

fn compute_max_inequality(_length: u32, series: &str) -> u32 {
    let mut s1:String = String::new();
    let mut s2:String = String::new();
    let mut changes = 0u32;

    for c in series.chars() {
        let v = c.to_string();
        if s1 == "" {
            s1 = v;
        } else if s2 == "" && s1 == v {
            s2 = v
        } else if s1 != v {
            s1 = v;
            changes+=1;
        } else if s2 != "" && s2 != v {
            s2 = v;
            changes+=1;
        }
    }

    changes
}

struct NumVec(Vec<u32>);

impl Display for NumVec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let mut out = String::new();

        for num in &self.0[0..self.0.len() - 1] {
            out.push_str(&num.to_string());
            out.push_str("\n");
        }

        out.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", out)
    }
}


pub fn max_inequality(
    input: &mut impl Read,
    output: &mut impl Write,
) -> Result<(), Error> {
    let mut buffer = "".to_string();

    input.read_to_string(&mut buffer)?;
    
    let mut iterator = buffer.split("\n");

    iterator.next();

    let mut results:Vec<u32> = vec![];

    while let Some(length) = iterator.next() {
        let length: u32 = length.parse().unwrap();
        // println!("{:?}", length);
        let series = if let Some(series) = iterator.next() { series } else { "" };
        // println!("{:?}", series);
        results.push(compute_max_inequality(length, series));
    }

    // println!("{:?}", results);

    let out = format!("{}\n", NumVec(results));

    output.write_all(out.as_bytes())?;

    Ok(())
}
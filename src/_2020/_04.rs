// 2020 day 04
use std::collections::HashMap;
use std::iter::IntoIterator;


#[derive(Debug)]
struct Passport<'a> {
    contents: HashMap<&'static str, &'a str>,
}


impl<'a> Passport<'a> {
    const BYR: &'static str = "byr";
    const IYR: &'static str = "iyr";
    const EYR: &'static str = "eyr";
    const HGT: &'static str = "hgt";
    const HCL: &'static str = "hcl";
    const ECL: &'static str = "ecl";
    const PID: &'static str = "pid";
    const CID: &'static str = "cid";

    const ALL: [&'static str; 8] = [
    Self::BYR,
    Self::CID,
    Self::ECL,
    Self::EYR,
    Self::HCL,
    Self::HGT,
    Self::IYR,
    Self::PID,
    ];

    const REQUIRED: [&'static str; 7] = [
    Self::BYR,
    Self::ECL,
    Self::EYR,
    Self::HCL,
    Self::HGT,
    Self::IYR,
    Self::PID,
    ];

    // Check if we are dealing with a valid field label
    fn to_field<'b>(x: &'b str) -> &'static str {
        let all_slice: &[&'static str] = &Self::ALL;
        let i = all_slice.binary_search(&x).unwrap();
        return Self::ALL[i];
    }

    // Check if we have all required fields
    fn has_field(&self, field: &str) -> bool {
        self.contents.contains_key(field)
    }

    pub fn is_valid(&self) -> bool {
        Self::REQUIRED.iter().all(|f| self.has_field(f))
    }

    pub fn from_iter<T>(vals: T) -> Self
    where
        T: IntoIterator<Item = &'a (&'a str, &'a str)>
    {
        let contents = vals.into_iter().map(|(f, v)| (Self::to_field(f), *v)).collect();
        Passport {
            contents
        }
    }

    pub fn from_str(s: &'a str) -> Self {
        let vals = s.split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let mut it = x.split(":");
                return (it.next().unwrap(), it.next().unwrap())
            });

        return Self::from_iter(vals);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_wrong_field() {
        let _p = Passport::from_iter(&[("a", "1990")]);
    }

    #[test]
    fn test() {
        let p = Passport::from_iter(&[("byr", "1990")]);
        println!("{:?}", p);
    }

}

// 2020 day 04
use std::collections::HashMap;


#[derive(Debug)]
struct Passport {
    contents: HashMap<&'static str, String>,
}


impl Passport {
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
    fn to_field(x: &str) -> &'static str {
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

    pub fn from_str(s: &str) -> Self {
        let contents: HashMap<&'static str, String> = s.split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let mut it = x.split(":");
                let field = Self::to_field(it.next().unwrap());
                let value = it.next().unwrap().to_owned();
                return (field, value)
            })
            .collect();

        Passport {
            contents
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // #[should_panic]
    // fn test_wrong_field() {
    //     let _p = Passport::from_iter(&[("a", "1990")]);
    // }
    //
    // #[test]
    // fn test() {
    //     let p = Passport::from_iter(&[("byr", "1990")]);
    //     println!("{:?}", p);
    // }

    #[test]
    #[should_panic]
    fn test_wrong_field() {
        let _p = Passport::from_str("foo:bar");
    }

    #[test]
    fn test() {
        let p = Passport::from_str("byr:1990");
        println!("{:?}", p);
    }

    #[test]
    fn test_valid_passport() {
        let p = Passport::from_str("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm");
        assert_eq!(true, p.is_valid());
    }

    #[test]
    fn test_invalid_passport() {
        let p = Passport::from_str("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929");
        assert_eq!(false, p.is_valid());
    }

    #[test]
    fn test_valid_passport2() {
        let p = Passport::from_str("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm");
        assert_eq!(true, p.is_valid());
    }

}

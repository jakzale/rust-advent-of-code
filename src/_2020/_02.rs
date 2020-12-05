/// Day 02


use nom::{
    IResult,
    Finish,
    character::complete::{
        digit1,
        anychar,
        alpha1,
        space1
    }
};

named!(dash<&str, &str>, tag!("-"));
named!(colon<&str, &str>, tag!(":"));

#[derive(Debug, PartialEq)]
pub struct PasswordEntry {
    from : i32,
    to : i32,
    c: char,
    password: String
}

impl PasswordEntry {

    pub fn parse_line(input: &str) -> IResult<&str, PasswordEntry> {
        let (input, from_str) = digit1(input)?;
        let from = from_str.parse::<i32>().unwrap();

        let (input, _) = dash(input)?;

        let (input, to_str) = digit1(input)?;
        let to = to_str.parse::<i32>().unwrap();

        let (input, _) = space1(input)?;

        let (input, char) = anychar(input)?;

        let (input, _) = colon(input)?;
        let (input, _) = space1(input)?;

        let (input , password_str) = alpha1(input)?;
        let password = String::from(password_str);


        Ok( (input,
            PasswordEntry {
                from,
                to,
                c: char,
                password
            })
        )
    }

    pub fn from_str(input: &str) -> PasswordEntry {
        Self::parse_line(input).finish().unwrap().1
    }

    pub fn is_valid(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if self.c == c {
                count += 1;
            }

        } 

        count >= self.from && count <= self.to
    }

    pub fn is_valid2(&self) -> bool {
        let l = self.password.len();
        let mut a = l >= self.from as usize;
        let mut b = l >= self.to as usize;

        if a {
            a = self.password.chars().nth((self.from - 1) as usize).unwrap() == self.c;
        }

        if b {
            b = self.password.chars().nth((self.to - 1) as usize).unwrap() == self.c;
        }

        a != b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_1() {
        let p = PasswordEntry::from_str("1-3 a: abcde");
        assert_eq!(p, PasswordEntry { from: 1, to: 3, c: 'a', password: String::from("abcde")});
        assert!(p.is_valid());
        assert!(p.is_valid2());
    }

    #[test]
    pub fn test_2() {
        let p = PasswordEntry::from_str("1-3 b: cdefg");
        assert_eq!(p, PasswordEntry { from: 1, to: 3, c: 'b', password: String::from("cdefg")});
        assert!(!p.is_valid());
        assert!(!p.is_valid2());
    }

    #[test]
    pub fn test_3() {
        let p = PasswordEntry::from_str("2-9 c: ccccccccc");
        assert_eq!(p, PasswordEntry { from: 2, to: 9, c: 'c', password: String::from("ccccccccc")});
        assert!(p.is_valid());
        assert!(!p.is_valid2());
    }
}
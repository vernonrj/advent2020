use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

use regex::Regex;

use super::Passport;

/**
 * A passport, validated according to the rules.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedPassport {
    pub birth_year: BirthYear,
    pub issue_year: IssueYear,
    pub expiration_year: ExpirationYear,
    pub height: Height,
    pub hair_color: HairColor,
    pub eye_color: EyeColor,
    pub passport_id: PassportId,
    pub country_id: Option<String>,
}

impl TryFrom<Passport> for ValidatedPassport {
    type Error = Box<dyn Error>;

    fn try_from(value: Passport) -> Result<Self, Self::Error> {
        let validated = ValidatedPassport {
            birth_year: BirthYear::from_str(&value.birth_year)?,
            issue_year: IssueYear::from_str(&value.issue_year)?,
            expiration_year: ExpirationYear::from_str(&value.expiration_year)?,
            height: Height::from_str(&value.height)?,
            hair_color: HairColor::from_str(&value.hair_color)?,
            eye_color: EyeColor::from_str(&value.eye_color)?,
            passport_id: PassportId::from_str(&value.passport_id)?,
            country_id: value.country_id,
        };
        Ok(validated)
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BirthYear(pub u32);

impl FromStr for BirthYear {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u32 = s.parse()?;
        if num < 1920 || num > 2002 {
            return Err(format!("number out of range: `{}`", num).into());
        }
        Ok(BirthYear(num))
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IssueYear(pub u32);

impl FromStr for IssueYear {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u32 = s.parse()?;
        if num < 2010 || num > 2020 {
            return Err(format!("number out of range: `{}`", num).into());
        }
        Ok(IssueYear(num))
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExpirationYear(pub u32);

impl FromStr for ExpirationYear {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u32 = s.parse()?;
        if num < 2020 || num > 2030 {
            return Err(format!("number out of range: `{}`", num).into());
        }
        Ok(ExpirationYear(num))
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Height {
    Centimeters(u32),
    Inches(u32),
}

impl FromStr for Height {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<num>\d+)(?P<unit>cm|in)$").unwrap();
        }
        let cap = match RE.captures(s) {
            Some(c) => c,
            None => return Err(format!("height: failed to parse field `{}`", s).into()),
        };
        let (num, unit) = match (cap.name("num"), cap.name("unit")) {
            (Some(n), Some(u)) => (n.as_str(), u.as_str()),
            (None, _) => return Err(format!("height: failed to find number in `{}`", s).into()),
            (_, None) => return Err(format!("height: failed to find unit in `{}`", s).into()),
        };

        let num: u32 = num.parse()?;
        match unit {
            "cm" if num >= 150 && num <= 193 => Ok(Height::Centimeters(num)),
            "in" if num >= 59 && num <= 76 => Ok(Height::Inches(num)),
            "cm" | "in" => Err(format!("invalid measurement for unit `{}`: `{}`", unit, num).into()),
            bad_unit => Err(format!("invalid unit specified: `{}`", bad_unit).into())
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HairColor(pub String);

impl FromStr for HairColor {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(?P<hex>[0-9a-f]+)$").unwrap();
        }
        let cap = match RE.captures(s) {
            Some(c) => c,
            None => return Err(format!("invalid hair color: `{}`", s).into()),
        };
        let color = match cap.name("hex") {
            Some(c) => c.as_str(),
            None => return Err(format!("failed to capture color").into()),
        };
        Ok(HairColor(color.to_string()))
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Gray),
            "grn" => Ok(EyeColor::Green),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            err => Err(format!("invalid eye color: `{}`", err).into()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassportId(pub String);

impl FromStr for PassportId {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        
        if RE.is_match(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(format!("failed to match Passport ID rules: `{}`", s).into())
        }
    }
}



#[test]
fn test_birth_year_valid() {
    assert_eq!(BirthYear::from_str("2002").unwrap(), BirthYear(2002));
    BirthYear::from_str("2003").unwrap_err();
}

#[test]
fn test_height_valid() {
    assert_eq!(Height::from_str("60in").unwrap(), Height::Inches(60));
    assert_eq!(Height::from_str("190cm").unwrap(), Height::Centimeters(190));
    
    Height::from_str("190in").unwrap_err();
    Height::from_str("190").unwrap_err();
}

#[test]
fn test_hair_color_valid() {
    assert_eq!(HairColor::from_str("#123abc").unwrap(), HairColor("123abc".to_string()));
    HairColor::from_str("#123abz").unwrap_err();
    HairColor::from_str("123abc").unwrap_err();
}

#[test]
fn test_eye_color_valid() {
    assert_eq!(EyeColor::from_str("amb").unwrap(), EyeColor::Amber);
    assert_eq!(EyeColor::from_str("blu").unwrap(), EyeColor::Blue);
    assert_eq!(EyeColor::from_str("brn").unwrap(), EyeColor::Brown);
    assert_eq!(EyeColor::from_str("gry").unwrap(), EyeColor::Gray);
    assert_eq!(EyeColor::from_str("grn").unwrap(), EyeColor::Green);
    assert_eq!(EyeColor::from_str("hzl").unwrap(), EyeColor::Hazel);
    assert_eq!(EyeColor::from_str("oth").unwrap(), EyeColor::Other);

    EyeColor::from_str("wat").unwrap_err();
}

#[test]
fn test_passport_id_valid() {
    assert_eq!(PassportId::from_str("000000001").unwrap(), PassportId("000000001".to_string()));
    PassportId::from_str("0123456789").unwrap_err();
}

#[test]
fn test_validated_passport_invalid() {
    use super::read_batch;
    use std::io::Cursor;
    let data = vec![
        "eyr:1972 cid:100",
        "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        "",
        "iyr:2019",
        "hcl:#602927 eyr:1967 hgt:170cm",
        "ecl:grn pid:012533040 byr:1946",
        "",
        "hcl:dab227 iyr:2012",
        "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        "",
        "hgt:59cm ecl:zzz",
        "eyr:2038 hcl:74454a iyr:2023",
        "pid:3556412378 byr:2007",
    ].join("\n");
    let batch_lines = read_batch(Cursor::new(data)).unwrap();

    let num_valid = batch_lines.into_iter()
        .flat_map(|line| Passport::from_str(&line).ok())
        .flat_map(ValidatedPassport::try_from)
        .count();
    assert_eq!(num_valid, 0);
}

#[test]
fn test_validated_passport_valid() {
    use super::read_batch;
    use std::io::Cursor;
    let data = vec![
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
        "hcl:#623a2f",
        "",
        "eyr:2029 ecl:blu cid:129 byr:1989",
        "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        "",
        "hcl:#888785",
        "hgt:164cm byr:2001 iyr:2015 cid:88",
        "pid:545766238 ecl:hzl",
        "eyr:2022",
        "",
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
    ].join("\n");
    let batch_lines = read_batch(Cursor::new(data)).unwrap();

    let num_valid = batch_lines.into_iter()
        .flat_map(|line| Passport::from_str(&line).ok())
        .map(|p| ValidatedPassport::try_from(p).unwrap())
        .count();
    assert_eq!(num_valid, 4);
}
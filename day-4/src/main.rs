use std::{error::Error, str::FromStr};
use std::fs::File;
use std::io::{Read, BufRead, BufReader};

use clap::{App, Arg};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let matches = App::new("day-4")
        .arg(Arg::with_name("input")
            .required(true)
            .takes_value(true)
            .help("the input file for the day"))
        .get_matches();
    
    let f = File::open(matches.value_of("input").unwrap())?;
    let batch_lines = read_batch(f)?;

    let num_valid = batch_lines.into_iter()
        .flat_map(|line| Passport::from_str(&line).ok())
        .count();
    
    println!("number of valid passports: {}", num_valid);

    Ok(())
}

/**
 * implements a Passport, with optional Country ID
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passport {
    pub birth_year: String,
    pub issue_year: String,
    pub expiration_year: String,
    pub height: String,
    pub hair_color: String,
    pub eye_color: String,
    pub passport_id: String,
    pub country_id: Option<String>,
}

impl FromStr for Passport {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // we start with an empty builder. As we walk the fields included, we fill it out.
        // After we've walked all the fields, we ensure that all required fields are included.
        let mut builder = PassportBuilder::default();
        let tokens: Vec<(&str, &str)> = tokenize(s)?;

        for (field, value) in tokens {
            let field = PassportFields::from_str(field)?;

            match field {
                PassportFields::BirthYear => builder.birth_year = Some(value.to_string()),
                PassportFields::IssueYear => builder.issue_year = Some(value.to_string()),
                PassportFields::ExpirationYear => builder.expiration_year = Some(value.to_string()),
                PassportFields::Height => builder.height = Some(value.to_string()),
                PassportFields::HairColor => builder.hair_color = Some(value.to_string()),
                PassportFields::EyeColor => builder.eye_color = Some(value.to_string()),
                PassportFields::PassportId => builder.passport_id = Some(value.to_string()),
                PassportFields::CountryId => builder.country_id = Some(value.to_string()),
            }
        }

        // Now we check to see if all required fields were included
        builder.build()
    }
}

/**
 * A builder for a Passport object.
 */
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PassportBuilder {
    pub birth_year: Option<String>,
    pub issue_year: Option<String>,
    pub expiration_year: Option<String>,
    pub height: Option<String>,
    pub hair_color: Option<String>,
    pub eye_color: Option<String>,
    pub passport_id: Option<String>,
    pub country_id: Option<String>,
}

impl PassportBuilder {
    /// Build a Passport from this object. If a required field is missing, then an error will be returned.
    pub fn build(self) -> AppResult<Passport> {
        match self {
            Self {
                birth_year: Some(birth_year),
                issue_year: Some(issue_year),
                expiration_year: Some(expiration_year),
                height: Some(height),
                hair_color: Some(hair_color),
                eye_color: Some(eye_color),
                passport_id: Some(passport_id),
                country_id, // country ID is the only optional field
            } => Ok(Passport { birth_year, issue_year, expiration_year, height, hair_color, eye_color, passport_id, country_id }),
            // a required field is missing. Return an error noting which field is missing
            Self {birth_year: None, ..} => Err(format!("Missing Field: {:?}", PassportFields::BirthYear).into()),
            Self {issue_year: None, ..} => Err(format!("Missing Field: {:?}", PassportFields::IssueYear).into()),
            Self {expiration_year: None, ..} => Err(format!("Missing Field: {:?}", PassportFields::ExpirationYear).into()),
            Self {height: None, ..} => Err(format!("Missing Field: {:?}", PassportFields::Height).into()),
            Self {hair_color: None, ..} => Err(format!("Missing Field: {:?}", PassportFields::HairColor).into()),
            Self {eye_color: None, ..} => Err(format!("Missing Field: {:?}", PassportFields::EyeColor).into()),
            Self {passport_id: None, ..} => Err(format!("Missing Field: {:?}", PassportFields::PassportId).into()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PassportFields {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

impl FromStr for PassportFields {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let found = match s {
            "byr" => Self::BirthYear,
            "iyr" => Self::IssueYear,
            "eyr" => Self::ExpirationYear,
            "hgt" => Self::Height,
            "hcl" => Self::HairColor,
            "ecl" => Self::EyeColor,
            "pid" => Self::PassportId,
            "cid" => Self::CountryId,
            e => return Err(format!("invalid field name: `{}`", e).into()),
        };
        Ok(found)
    }
}

pub fn read_batch(reader: impl Read) -> AppResult<Vec<String>> {
    let mut output: Vec<String> = Vec::new();
    let mut current_line = String::new();
    for line in BufReader::new(reader).lines() {
        let line: String = line?;
        if line.is_empty() {
            output.push(current_line);
            current_line = String::new();
        } else {
            if ! current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(&line);
        }
    }
    if ! current_line.is_empty() {
        output.push(current_line);
    }
    Ok(output)
}

/**
 * Tokenizes a line into pairs.
 *
 * ```
 * assert_eq!(tokenize("ecl:gry pid:1234"), vec![("ecl", "gry"), ("pid", "1234")])
 * ```
 */
pub fn tokenize<'a>(line: &'a str) -> AppResult<Vec<(&'a str, &'a str)>> {
    let mut output = Vec::new();
    // first, iterate over the whitespace-delimited items
    for part in line.split_whitespace() {
        // now we split on the `:` character. There must be content on both sides.
        let mut split = part.splitn(2, ':');
        let (left, right) = match (split.next(), split.next()) {
            (Some(l), Some(r)) => (l, r),
            (_, _) => return Err(format!("failed to split field `{}`", part).into()),
        };
        output.push((left, right));
    }
    Ok(output)
}

#[test]
fn test_part_1() {
    use std::io::Cursor;
    let data = include_str!("../input.txt");
    let batch_lines = read_batch(Cursor::new(data)).unwrap();

    let num_valid = batch_lines.into_iter()
        .flat_map(|line| Passport::from_str(&line).ok())
        .count();
    assert_eq!(num_valid, 219);
}

#[test]
fn test_read_batch() {
    use std::io::Cursor;
    let data = vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
        "byr:1937 iyr:2017 cid:147 hgt:183cm",
        "",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
        "hcl:#cfa07d byr:1929",
        "",
        "hcl:#ae17e1 iyr:2013",
        "eyr:2024",
        "ecl:brn pid:760753108 byr:1931",
        "hgt:179cm",
        "",
        "hcl:#cfa07d eyr:2025 pid:166559648",
        "iyr:2011 ecl:brn hgt:59in",
    ].join("\n");
    let batch_lines = read_batch(Cursor::new(data)).unwrap();
    assert_eq!(batch_lines, vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929",
        "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm",
        "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in",
    ]);

    let num_valid = batch_lines.into_iter()
        .flat_map(|line| Passport::from_str(&line).ok())
        .count();
    assert_eq!(num_valid, 2);
}

#[test]
fn test_tokenizer() {
    assert_eq!(tokenize("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd").unwrap(), vec![
        ("ecl", "gry"), ("pid", "860033327"), ("eyr", "2020"), ("hcl", "#fffffd")
    ]);
}

#[test]
fn test_fields_from_str() {
    assert_eq!(PassportFields::from_str("byr").unwrap(), PassportFields::BirthYear);
    assert_eq!(PassportFields::from_str("iyr").unwrap(), PassportFields::IssueYear);
    assert_eq!(PassportFields::from_str("eyr").unwrap(), PassportFields::ExpirationYear);
    assert_eq!(PassportFields::from_str("hgt").unwrap(), PassportFields::Height);
    assert_eq!(PassportFields::from_str("hcl").unwrap(), PassportFields::HairColor);
    assert_eq!(PassportFields::from_str("ecl").unwrap(), PassportFields::EyeColor);
    assert_eq!(PassportFields::from_str("pid").unwrap(), PassportFields::PassportId);
    assert_eq!(PassportFields::from_str("cid").unwrap(), PassportFields::CountryId);

    assert!(PassportFields::from_str("lol").is_err())
}

#[test]
fn test_passport_from_str() {
    assert_eq!(Passport::from_str("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm").unwrap(), Passport {
        eye_color: "gry".to_string(),
        passport_id: "860033327".to_string(),
        expiration_year: "2020".to_string(),
        hair_color: "#fffffd".to_string(),
        birth_year: "1937".to_string(),
        issue_year: "2017".to_string(),
        country_id: Some("147".to_string()),
        height: "183cm".to_string(),
    });

    Passport::from_str("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929").unwrap_err();

    assert_eq!(Passport::from_str("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm").unwrap(), Passport {
        hair_color: "#ae17e1".to_string(),
        issue_year: "2013".to_string(),
        expiration_year: "2024".to_string(),
        eye_color: "brn".to_string(),
        passport_id: "760753108".to_string(),
        birth_year: "1931".to_string(),
        height: "179cm".to_string(),
        country_id: None,
    });

    Passport::from_str("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in").unwrap_err();
}
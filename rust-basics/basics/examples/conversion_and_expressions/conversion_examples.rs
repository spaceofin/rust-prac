use std::convert::{From, Into, TryFrom, TryInto};
use std::{fmt, str::FromStr};
use std::num::ParseIntError;

#[derive(Debug)]
struct Number {
    value: i32,
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

#[derive(Debug)]
struct Circle {
    radius: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// The Into trait for this struct is automatically implemented via the From trait,
// so there is no need to manually implement Into.
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

// The Into trait for this struct is automatically implemented via the From trait,
// so there is no need to manually implement Into.
// impl TryInto<EvenNumber> for i32 {
//     type Error = ();

//     fn try_into(self) -> Result<EvenNumber, Self::Error> {
//         if value % 2 == 0 {
//             Ok(EvenNumber(self))
//         } else {
//             Err(())
//         }
//     }
// }

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle{ radius: num }),
            Err(e) => Err(e),
        }
    }
}


fn from_and_into() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{}",my_string);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

fn tryfrom_and_tryinto() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(())); 
}

fn string_examples() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let radius = "    3 ";
    let circle: Circle = radius.parse().unwrap();
    println!("{:?}", circle);
}

pub fn conversion_demo() {
    from_and_into();
    tryfrom_and_tryinto();
    string_examples();
}
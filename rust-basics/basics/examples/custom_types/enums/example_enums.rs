pub enum Level {
    Easy,
    Medium,
    Hard,
}

pub enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    pub fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

 #[derive(Debug)]
// enum with implicit discriminator (starts at 0)
pub enum Number {
    Zero,
    One,
    Two,
}

 #[derive(Debug)]
// enum with explicit discriminator
pub enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
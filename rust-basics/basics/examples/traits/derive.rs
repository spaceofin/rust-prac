#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

struct Feet(i32);

impl Feet{
    fn to_inches(&self) -> Inches {
        let &Feet(feet) = self;

        Inches(feet * 12)
    }
}

#[derive(Debug, PartialEq)]
struct Seconds(i32);

struct RawSeconds(i32);

pub fn derive_demo() {
    let _one_second = RawSeconds(1);

    // Compile Error: `RawSeconds` doesn't implement the `Debug` and `PartialEq` traits.
    //println!("One second looks like: {:?}", _one_second);
    //let _this_is_true = _one_second == _one_second;

    let one_second = Seconds(1);
    println!("One second looks like: {:?}", one_second);
    let this_is_true = one_second == one_second;
    println!("this_is_true: {}",this_is_true);    

    let foot = Feet(1);
    println!("One foot equals {:?}", foot.to_inches());

    let meter = Centimeters(100.0);
    let cmp =
        if foot.to_inches().to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("One foot is {} than one meter.", cmp);

}
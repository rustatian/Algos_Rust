//Write function bmi that calculates body mass index (bmi = weight / height ^ 2).
//if bmi <= 18.5 return "Underweight"
//if bmi <= 25.0 return "Normal"
//if bmi <= 30.0 return "Overweight"
//if bmi > 30 return "Obese"
fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi_calc = weight as f32 / (height * height);

    if bmi_calc <= 18.5 {
        return "Underweight";
    }

    if bmi_calc <= 25.0 {
        return "Normal";
    }

    if bmi_calc <= 30.0 {
        return "Overweight";
    }

    return "Obese";
}

// BEST PRACTISE
fn bmi_best_practices(weight:u32, height:f32) -> &'static str {
    let index = weight as f32 / height.powi(2);
    match index {
        index if index <= 18.5 => "Underweight",
        index if index <= 25.0 => "Normal",
        index if index <= 30.0 => "Overweight",
        _ => "Obese"
    }
}

// ALSO GOOD
fn bmi_good(weight: u32, height: f32) -> &'static str {
    let w = weight as f32;
    let index = w / (height * height);

    match index {
        0.0 ... 18.5 => return "Underweight",
        18.6 ... 25.0 => return "Normal",
        25.1 ... 30.0 => return "Overweight",
        _ => return "Obese"
    }
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html
#[test]
fn basic_unit_test() {
    assert_eq!(bmi(80, 1.80), "Normal");
}
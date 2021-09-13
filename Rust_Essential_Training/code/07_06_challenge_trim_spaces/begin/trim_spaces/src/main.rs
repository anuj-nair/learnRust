fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

/* YOUR CODE GOES HERE */
fn trim_spaces(text: &str)->&str{
    let mut start=0;
    let mut end=0;

    for (index, letter) in text.chars().enumerate(){
        if letter != ' ' {
            start=index;
            break;
        }
    }

    for (index, letter ) in text.chars().rev().enumerate(){
        if letter != ' ' {
            end=text.len()-index;
            break;
        }
    }

    return &text[start..end];
}

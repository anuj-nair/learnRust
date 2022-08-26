mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut count: i32=0;
        let mut encoded  = String::new();
        let mut prev: Option<char>=None;
        let mut chars = text.chars();

        while let Some(c) = chars.next(){
            if prev.is_none(){
                prev=Some(c)
            }
            let prev_ = prev.unwrap();
            if prev_ != c || count == 9{
                encoded.push_str(&format!("{}{}",count,prev_));
                prev=Some(c);
                count=0;
            }
            count+=1;
        }
        if prev.is_some(){
            encoded.push_str(&format!("{}{}",count,prev.unwrap()));
        }

        encoded
    }
    
    pub fn decode(text: &str) -> String {
        let mut count: u32=0;
        let mut decoded  = String::new();

        for (i,c) in text.char_indices(){
            if i%2 == 0{
                count=c.to_digit(10).unwrap();
            }else{
                for _ in 0..count{
                    decoded.push_str(&format!("{}",c));
                }
            }
        }
        decoded
    }
}

fn main() {
    // 
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}

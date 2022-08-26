mod vigenere {
//    const ALPHABETS:[u8;26]=*b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const A:u8 =b'A';
    const Z:u8 =b'Z';
    const WRAP:u8 =26;

//    pub fn encrypt(plaintext: &str, key: &str) -> String {
//        String::new() // Optional
//    }

    fn clean_input(input: &str) -> impl Iterator<Item = u8> + '_{
        input.bytes().filter_map(|x|{
            match x{
                A..=Z => Some(x),
                b'a'..=b'z' => Some(x-(b'a' - A)),
                _ => None,
            }
        })
    }
    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let mut key_iter=key
            .bytes()
            .map(|b|{
                b-A
            }).cycle();
        let ciphertext=clean_input(ciphertext)
            .map(|x|{
                let letter_in_key=key_iter.next().unwrap();
                ((x+WRAP-A)-letter_in_key)%WRAP +A
            }).collect();
        String::from_utf8(ciphertext).unwrap()

    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("{}", plaintext);
}

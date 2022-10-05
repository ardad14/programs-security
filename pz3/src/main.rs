use crate::alphabet::Alphabet;
mod alphabet;

fn main() {
    let encode_text = String::from("Так он и в этом случае на меня не посмотрел. Поднял глаза на хозяйку квартиры и говорит:");
    let cipher = caesar_encode(&encode_text, 11);

    println!("Cipher text - {}", cipher);

    attack(&cipher);
}

fn attack(cipher: &String) {
    for i in 1..alphabet::Cyrillic.length() {
        let plain_text = caesar_decode(cipher, i);
        println!("Try to attack {} - {}", i, plain_text);
    }
}

fn caesar_encode(message: &String, offset: usize) -> String {
    substitution(message, |idx| {
        alphabet::Cyrillic.module((idx + offset) as isize)
    })
}

fn caesar_decode(message: &String, offset: usize) -> String {
    substitution(message, |idx| {
        alphabet::Cyrillic.module(idx as isize - offset as isize)
    })
}

fn substitution<F>(text: &String, calc_index: F) -> String
    where
        F: Fn(usize) -> usize,
{
    let mut result_string = String::new();
    for c in text.chars() {
        let pos = alphabet::Cyrillic.find_position(c);
        match pos {
            Some(pos) => {
                let si = calc_index(pos);
                result_string.push(alphabet::Cyrillic.get_letter(si, c.is_uppercase()));
            }
            None => result_string.push(c),
        }
    }

    result_string
}

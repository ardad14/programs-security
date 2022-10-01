use crate::alphabet::Alphabet;
mod alphabet;

fn main() {
    println!(
        "{}",
        caesar_encode("Так он и в этом случае на меня не посмотрел. Поднял глаза на хозяйку квартиры и говорит:", 11)
    );
    println!(
        "{}",
        caesar_decode("мьэыпъкшшёп мщцщьё у шплыпсшщьэж м щопсоп. Ъыпоьэкмцйпэп, мщщлдп шп ъщьчщэыпц м", 11)
    );
}

fn caesar_encode(message: &str, offset: usize) -> String {
    substitution(message, |idx| {
        alphabet::Cyrillic.module((idx + offset) as isize)
    })
}

fn caesar_decode(message: &str, offset: usize) -> String {
    substitution(message, |idx| {
        alphabet::Cyrillic.module(idx as isize - offset as isize)
    })
}

fn substitution<F>(text: &str, calc_index: F) -> String
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

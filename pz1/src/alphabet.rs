const CYRILLIC_ALPHABET_LOWER: [char; 33] = [
    'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т',
    'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я'
];

const CYRILLIC_ALPHABET_UPPER: [char; 33] = [
    'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р', 'С', 'Т',
    'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я'
];

pub trait Alphabet {
    fn find_position(&self, c: char) -> Option<usize>;

    fn get_letter(&self, index: usize, is_uppercase: bool) -> char;

    fn module(&self, i: isize) -> usize;

    fn length(&self) -> usize;
}

pub struct Cyrillic;
impl Alphabet for Cyrillic {
    fn find_position(&self, c: char) -> Option<usize> {
        CYRILLIC_ALPHABET_LOWER
            .iter()
            .position(|&a| a == c)
            .or_else(|| CYRILLIC_ALPHABET_UPPER.iter().position(|&a| a == c))
    }

    fn get_letter(&self, index: usize, is_uppercase: bool) -> char {
        if index > self.length() {
            panic!("Invalid index to the alphabet: {}.", index);
        }

        if is_uppercase {
            CYRILLIC_ALPHABET_UPPER[index]
        } else {
            CYRILLIC_ALPHABET_LOWER[index]
        }
    }

    fn module(&self, i: isize) -> usize {
        (((i % self.length() as isize) + self.length() as isize) % self.length() as isize) as usize
    }

    fn length(&self) -> usize {
        CYRILLIC_ALPHABET_LOWER.len()
    }
}
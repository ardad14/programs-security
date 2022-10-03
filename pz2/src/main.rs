//extern crate rand;           // Used to generate random numbers

use std::fs::File;
use std::io::Write;
use std::path::Path;
use rand::{thread_rng, Rng}; // Random number generation system

fn main() {
    //let text = String::from("25665");
    //let pad = String::from("85948");
    //let pad = generate_pad(text.chars().count());
    //println!("The pad is: {}", pad);

    //let ciphertext = encrypt(&text, &pad);
    //println!("\nThe ciphertext is: {}", ciphertext);

    let ciphertext = String::from("6719882196864085864979275245");
    let pad = String::from("8845884588458845884588458845");
    let plaintext = decrypt(&pad, &ciphertext);
    println!("\nThe decrypted plaintext is: {}", plaintext);
}

fn generate_pad(length: usize) -> String {
    let mut pad = String::new();

    let mut rng = thread_rng();
    let mut current_number: u32;

    let mut counter = 0;
    loop {
        // Grab random number between 0 and 255
        current_number = rng.gen_range(0, 255);

        // Create char option from random number
        let option = std::char::from_u32(current_number);

        // Push random character to the string
        pad.push(*option.as_ref().unwrap());

        // When pad is same length as original text, break the loop
        counter += 1;
        if counter == length{
            break;
        }
    }

    return pad;
}

fn encrypt(text: &String, pad: &String) -> String{
    let mut ciphertext = String::new();

    // Generate vectors of characters to iterate over
    let text_characters: Vec<char> = text.chars().collect();
    let pad_characters: Vec<char> = pad.chars().collect();

    // Will store the xored value of the current pad and text character
    let mut xored_value: u32;

    for i in 0 .. pad.chars().count() {
        xored_value = u32::from(text_characters[i]) ^ u32::from(pad_characters[i]);

        // Create char option from the xored Value
        let option = std::char::from_u32(xored_value);
        ciphertext.push(*option.as_ref().unwrap());
    }

    return ciphertext;

}

fn decrypt(pad: &String, ciphertext: &String) -> String{
    let mut plaintext = String::new();

    // Generate vectors of characters to iterate over
    let ciphertext_characters: Vec<char> = ciphertext.chars().collect();
    let pad_characters: Vec<char> = pad.chars().collect();

    // Will store the xored value of the current pad and text character
    let mut xored_value: u32;

    for i in 0 .. pad.chars().count() {
        //println!("pad number - {}", u32::from(pad_characters[i]) );
        //println!("ciphertext number - {}\n End subtraction", u32::from(ciphertext_characters[i]) );
        let diff = (pad_characters[i]) as i32 - ciphertext_characters[i] as i32;
        xored_value = diff.abs() as u32;

        // Create char option from the xored Value
        let option = std::char::from_u32(xored_value);
        println!("absolute value as char - {}", xored_value);
        plaintext.push(*option.as_ref().unwrap());
    }

    let path = Path::new("plaintext.txt");
    save(&plaintext, path);
    return plaintext;
}

fn save(text: &String, path: &Path){

    let mut file = File::create(path).expect("Error when creating file");

    file.write_all(text.as_bytes()).expect("Failed to write file");

}

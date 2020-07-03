// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#summary
//
// Convert strings to pig latin. The first consonant of each word is moved to
// the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
// that start with a vowel have “hay” added to the end instead (“apple” becomes
// “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn main() {
    const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

    let words = ["apple", "computer", "pig", "latin", "artery"];

    for word in words.iter() {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            if VOWELS.contains(&first.to_string().as_str()) {
                println!("{}-hay", word);
            } else {
                let s: String = chars.collect();
                println!("{}-{}ay", s, first);
            }
        }
    }
}

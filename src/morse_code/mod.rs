/// Represents a pulse in Morse code, which can be either short or long.
#[derive(Debug, PartialEq)]
pub enum Pulse {
    Short,
    Long,
}

/// Represents a single character in Morse code, which is a sequence of pulses.
pub type Letter = Vec<Pulse>;

/// Represents a string of characters in Morse code.
pub type Message = Vec<Letter>;

/// A trait for converting a string to Morse code.
pub trait MorseCode {
    /// Converts the string to Morse code.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_code_challenges::morse_code::{MorseCode, Message, Pulse};
    ///
    /// let message: Message = "SOS".to_string().to_morse_code();
    /// let expected = vec![
    ///     vec![Pulse::Short, Pulse::Short, Pulse::Short],
    ///     vec![Pulse::Long, Pulse::Long, Pulse::Long],
    ///     vec![Pulse::Short, Pulse::Short, Pulse::Short],
    /// ];
    /// assert_eq!(message, expected);
    /// ```
    fn to_morse_code(&self) -> Message;
}

impl std::fmt::Display for Pulse {
    /// Formats the pulse as a string, where a short pulse is represented by a dot (".")
    /// and a long pulse is represented by an underscore ("_").
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

impl MorseCode for String {
    /// Converts the string to Morse code.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_code_challenges::morse_code::{MorseCode, Message, Pulse};
    ///
    /// let message: Message = "Hello, world!".to_string().to_morse_code();
    /// let expected = vec![
    ///     vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
    ///     vec![Pulse::Short],
    ///     vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
    ///     vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
    ///     vec![Pulse::Long, Pulse::Long, Pulse::Long],
    ///     vec![Pulse::Short, Pulse::Long, Pulse::Long],
    ///     vec![Pulse::Long, Pulse::Long, Pulse::Long],
    ///     vec![Pulse::Short, Pulse::Long, Pulse::Short],
    ///     vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
    ///     vec![Pulse::Long, Pulse::Short, Pulse::Short],
    /// ];
    /// assert_eq!(message, expected);
    /// ```
    fn to_morse_code(&self) -> Message {
        let mut message: Message = Message::new();
        for c in self.chars() {
            let pulse = match c {
                'a' | 'A' => vec![Pulse::Short, Pulse::Long],
                'b' | 'B' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
                'c' | 'C' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short],
                'd' | 'D' => vec![Pulse::Long, Pulse::Short, Pulse::Short],
                'e' | 'E' => vec![Pulse::Short],
                'f' | 'F' => vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short],
                'g' | 'G' => vec![Pulse::Long, Pulse::Long, Pulse::Short],
                'h' | 'H' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
                'i' | 'I' => vec![Pulse::Short, Pulse::Short],
                'j' | 'J' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long],
                'k' | 'K' => vec![Pulse::Long, Pulse::Short, Pulse::Long],
                'l' | 'L' => vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
                'm' | 'M' => vec![Pulse::Long, Pulse::Long],
                'n' | 'N' => vec![Pulse::Long, Pulse::Short],
                'o' | 'O' => vec![Pulse::Long, Pulse::Long, Pulse::Long],
                'p' | 'P' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short],
                'q' | 'Q' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long],
                'r' | 'R' => vec![Pulse::Short, Pulse::Long, Pulse::Short],
                's' | 'S' => vec![Pulse::Short, Pulse::Short, Pulse::Short],
                't' | 'T' => vec![Pulse::Long],
                'u' | 'U' => vec![Pulse::Short, Pulse::Short, Pulse::Long],
                'v' | 'V' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long],
                'w' | 'W' => vec![Pulse::Short, Pulse::Long, Pulse::Long],
                'x' | 'X' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long],
                'y' | 'Y' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long],
                'z' | 'Z' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short],
                _ => vec![], // ignore unknown characters
            };
            if pulse.len() > 0 {
                message.push(pulse);
            }
        }
        message
    }
}

/// Prints a Morse code message to the console.
pub fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

#[cfg(test)]
mod moris_code_tests {
    use super::*;

    #[test]
    fn hello_world() {
        use Pulse::*;

        let expected = vec![
            vec![Short, Short, Short, Short],
            vec![Short],
            vec![Short, Long, Short, Short],
            vec![Short, Long, Short, Short],
            vec![Long, Long, Long],
            vec![Short, Long, Long],
            vec![Long, Long, Long],
            vec![Short, Long, Short],
            vec![Short, Long, Short, Short],
            vec![Long, Short, Short],
        ];

        let actual = "Hello, world".to_string().to_morse_code();
        assert_eq!(actual, expected);
    }

    #[test]
    fn whole_alphabet_case_insensitive() {
        let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

        let morse_code_lowercase: Message = alphabet.to_morse_code();
        let morse_code_uppercase: Message = alphabet.to_uppercase().to_morse_code();

        assert_eq!(morse_code_lowercase, morse_code_uppercase);
    }
}

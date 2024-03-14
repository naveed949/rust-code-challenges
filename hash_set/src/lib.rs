
use std::collections::HashSet;

pub enum LearningMode {
    ByWords,
    ByCharacters,
    ByLength(usize),
}

struct TextAnalyzer {
    brain: HashSet<String>,
    learning_mode: LearningMode,
}


impl TextAnalyzer {
    fn new(learning_mode: LearningMode) -> TextAnalyzer {
        TextAnalyzer {
            brain: HashSet::new(),
            learning_mode,
        }
    }


    fn learn(&mut self, text: &str) {
        match self.learning_mode {
            LearningMode::ByWords => {
                for word in text.split_whitespace() {
                    self.brain.insert(word.to_string());
                }
            }
            LearningMode::ByCharacters => {
                for character in text.chars() {
                    self.brain.insert(character.to_string());
                }
            }
            LearningMode::ByLength(length) => {
                let mut chunk = String::new();
                for (i, c) in text.chars().enumerate(){
                    if i < text.len() && chunk.len() < length {
                        chunk.push(c);
                    } else {
                        self.brain.insert(chunk.clone());
                        chunk.clear();
                    }
                }
            }
        }
    }


    fn query(&self, text: &str) -> bool {
        for word in text.split_whitespace() {
            if !self.brain.contains(word) {
                return false;
            }
        }
        true
    }
}
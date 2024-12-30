pub(crate) struct Prettifier {
    indentation_character: char,
    number_of_indentation_characters: u8
}

impl Prettifier {
    pub fn new(indentation_character: char, number_of_indentation_characters: u8) -> Self {
        Prettifier{
            indentation_character,
            number_of_indentation_characters,
        }
    }

    pub(crate) fn prettify(&self, transpiled_code: String) -> String {
        let mut output = String::new();

        let mut number_of_open_parens: i32 = 0;

        for character in transpiled_code.chars() {
            number_of_open_parens += match character { '(' => 1, ')' => -1, _ => 0 };

            while character == ')' && char::is_whitespace(output.chars().last().unwrap()) {
                output.remove(output.len() - 1);
            }

            output.push(character);

            if character == '\n' && number_of_open_parens > 0 {
                output.push_str(self.indentation_character.to_string()
                    .repeat((self.number_of_indentation_characters as i32 * number_of_open_parens) as usize)
                    .as_str());
            }
        }

        output
    }
}
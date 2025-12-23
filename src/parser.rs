pub struct Parser {
    pos: usize,
    pub input: String,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            pos: 0,
            input: input,
        }
    }

    /// Paskatās, bet neconsumo nākamo simbolu
    pub fn peek(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Consumo un atgriežo nākamo simbolu
    pub fn next(&mut self) -> char {
        let c = self.peek();
        self.pos += c.len_utf8();
        return c;
    }

    /// Consumo nākamo simbolu un asserto pret 'expected_char'
    pub fn next_expect(&mut self, expected_char: char) -> char {
        let c = self.next();

        if c != expected_char {
            panic!(
                "Expected {} at byte {}, but found {}",
                expected_char,
                self.pos - c.len_utf8(),
                c
            );
        }

        return c;
    }

    // darbojas tāpat kā 'next_expect', bet matcho pret piedāvāto simbolu sarakstu
    // 'expected_chars',
    // potenciālajiem simboliem jābūt vienkārši simbolu virnē pēc kārtas,
    // piemēram, lai matchotu dažāda veida atdalītājsimbolus, strings jāveido kā: ;,:|.
    pub fn next_expect_options(&mut self, expected_chars: &str) -> char {
        let c = self.next();

        if !expected_chars.contains(c) {
            panic!(
                "Expected one of {} at byte {}, but found {}",
                // starp 'expected_char' ieliek atdološos komatus, lai vieglāk lasīt err msg
                expected_chars
                    .chars()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, c)| {
                        acc.push(c);
                        if i < expected_chars.len() - 1 {
                            acc.push(',');
                        }
                        acc
                    }),
                self.pos - c.len_utf8(),
                c
            );
        }

        return c;
    }

    pub fn next_expect_str(&mut self, s: &str) {
        if self.starts_with(s) {
            self.pos += s.len();
        } else {
            panic!("Expected {} at byte {} but it was not found", s, self.pos);
        }
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn next_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();

        while !self.eof() && test(self.peek()) {
            result.push(self.next());
        }

        return result;
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    pub fn consume_whitespace(&mut self) {
        self.next_while(char::is_whitespace);
    }

    pub fn parse_name(&mut self) -> String {
        self.next_while(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0' ..= '9'))
    }

    pub fn parsing_error(&self, reason: String) {
        panic!("Error at byte {}.\nReason: \n{}", self.pos, reason);
    }
}

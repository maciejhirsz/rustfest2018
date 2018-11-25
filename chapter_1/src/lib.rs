#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token<'source> {
    /// Should match a combination of letters a-z and A-Z.
    /// We don't need underscores but feel free to add them.
    Identifier(&'source str),

    /// We will just need integers, so sequences of digits 0-9
    /// will suffice
    Number(u64),

    /// '+'
    Add,

    /// '-'
    Subtract,

    /// '*'
    Multiply,

    /// '/'
    Divide,

    /// '='
    Assign,

    /// ';'
    Semicolon,
}

/// This struct needs some fields!
pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    /// It also needs some code
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            index: 0,
        }
    }
}

// Douglas Crockford - Syntaxation | Pratt Parser


/// We will also use the `Iterator` trait from the
/// standard library for our Lexer.
impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let token = loop {
            let bytes: &[u8] = self.source.as_bytes();
            let byte: &u8 = bytes.get(self.index)?;

            let token_start = self.index;
            self.index += 1;


            match *byte {
                b'a'...b'z' |
                b'A'...b'Z' => {
                    loop {
                        let byte = self.source.as_bytes().get(self.index)?;

                        match byte {
                            b'a'...b'z' |
                            b'A'...b'Z' => self.index += 1,

                            _ => break,
                        }
                    }

                    let slice = self.source
                            .get(token_start..self.index)?;

                    break Token::Identifier(slice);
                },
                b'0'...b'9' => {
                    let mut num: u64 = (*byte - b'0') as u64;

                    loop {
                        let byte = self.source.as_bytes().get(self.index)?;

                        match *byte {
                            b'0'...b'9' => {
                                num = num * 10 + ((*byte - b'0') as u64);

                                self.index += 1;
                            },

                            _ => break,
                        }
                    }

                    break Token::Number(num);
                },
                b'+' => break Token::Add,
                b'-' => break Token::Subtract,
                b'*' => break Token::Multiply,
                b'/' => break Token::Divide,
                b'=' => break Token::Assign,
                b';' => break Token::Semicolon,
                b' ' | b'\n' => continue,
                _ => return None,
            }
        };

        Some(token)
    }
}

#[cfg(test)]
#[test]
fn test() {
    let source = "four = 2 + 2; omg = 12345 / 0;";

    let expect = &[
        Token::Identifier("four"),
        Token::Assign,
        Token::Number(2),
        Token::Add,
        Token::Number(2),
        Token::Semicolon,
        Token::Identifier("omg"),
        Token::Assign,
        Token::Number(12345),
        Token::Divide,
        Token::Number(0),
        Token::Semicolon,
    ];

    // Create an iterator of Tokens out of the slice here.
    // let expect = expect.iter().cloned();
    let lexer = Lexer::new(source);

    let got = lexer.collect::<Vec<_>>();

    // We can use the `eq` method of the `Iterator` trait
    // to check that they are equal
    assert_eq!(&got, expect);
}

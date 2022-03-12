use std::env;
use std::fmt;

#[derive(PartialEq)]
enum TokenKind {
    RESERVED,
    NUM,
    EOF
}

struct Token {
    kind: TokenKind ,
    next: Option<Box<Token>>,
    val: isize,
    str_: String
}

impl Token {
    fn new() -> Self {
        Token {
            kind: TokenKind::EOF,
            next: None,
            val: 0,
            str_: String::new()
        }
    }

    fn consume(&self, op: char)->Result<&Option<Box<Token>>, &str> {
        if self.kind != TokenKind::RESERVED 
            || self.str_.chars().next() != Some(op) {
                return Err("Invalid operation");
        }
        Ok(&self.next)
    }

    fn expect_number(&self) ->Result<isize, &str> {
        match self.kind {
            TokenKind::NUM => Ok(self.val),
            _ => Err("Not a number")
        }
    }

    fn at_eof(&self) -> bool {
        self.kind == TokenKind::EOF
    }

    fn append(&mut self, kind: TokenKind, str_: Option<&str>)->&mut Token {
        let tok = Box::new(Token {
            kind,
            next: None,
            val: 0,
            str_: str_.map_or(String::new(), |x| x.to_string()),
        });
        self.next = Some(tok);
        self.next.as_mut().unwrap()
    }

    fn tokenize(input: &str)->Self {
        let mut head = Token::new();
        let mut cur = &mut head;

        for p in input.chars() {
            match p {
                ' ' => continue,
                '+' | '-' => {
                    cur = cur.append(TokenKind::RESERVED, Some(&p.to_string()));
                },
                _ => panic!("this cannot tokenize")
            }
        }
        cur.append(TokenKind::EOF, None);
        head
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut head = self.next.as_ref();
        while let Some(tok) = head {
            match tok.kind {
                TokenKind::RESERVED => write!(f, "op: {},", tok.str_),
                TokenKind::NUM => write!(f, "num: {},", tok.val),
                TokenKind::EOF => write!(f, "eof.")
            };
            head = tok.next.as_ref();
        }
        write!(f, "")
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("invalid number of argments");
    }

    let input = &args[1];

    let token = Token::tokenize("+ - +");
    println!("token: {:?}", token);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    // println!("  mov rax, {}", );
    println!("  ret");
}

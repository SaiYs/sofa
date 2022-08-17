#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Reserved(&'static str),
    Num(u32),
    EOF,
}

#[derive(Debug)]
pub struct TokenList {
    tokens: Vec<Token>,
    head: usize,
}

impl TokenList {
    fn new(source: String) -> Self {
        let mut tokens = Vec::new();
        let source = source.chars().collect::<Vec<_>>();
        let mut cur = 0;
        while cur < source.len() {
            match source[cur] {
                ws if ws.is_ascii_whitespace() => cur += 1,
                d if d.is_digit(10) => {
                    let l = (cur..)
                        .take_while(|&x| x < source.len() && source[x].is_digit(10))
                        .count();
                    let n = source[cur..cur + l]
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    tokens.push(Token::Num(n));
                    cur += l;
                }
                _ => {
                    if source[cur..].starts_with(&['=', '=']) {
                        tokens.push(Token::Reserved("=="));
                        cur += 2;
                    }
                    if source[cur..].starts_with(&['!', '=']) {
                        tokens.push(Token::Reserved("!="));
                        cur += 2;
                    }
                    if source[cur..].starts_with(&['<', '=']) {
                        tokens.push(Token::Reserved("<="));
                        cur += 2;
                    }
                    if source[cur..].starts_with(&['>', '=']) {
                        tokens.push(Token::Reserved(">="));
                        cur += 2;
                    }
                    if source[cur..].starts_with(&['<']) {
                        tokens.push(Token::Reserved("<"));
                        cur += 1;
                    }
                    if source[cur..].starts_with(&['>']) {
                        tokens.push(Token::Reserved(">"));
                        cur += 1;
                    }
                    if source[cur..].starts_with(&['+']) {
                        tokens.push(Token::Reserved("+"));
                        cur += 1;
                    }
                    if source[cur..].starts_with(&['-']) {
                        tokens.push(Token::Reserved("-"));
                        cur += 1;
                    }
                    if source[cur..].starts_with(&['*']) {
                        tokens.push(Token::Reserved("*"));
                        cur += 1;
                    }
                    if source[cur..].starts_with(&['/']) {
                        tokens.push(Token::Reserved("/"));
                        cur += 1;
                    }
                    if source[cur..].starts_with(&['(']) {
                        tokens.push(Token::Reserved("("));
                        cur += 1;
                    }
                    if source[cur..].starts_with(&[')']) {
                        tokens.push(Token::Reserved(")"));
                        cur += 1;
                    }
                }
            }
        }

        tokens.push(Token::EOF);

        TokenList { tokens, head: 0 }
    }

    fn get(&self) -> &Token {
        &self.tokens[self.head]
    }

    fn consume(&mut self, expected: Token) -> bool {
        let f = self.get() == &expected;
        if f {
            self.head += 1;
        }
        f
    }

    fn expect(&mut self, expected: Token) -> Token {
        match self.get() {
            actual if actual == &expected => {
                self.head += 1;
                expected
            }
            _ => panic!("unexpected token"),
        }
    }

    fn expect_number(&mut self) -> u32 {
        let res = match self.get() {
            Token::Num(n) => *n,
            _ => panic!("expected number but found not number"),
        };
        self.head += 1;
        res
    }

    // fn expect_reserved(&mut self) -> String {
    //     match self.get() {
    //         Token::Reserved(ref s) => {
    //             self.head += 1;
    //             s.clone()
    //         }
    //         _ => panic!("expected reserved but found not reserved"),
    //     }
    // }

    // fn eof(&self) -> bool {
    //     self.get() == Token::EOF
    // }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num(u32),
    Eq,
    Neq,
    Le,
    LeEq,
}

#[derive(Debug)]
pub struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    fn new(tokens: &mut TokenList) -> Self {
        Self::expr(tokens)
    }

    fn expr(tokens: &mut TokenList) -> Self {
        Self::equality(tokens)
    }

    fn equality(tokens: &mut TokenList) -> Self {
        let mut res = Self::relational(tokens);

        loop {
            if tokens.consume(Token::Reserved("==")) {
                res = Node {
                    kind: NodeKind::Eq,
                    lhs: Some(Box::new(res)),
                    rhs: Some(Box::new(Self::relational(tokens))),
                }
            } else if tokens.consume(Token::Reserved("!=")) {
                res = Node {
                    kind: NodeKind::Neq,
                    lhs: Some(Box::new(res)),
                    rhs: Some(Box::new(Self::relational(tokens))),
                }
            } else {
                return res;
            }
        }
    }

    fn relational(tokens: &mut TokenList) -> Self {
        let mut res = Self::add(tokens);

        loop {
            if tokens.consume(Token::Reserved("<=")) {
                res = Node {
                    kind: NodeKind::LeEq,
                    lhs: Some(Box::new(res)),
                    rhs: Some(Box::new(Self::add(tokens))),
                }
            } else if tokens.consume(Token::Reserved(">=")) {
                res = Node {
                    kind: NodeKind::LeEq,
                    lhs: Some(Box::new(Self::add(tokens))),
                    rhs: Some(Box::new(res)),
                }
            } else if tokens.consume(Token::Reserved("<")) {
                res = Node {
                    kind: NodeKind::Le,
                    lhs: Some(Box::new(res)),
                    rhs: Some(Box::new(Self::add(tokens))),
                }
            } else if tokens.consume(Token::Reserved(">")) {
                res = Node {
                    kind: NodeKind::Le,
                    lhs: Some(Box::new(Self::add(tokens))),
                    rhs: Some(Box::new(res)),
                }
            } else {
                return res;
            }
        }
    }

    fn add(tokens: &mut TokenList) -> Self {
        let mut res = Self::mul(tokens);

        loop {
            if tokens.consume(Token::Reserved("+")) {
                res = Node {
                    kind: NodeKind::Add,
                    lhs: Some(Box::new(res)),
                    rhs: Some(Box::new(Self::mul(tokens))),
                };
            } else if tokens.consume(Token::Reserved("-")) {
                res = Node {
                    kind: NodeKind::Sub,
                    lhs: Some(Box::new(res)),
                    rhs: Some(Box::new(Self::mul(tokens))),
                };
            } else {
                return res;
            }
        }
    }

    fn mul(tokens: &mut TokenList) -> Self {
        let mut res = Self::unary(tokens);

        loop {
            if tokens.consume(Token::Reserved("*")) {
                res = Node {
                    kind: NodeKind::Mul,
                    lhs: Some(Box::new(res)),
                    rhs: Some(Box::new(Self::unary(tokens))),
                };
            } else if tokens.consume(Token::Reserved("/")) {
                res = Node {
                    kind: NodeKind::Div,
                    lhs: Some(Box::new(res)),
                    rhs: Some(Box::new(Self::unary(tokens))),
                };
            } else {
                return res;
            }
        }
    }

    fn unary(tokens: &mut TokenList) -> Self {
        if tokens.consume(Token::Reserved("+")) {
            Self::primary(tokens)
        } else if tokens.consume(Token::Reserved("-")) {
            Node {
                kind: NodeKind::Sub,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num(0),
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(Self::primary(tokens))),
            }
        } else {
            Self::primary(tokens)
        }
    }

    fn primary(tokens: &mut TokenList) -> Self {
        if tokens.consume(Token::Reserved("(")) {
            let inner = Self::add(tokens);
            tokens.expect(Token::Reserved(")"));
            inner
        } else {
            Node {
                kind: NodeKind::Num(tokens.expect_number()),
                lhs: None,
                rhs: None,
            }
        }
    }
}

fn gen(node: Node) {
    if let NodeKind::Num(n) = node.kind {
        println!("    push {}", n);
    } else {
        gen(*node.lhs.unwrap());
        gen(*node.rhs.unwrap());

        println!("    pop rdi");
        println!("    pop rax");

        match node.kind {
            NodeKind::Add => println!("    add rax, rdi"),
            NodeKind::Sub => println!("    sub rax, rdi"),
            NodeKind::Mul => println!("    imul rax, rdi"),
            NodeKind::Div => {
                println!("    cqo");
                println!("    idiv rdi");
            }
            NodeKind::Num(_) => unreachable!(),
            NodeKind::Eq => {
                println!("    cmp rax, rdi");
                println!("    sete al");
                println!("    movzb rax, al");
            }
            NodeKind::Neq => {
                println!("    cmp rax, rdi");
                println!("    setne al");
                println!("    movzb rax, al");
            }
            NodeKind::Le => {
                println!("    cmp rax, rdi");
                println!("    setl al");
                println!("    movzb rax, al");
            }
            NodeKind::LeEq => {
                println!("    cmp rax, rdi");
                println!("    setle al");
                println!("    movzb rax, al");
            }
        }

        println!("    push rax");
    }
}

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        panic!("invalid args");
    }

    let source = args.nth(1).unwrap();
    let mut tokens = TokenList::new(source);
    let node = Node::new(&mut tokens);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    gen(node);

    println!("    pop rax");
    println!("    ret");
}

#[test]
fn test_tokenize() {
    let s = "1 + 2 >= (4 - 1)";
    let mut tokens = TokenList::new(s.to_string());
    dbg!(&tokens);

    let node = Node::new(&mut tokens);
    dbg!(&node);
}

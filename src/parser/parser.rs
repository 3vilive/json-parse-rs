use super::tokenizer::{Token, TokenKind};

#[derive(Debug)]
pub enum NodeKind {
    Root,
    Boolean,
    Null,
    String,
    Number,
    Object,
    Array,
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub value: Option<Vec<char>>,
    pub children: Option<Vec<Node>>,
}

pub struct TokenParser {
    pc: usize,
    tokens: Vec<Token>,
}

impl TokenParser {
    pub fn new() -> Self {
        Self {
            pc: 0,
            tokens: vec![],
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Node, String> {
        self.pc = 0;
        self.tokens = tokens;

        let mut root = Node {
            kind: NodeKind::Root,
            value: None,
            children: None,
        };

        let mut children = vec![];
        while self.pc < self.tokens.len() {
            let node = self.walk()?;
            children.push(node);
        }

        root.children = Some(children);
        Ok(root)
    }

    fn walk(&mut self) -> Result<Node, String> {
        let mut token = &self.tokens[self.pc];
        // println!("walk pc({}) Token(kind {:?} value {})", self.pc, token.kind, token.value.iter().collect::<String>());

        match token.kind {
            TokenKind::CurlyBracket => {
                let bracket = token.value.iter().collect::<String>();
                match bracket.as_str() {
                    "{" => {
                        let mut object_node = Node {
                            kind: NodeKind::Object,
                            value: None,
                            children: None,
                        };
                        let mut children = vec![];
        
                        self.pc += 1;
                        token = &self.tokens[self.pc];
        
                        while token.kind != TokenKind::CurlyBracket
                            || (token.kind == TokenKind::CurlyBracket
                                && token.value.iter().collect::<String>() != "}")
                        {
                            children.push(self.walk()?);
                            token = &self.tokens[self.pc];
                        }
                        self.pc += 1;
                        object_node.children = Some(children);
                        return Ok(object_node);
                    },
                    "}" => {
                        return Err(format!("unexpected '}}'"))
                    }
                    _ => {
                        return Err(format!("invalid curly bracket '{}'", bracket))
                    }
                }
            }
            TokenKind::SquareBracket => {
                let bracket = token.value.iter().collect::<String>();
                match bracket.as_str() {
                    "[" => {
                        let mut children = vec![];

                        self.pc += 1;
                        token = &self.tokens[self.pc];

                        while token.kind != TokenKind::SquareBracket || (
                            token.kind == TokenKind::SquareBracket && token.value.iter().collect::<String>() != "]"
                        ) {
                            children.push(self.walk()?);
                            token = &self.tokens[self.pc];
                        }

                        self.pc += 1;

                        let array_node = Node {
                            kind: NodeKind::Array,
                            value: None,
                            children: Some(children),
                        };
                        return Ok(array_node)
                    }
                    "]" => {
                        return Err(format!("unexpected '}}'"))
                    }
                    _ => {
                        return Err(format!("invalid curly bracket '{}'", bracket))
                    }
                }
            }
            TokenKind::Colon | TokenKind::Comma => {
                self.pc += 1;
                return self.walk();
            }
            TokenKind::True | TokenKind::False => {
                self.pc += 1;
                return Ok(Node {
                    kind: NodeKind::Boolean,
                    value: Some(token.value.clone()),
                    children: None,
                });
            }
            TokenKind::Null => {
                self.pc += 1;
                return Ok(Node {
                    kind: NodeKind::Null,
                    value: None,
                    children: None,
                });
            }
            TokenKind::String => {
                self.pc += 1;
                return Ok(Node {
                    kind: NodeKind::String,
                    value: Some(token.value.clone()),
                    children: None,
                });
            }
            TokenKind::Number => {
                self.pc += 1;
                return Ok(Node {
                    kind: NodeKind::Number,
                    value: Some(token.value.clone()),
                    children: None,
                });
            }
            TokenKind::Unknown(ref value) => {
                return Err(format!(
                    "unknown token `{}`",
                    value.iter().collect::<String>()
                ))
            }
        }
    }
}

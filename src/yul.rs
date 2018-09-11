use std::fmt;

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct Literal {
    pub literal: String,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct Identifier {
    pub identifier: String,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct FunctionCall {
    pub identifier: Identifier,
    pub arguments: Vec<Expression>,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub returns: Vec<Identifier>,
    pub block: Block,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct VariableDeclaration {
    pub identifiers: Vec<Identifier>,
    pub expression: Option<Expression>,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct Assignment {
    pub identifiers: Vec<Identifier>,
    pub expression: Expression,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    FunctionCall(FunctionCall),
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct If {
    pub expression: Expression,
    pub block: Block,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct Case {
    pub literal: Literal,
    pub block: Block,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct Switch {
    pub expression: Expression,
    pub cases: Vec<Case>,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub struct ForLoop {
    pub pre: Block,
    pub condition: Expression,
    pub post: Block,
    pub body: Block,
}

#[derive(Hash, Clone, PartialEq, Debug)]
pub enum Statement {
    Block(Block),
    FunctionDefinition(FunctionDefinition),
    VariableDeclaration(VariableDeclaration),
    Assignment(Assignment),
    Expression(Expression),
    If(If),
    Switch(Switch),
    ForLoop(ForLoop),
    Break,
    Continue,
}

impl Identifier {
    pub fn new(identifier: &str) -> Self {
        Identifier {
            identifier: identifier.to_string(),
        }
    }
}

impl Literal {
    pub fn new(literal: &str) -> Self {
        Literal {
            literal: literal.to_string(),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}(", self.identifier));
        for (i, argument) in self.arguments.iter().enumerate() {
            try!(write!(f, "{}", argument));
            if i < self.arguments.len() - 1 {
                try!(write!(f, ","));
            }
        }
        write!(f, ")")
    }
}

impl fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "function {}(", self.name));
        for (i, identifier) in self.parameters.iter().enumerate() {
            try!(write!(f, "{}", identifier));
            if i < self.parameters.len() - 1 {
                try!(write!(f, ", "));
            }
        }
        try!(write!(f, ")"));
        if self.returns.len() > 0 {
            try!(write!(f, " -> "));
            for (i, identifier) in self.returns.iter().enumerate() {
                try!(write!(f, "{}", identifier));
                if i < self.returns.len() - 1 {
                    try!(write!(f, ", "));
                }
            }
        }
        write!(f, " {}", self.block)
    }
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: should validate this on the new/default trait
        if self.identifiers.len() == 0 {
            panic!("VariableDeclaration must have identifiers")
        }
        try!(write!(f, "let "));
        for (i, identifier) in self.identifiers.iter().enumerate() {
            try!(write!(f, "{}", identifier));
            if i < self.identifiers.len() - 1 {
                try!(write!(f, ", "));
            }
        }
        if let Some(expression) = &self.expression {
            write!(f, " := {}", expression)
        } else {
            write!(f, "")
        }
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: should validate this on the new/default trait
        if self.identifiers.len() == 0 {
            panic!("Assignment must have identifiers")
        }
        for (i, identifier) in self.identifiers.iter().enumerate() {
            try!(write!(f, "{}", identifier));
            if i < self.identifiers.len() - 1 {
                try!(write!(f, ", "));
            }
        }
        write!(f, " := {}", self.expression)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Literal(ref literal) => write!(f, "{}", literal),
            Expression::Identifier(ref identifier) => write!(f, "{}", identifier),
            Expression::FunctionCall(ref functioncall) => write!(f, "{}", functioncall),
        }
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if {} {}", self.expression, self.block)
    }
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.literal.literal.len() == 0 {
            write!(f, "default: {}", self.block)
        } else {
            write!(f, "case {}: {}", self.literal, self.block)
        }
    }
}

impl fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "switch {} ", self.expression));
        for case in &self.cases {
            try!(write!(f, "{} ", case));
        }
        write!(f, "")
    }
}

impl fmt::Display for ForLoop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "for {} {} {} {}",
            self.pre, self.condition, self.post, self.body
        )
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Block(ref block) => write!(f, "{}", block),
            Statement::FunctionDefinition(ref function) => write!(f, "{}", function),
            Statement::VariableDeclaration(ref variabledeclaration) => {
                write!(f, "{}", variabledeclaration)
            }
            Statement::Assignment(ref assignment) => write!(f, "{}", assignment),
            Statement::Expression(ref expression) => write!(f, "{}", expression),
            Statement::If(ref if_statement) => write!(f, "{}", if_statement),
            Statement::Switch(ref switch) => write!(f, "{}", switch),
            Statement::ForLoop(ref forloop) => write!(f, "{}", forloop),
            Statement::Break => write!(f, "break"),
            Statement::Continue => write!(f, "continue"),
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{{"));
        for (_, statement) in self.statements.iter().enumerate() {
            try!(write!(f, " {}", statement));
        }
        write!(f, " }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal() {
        assert_eq!(
            Literal {
                literal: "testliteral".to_string(),
            }.to_string(),
            "testliteral"
        );
    }

    #[test]
    fn identifier() {
        assert_eq!(
            Identifier {
                identifier: "testidentifier".to_string(),
            }.to_string(),
            "testidentifier"
        );
    }

    #[test]
    fn functioncall() {
        assert_eq!(
            FunctionCall {
                identifier: Identifier {
                    identifier: "test".to_string(),
                },
                arguments: vec![
                    Expression::Identifier(Identifier {
                        identifier: "test".to_string(),
                    }),
                    Expression::Literal(Literal {
                        literal: "literal".to_string(),
                    }),
                ],
            }.to_string(),
            "test(test,literal)"
        );
    }

    #[test]
    fn if_statement() {
        assert_eq!(
            If {
                expression: Expression::Literal(Literal {
                    literal: "literal".to_string(),
                }),
                block: Block { statements: vec![] },
            }.to_string(),
            "if literal { }"
        );
    }

    #[test]
    fn block_empty() {
        assert_eq!(Block { statements: vec![] }.to_string(), "{ }");
    }

    #[test]
    fn block_nested() {
        assert_eq!(
            Block {
                statements: vec![Statement::Block(Block { statements: vec![] })],
            }.to_string(),
            "{ { } }"
        );
    }

    #[test]
    fn block_literal() {
        assert_eq!(
            Block {
                statements: vec![Statement::Expression(Expression::Literal(Literal {
                    literal: "literal".to_string(),
                }))],
            }.to_string(),
            "{ literal }"
        );
    }

    #[test]
    fn assignment_single() {
        assert_eq!(
            Assignment {
                identifiers: vec![Identifier {
                    identifier: "a".to_string(),
                }],
                expression: Expression::Literal(Literal {
                    literal: "1".to_string(),
                }),
            }.to_string(),
            "a := 1"
        );
    }

    #[test]
    fn assignment_multi() {
        assert_eq!(
            Assignment {
                identifiers: vec![
                    Identifier {
                        identifier: "a".to_string(),
                    },
                    Identifier {
                        identifier: "b".to_string(),
                    },
                    Identifier {
                        identifier: "c".to_string(),
                    },
                ],
                expression: Expression::Literal(Literal {
                    literal: "1".to_string(),
                }),
            }.to_string(),
            "a, b, c := 1"
        );
    }

    #[test]
    fn variabledeclaration_empty() {
        assert_eq!(
            VariableDeclaration {
                identifiers: vec![Identifier {
                    identifier: "a".to_string(),
                }],
                expression: None,
            }.to_string(),
            "let a"
        );
    }

    #[test]
    fn variabledeclaration_single() {
        assert_eq!(
            VariableDeclaration {
                identifiers: vec![Identifier {
                    identifier: "a".to_string(),
                }],
                expression: Some(Expression::Literal(Literal {
                    literal: "1".to_string(),
                })),
            }.to_string(),
            "let a := 1"
        );
    }

    #[test]
    fn variabledeclaration_multi() {
        assert_eq!(
            VariableDeclaration {
                identifiers: vec![
                    Identifier {
                        identifier: "a".to_string(),
                    },
                    Identifier {
                        identifier: "b".to_string(),
                    },
                    Identifier {
                        identifier: "c".to_string(),
                    },
                ],
                expression: Some(Expression::Literal(Literal {
                    literal: "1".to_string(),
                })),
            }.to_string(),
            "let a, b, c := 1"
        );
    }

    #[test]
    fn functiondefinition_basic() {
        assert_eq!(
            FunctionDefinition {
                name: Identifier {
                    identifier: "name".to_string(),
                },
                parameters: vec![],
                returns: vec![],
                block: Block { statements: vec![] },
            }.to_string(),
            "function name() { }"
        );
    }

    #[test]
    fn functiondefinition_single_arg() {
        assert_eq!(
            FunctionDefinition {
                name: Identifier {
                    identifier: "name".to_string(),
                },
                parameters: vec![Identifier {
                    identifier: "a".to_string(),
                }],
                returns: vec![],
                block: Block { statements: vec![] },
            }.to_string(),
            "function name(a) { }"
        );
    }

    #[test]
    fn functiondefinition_single_ret() {
        assert_eq!(
            FunctionDefinition {
                name: Identifier {
                    identifier: "name".to_string(),
                },
                parameters: vec![],
                returns: vec![Identifier {
                    identifier: "a".to_string(),
                }],
                block: Block { statements: vec![] },
            }.to_string(),
            "function name() -> a { }"
        );
    }

    #[test]
    fn functiondefinition_multi() {
        assert_eq!(
            FunctionDefinition {
                name: Identifier {
                    identifier: "name".to_string(),
                },
                parameters: vec![
                    Identifier {
                        identifier: "a".to_string(),
                    },
                    Identifier {
                        identifier: "b".to_string(),
                    },
                ],
                returns: vec![
                    Identifier {
                        identifier: "c".to_string(),
                    },
                    Identifier {
                        identifier: "d".to_string(),
                    },
                ],
                block: Block { statements: vec![] },
            }.to_string(),
            "function name(a, b) -> c, d { }"
        );
    }

    #[test]
    fn case() {
        assert_eq!(
            Case {
                literal: Literal {
                    literal: "literal".to_string(),
                },
                block: Block { statements: vec![] },
            }.to_string(),
            "case literal: { }"
        );
    }

    #[test]
    fn case_default() {
        assert_eq!(
            Case {
                literal: Literal {
                    literal: "".to_string(),
                },
                block: Block { statements: vec![] },
            }.to_string(),
            "default: { }"
        );
    }

    #[test]
    fn switch() {
        assert_eq!(
            Switch {
                expression: Expression::Literal(Literal {
                    literal: "3".to_string(),
                }),
                cases: vec![
                    Case {
                        literal: Literal {
                            literal: "1".to_string(),
                        },
                        block: Block { statements: vec![] },
                    },
                    Case {
                        literal: Literal {
                            literal: "".to_string(),
                        },
                        block: Block { statements: vec![] },
                    },
                ],
            }.to_string(),
            "switch 3 case 1: { } default: { } "
        );
    }

    #[test]
    fn forloop() {
        assert_eq!(
            ForLoop {
                pre: Block { statements: vec![] },
                condition: Expression::Literal(Literal {
                    literal: "1".to_string(),
                }),
                post: Block { statements: vec![] },
                body: Block { statements: vec![] },
            }.to_string(),
            "for { } 1 { } { }"
        );
    }
}

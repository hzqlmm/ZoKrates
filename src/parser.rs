/*

Grammar:

<statement> ::= <ide> `=' <expr> `\\n' 1

<return> ::= `return' <expr> `\\n' 2

<expr> ::= `if' <expr> <comparator> <expr> `then' <expr> `else' <expr> `fi' 3 <expr'>
	\alt `(' <expr> `)' 11 <term'> 6 <expr'>
	\alt <ide> 12 <term'> 6 <expr'>
	\alt <num> 13 <term'> 6 <expr'>

<expr'> ::= `+' <term> 4 <expr'>
	\alt `-' <term> 5 <expr'>
	\alt `**' <num> 10 <term'> 6 <expr'>
	\alt $\varepsilon$

<term> ::= <factor> <term'>

<term'> ::= `*' <term> 7
	\alt `/' <term> 8
	\alt $\varepsilon$ 9

<factor> ::= `if' <expr> <comparator> <expr> `then' <expr> `else' <expr> `fi' 3 <expr'> `**' <num> 10
	\alt `(' <expr> `)' 11 <factor'>
	\alt <ide> 12 <factor'>
	\alt <num> 13 <factor'>

<factor'> ::= <term'> 6 <expr'> `**' <num> 10
	\alt $\varepsilon$

<comparator> ::= `<' | `<=' | `==' | `>=' | `>'

<num> ::= `d' <num> | `d' 14

<ide> ::= `l' <trail> | `l' 15

<trail> ::= `d' <trail> | `l' <trail> | `d' 16 | `l' 17

*/

extern crate regex;

use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use self::regex::Regex;
use absy::*;

#[derive(Clone)]
struct Position {
    line: usize,
    col: usize
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

enum Token {
    Open, Close, Eq, Return,
    If, Then, Else, Fi,
    Lt, Le, Eqeq, Ge, Gt,
    Add, Sub, Mult, Div, Pow,
    Ide(String),
    Num(i32),
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Open => write!(f, "("),
            Token::Close => write!(f, ")"),
            Token::Eq => write!(f, "="),
            Token::Return => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::Fi => write!(f, "fi"),
            Token::Lt => write!(f, "<"),
            Token::Le => write!(f, "<="),
            Token::Eqeq => write!(f, "=="),
            Token::Ge => write!(f, ">="),
            Token::Gt => write!(f, ">"),
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mult => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Pow => write!(f, "**"),
            Token::Ide(ref x) => write!(f, "{}", x),
            Token::Num(ref x) => write!(f, "{}", x),
        }
    }
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn parse_num(input: &String, pos: Position) -> (Token, String, Position) {
    let mut end = 0;
    loop {
        match input.chars().nth(end) {
            Some(x) => match x {
                '0'...'9' => end += 1,
                _ => break,
            },
            None => break,
        }
    }
    assert!(end > 0);
    (Token::Num(input[0..end].parse().unwrap()), input[end..].to_string(), Position { line: pos.line, col: pos.col + end })
}

fn parse_ide(input: &String, pos: Position) -> (Token, String, Position) {
    assert!(match input.chars().next().unwrap() { 'a'...'z' | 'A'...'Z' => true, _ => false });
    let mut end = 1;
    loop {
        match input.chars().nth(end) {
            Some(x) => match x {
                'a'...'z' | 'A'...'Z' | '0'...'9' => end += 1,
                _ => break,
            },
            None => break,
        }
    }
    (Token::Ide(input[0..end].to_string()), input[end..].to_string(), Position { line: pos.line, col: pos.col + end })
}

fn skip_whitespaces(input: &String) -> usize {
    let mut i = 0;
    loop {
        match input.chars().nth(i) {
            Some(' ') | Some('\t') => i += 1,
            _ => return i,
        }
    }
}

fn next_token(input: &String, pos: Position) -> Option<(Token, String, Position)> {
    let offset = skip_whitespaces(input);
    Some(match input.chars().nth(offset) {
        Some('(') => (Token::Open, input[offset + 1..].to_string(), Position { line: pos.line, col: pos.col + offset + 1 }),
        Some(')') => (Token::Close, input[offset + 1..].to_string(), Position { line: pos.line, col: pos.col + offset + 1 }),
        Some('=') => match input.chars().nth(offset + 1) {
            Some('=') => (Token::Eqeq, input[offset + 2..].to_string(), Position { line: pos.line, col: pos.col + offset + 2 }),
            _ => (Token::Eq, input[offset + 1..].to_string(), Position { line: pos.line, col: pos.col + offset + 1 }),
        },
        Some('<') => match input.chars().nth(offset + 1) {
            Some('=') => (Token::Le, input[offset + 2..].to_string(), Position { line: pos.line, col: pos.col + offset + 2 }),
            _ => (Token::Lt, input[offset + 1..].to_string(), Position { line: pos.line, col: pos.col + offset + 1 }),
        },
        Some('>') => match input.chars().nth(offset + 1) {
            Some('=') => (Token::Ge, input[offset + 2..].to_string(), Position { line: pos.line, col: pos.col + offset + 2 }),
            _ => (Token::Gt, input[offset + 1..].to_string(), Position { line: pos.line, col: pos.col + offset + 1 }),
        },
        Some('+') => (Token::Add, input[offset + 1..].to_string(), Position { line: pos.line, col: pos.col + offset + 1 }),
        Some('-') => (Token::Sub, input[offset + 1..].to_string(), Position { line: pos.line, col: pos.col + offset + 1 }),
        Some('*') => match input.chars().nth(offset + 1) {
            Some('*') => (Token::Pow, input[offset + 2..].to_string(), Position { line: pos.line, col: pos.col + offset + 2 }),
            _ => (Token::Mult, input[offset + 1..].to_string(), Position { line: pos.line, col: pos.col + offset + 1 }),
        },
        Some('/') => (Token::Div, input[offset + 1..].to_string(), Position { line: pos.line, col: pos.col + offset + 1 }),
        Some(_) if input[offset..].starts_with("return ") => (Token::Return, input[offset + 7..].to_string(), Position { line: pos.line, col: pos.col + offset + 7 }),
        Some(_) if input[offset..].starts_with("if ") => (Token::If, input[offset + 3..].to_string(), Position { line: pos.line, col: pos.col + offset + 3 }),
        Some(_) if input[offset..].starts_with("then ") => (Token::Then, input[offset + 5..].to_string(), Position { line: pos.line, col: pos.col + offset + 5 }),
        Some(_) if input[offset..].starts_with("else ") => (Token::Else, input[offset + 5..].to_string(), Position { line: pos.line, col: pos.col + offset + 5 }),
        Some(_) if input[offset..].starts_with("fi ") || input[offset..].to_string() == "fi" => (Token::Fi, input[offset + 2..].to_string(), Position { line: pos.line, col: pos.col + offset + 2 }),
        Some(x) => match x {
            '0'...'9' => parse_num(&input[offset..].to_string(), Position { line: pos.line, col: pos.col + offset }),
            'a'...'z' | 'A'...'Z' => parse_ide(&input[offset..].to_string(), Position { line: pos.line, col: pos.col + offset }),
            _ => panic!("unexpected"),
        },
        None => return None,
    })
}

fn parse_if_then_else(input: &String, pos: Position) -> Result<(Expression, String, Position), String> {
    match next_token(input, pos.clone()) {
        Some((Token::If, s1, p1)) => match parse_expr(s1, p1) {
            Ok((e2, s2, p2)) => match next_token(&s2, p2) {
                Some((Token::Lt, s3, p3)) => match parse_expr(s3, p3) {
                    Ok((e4, s4, p4)) => match next_token(&s4, p4) {
                        Some((Token::Then, s5, p5)) => match parse_expr(s5, p5) {
                            Ok((e6, s6, p6)) => match next_token(&s6, p6) {
                                Some((Token::Else, s7, p7)) => match parse_expr(s7, p7) {
                                    Ok((e8, s8, p8)) => match next_token(&s8, p8) {
                                        Some((Token::Fi, s9, p9)) => parse_expr1(Expression::IfElse(box Condition::Lt(e2, e4), box e6, box e8), s9, p9),
                                        _ => Err(format!("Expected `fi`, got ...")),
                                    },
                                    err => err,
                                },
                                _ => Err(format!("Expected `else`, got ...")),
                            },
                            err => err,
                        },
                        _ => Err(format!("Expected `then`, got ...")),
                    },
                    err => err,
                },
                _ => unimplemented!()
            },
            err => err,
        },
        _ => Err(format!("Expected `if`, got ...")),
    }
}

fn parse_factor1(expr: Expression, input: String, pos: Position) -> Result<(Expression, String, Position), String> {
    match parse_term1(expr.clone(), input.clone(), pos.clone()) {
        Ok((e1, s1, p1)) => match parse_expr1(e1, s1, p1) {
            Ok((e2, s2, p2)) => match next_token(&s2, p2) {
                Some((Token::Pow, s3, p3)) => match next_token(&s3, p3.clone()) {
                    Some((Token::Num(x), s4, p4)) => Ok((Expression::Pow(box e2, box Expression::NumberLiteral(x)), s4, p4)),
                    _ => Err(format!("Exprected number at {}, got: ...", p3)),
                },
                _ => Ok((expr, input, pos)),
            },
            Err(why) => Err(why),
        },
        Err(why) => Err(why),
    }
}

fn parse_factor(input: String, pos: Position) -> Result<(Expression, String, Position), String> {
    match next_token(&input, pos.clone()) {
        Some((Token::If, ..)) => parse_if_then_else(&input, pos),
        Some((Token::Open, s1, p1)) => match parse_expr(s1, p1) {
            Ok((e2, s2, p2)) => match next_token(&s2, p2) {
                Some((Token::Close, s3, p3)) => parse_factor1(e2, s3, p3),
                _ => Err(format!("Expected `)`, got ...")),
            },
            Err(why) => Err(why),
        },
        Some((Token::Ide(x), s1, p1)) => parse_factor1(Expression::VariableReference(x), s1, p1),
        Some((Token::Num(x), s1, p1)) => parse_factor1(Expression::NumberLiteral(x), s1, p1),
        e => Err(format!("expected one of `if`, `(`, variable or a number, found {:?}\n\tat {}", e, pos)),
    }
}

fn parse_term1(expr: Expression, input: String, pos: Position) -> Result<(Expression, String, Position), String> {
    match next_token(&input, pos.clone()) {
        Some((Token::Mult, s1, p1)) => {
            match parse_term(s1, p1) {
                Ok((e, s2, p2)) => Ok((Expression::Mult(box expr, box e), s2, p2)),
                Err(why) => Err(why),
            }
        },
        Some((Token::Div, s1, p1)) => {
            match parse_term(s1, p1) {
                Ok((e, s2, p2)) => Ok((Expression::Div(box expr, box e), s2, p2)),
                Err(why) => Err(why),
            }
        },
        _ => Ok((expr, input, pos)),
    }
}

fn parse_term(input: String, pos: Position) -> Result<(Expression, String, Position), String> {
    match parse_factor(input, pos) {
        Ok((e, s1, p1)) => parse_term1(e, s1, p1),
        e @ Err(_) => e,
    }
}

fn parse_expr1(expr: Expression, input: String, pos: Position) -> Result<(Expression, String, Position), String> {
    match next_token(&input, pos.clone()) {
        Some((Token::Add, s1, p1)) => {
            match parse_term(s1, p1) {
                Ok((e2, s2, p2)) => parse_expr1(Expression::Add(box expr, box e2), s2, p2),
                Err(why) => Err(why),
            }
        },
        Some((Token::Sub, s1, p1)) => {
            match parse_term(s1, p1) {
                Ok((e2, s2, p2)) => parse_expr1(Expression::Sub(box expr, box e2), s2, p2),
                Err(why) => Err(why),
            }
        },
        Some((Token::Pow, s1, p1)) => {
            match parse_num(&s1, p1) {
                (Token::Num(x), s2, p2) => match parse_term1(Expression::Pow(box expr, box Expression::NumberLiteral(x)), s2, p2) {
                    Ok((e3, s3, p3)) => parse_expr1(e3, s3, p3),
                    Err(why) => Err(why),
                },
                (t2, _, p2) => Err(format!("Expected number, got `{}` at {}", t2, p2)),
            }
        },
        _ => Ok((expr, input, pos)),
    }
}

fn parse_expr(input: String, pos: Position) -> Result<(Expression, String, Position), String> {
    match next_token(&input, pos.clone()) {
        Some((Token::If, ..)) => parse_if_then_else(&input, pos),
        Some((Token::Open, s1, p1)) => match parse_expr(s1, p1) {
            Ok((e2, s2, p2)) => match next_token(&s2, p2) {
                Some((Token::Close, s3, p3)) => match parse_term1(e2, s3, p3) {
                    Ok((e4, s4, p4)) => parse_expr1(e4, s4, p4),
                    Err(why) => Err(why),
                },
                _ => Err(format!("Expected `)`, got ...")),
            },
            Err(why) => Err(why),
        },
        Some((Token::Ide(x), s1, p1)) => {
            match parse_term1(Expression::VariableReference(x), s1, p1) {
                Ok((e2, s2, p2)) => parse_expr1(e2, s2, p2),
                Err(why) => Err(why),
            }
        }
        Some((Token::Num(x), s1, p1)) => {
            match parse_term1(Expression::NumberLiteral(x), s1, p1) {
                Ok((e2, s2, p2)) => parse_expr1(e2, s2, p2),
                Err(why) => Err(why),
            }
        },
        _ => panic!("unexpected"),
    }
}

fn parse_statement(input: &String, pos: Position) -> Result<(Statement, String, Position), String> {
    match next_token(input, pos) {
        Some((Token::Ide(x), s1, p1)) => {
            match next_token(&s1, p1) {
                Some((Token::Eq, s2, p2)) => match parse_expr(s2, p2) {
                    Ok((expr, s3, p3)) => Ok((Statement::Definition(x, expr), s3, p3)),
                    Err(e) => Err(e),
                },
                Some((t, _, p2)) => Err(format!("Expected '=' at {}, got: '{}'", p2, t)),
                None => Err(format!("Expexted '=', got nothing ...")),
            }
        },
        Some((Token::Return, s1, p1)) => {
            match parse_expr(s1, p1) {
                Ok((expr, s, p)) => {
                    assert_eq!(s, "");
                    Ok((Statement::Return(expr), s, p))
                },
                Err(e) => Err(e),
            }
        },
        e => Err(format!("Error parsing statement, got: {:?}", e)),
    }
}

pub fn parse_program(file: File) -> Result<Prog, String> {
    let mut current_line = 1;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // TODO def parse old
    let id;
    let args;
    let def_regex = Regex::new(r"^def\s(?P<id>\D[a-zA-Z0-9]+)\(\s*([a-z]+)(,\s*[a-z]+)*\s*\):$").unwrap();
    let args_regex = Regex::new(r"\(\s*(?P<args>[a-z]+(,\s*[a-z]+)*)\s*\)").unwrap();
    loop { // search and make Prog
        match lines.next() {
            Some(Ok(ref x)) if x.starts_with("def") => {
                id = match def_regex.captures(x) {
                    Some(x) => x["id"].to_string(),
                    None => panic!("Wrong definition of function"),
                };
                args = match args_regex.captures(x) {
                    Some(x) => x["args"].replace(" ", "").replace("\t", "").split(",")
                                        .map(|p| Parameter { id: p.to_string() })
                                        .collect::<Vec<_>>(),
                    None => panic!("Wrong argument definition in function: {}", id),
                };
                break;
            },
            Some(Ok(ref x)) if x.trim().starts_with("//") || x == "" => {},
            None => panic!("End of file reached without function def"),
            Some(x) => panic!("Found '{:?}' outside of function", x),
        }
        current_line += 1;
    };

    let mut defs = Vec::new();
    loop {
        match lines.next() {
            Some(Ok(ref x)) if x.trim().starts_with("//") => {}, // skip
            Some(Ok(ref x)) => match parse_statement(x, Position { line: current_line, col: 0 }) {
                Ok((statement, ..)) => defs.push(statement),
                Err(e) => panic!("Error: {}", e),
            },
            None => break,
            Some(Err(e)) => panic!("Error while reading Definitions: {}", e),
        }
        current_line += 1;
    }

    match defs.last() {
        Some(&Statement::Return(_)) => {},
        Some(x) => panic!("Last definition not Return: {}", x),
        None => panic!("Error while checking last definition"),
    }
    Ok(Prog { id: id, arguments: args, statements: defs })
}
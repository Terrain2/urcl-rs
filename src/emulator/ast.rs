use std::collections::HashMap; // hi

use super::{*, lexer::{Token, Kind, UToken}, errorcontext::ErrorContext};

struct TokenBuffer<'a> {
    index: usize,
    toks: Vec<UToken<'a>>
}
impl <'a> TokenBuffer<'a> {
    #[inline]
    pub fn new(toks: Vec<UToken<'a>>) -> Self {
        TokenBuffer {
            toks: toks,
            index: 0,
        }
    }
    #[inline]
    pub fn has_next(&self) -> bool {
        self.index < self.toks.len()
    }
    #[inline]
    pub fn advance(&mut self) {
        self.index += 1;
        while self.current().kind == Kind::White {
            self.index += 1;
        }
    }
    #[inline]
    pub fn next(&mut self) -> UToken<'a> {
        self.advance();
        self.toks[self.index].clone()
    }
    #[inline]
    pub fn current(&self) -> UToken<'a> {
        if self.has_next() {
            self.toks[self.index].clone()
        } else{
            Token {kind: Kind::EOF, str: ""}
        }
    }
}

struct Parser<'a> {
    buf: TokenBuffer<'a>,
    err: ErrorContext,
    ast: Program
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

pub fn gen_ast<'a>(toks: Vec<UToken<'a>>) -> Program {
    let mut err = ErrorContext::new();
    let mut ast = Program::new();
    let mut buf = TokenBuffer::new(toks);
    let mut p = Parser {buf, err, ast};

    while p.buf.has_next() {
        match p.buf.current().kind {
            Kind::Name => { // removes first char
                match p.buf.current().str {
                    "imm" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        //lets put it in the lexer then
                        p.ast.instructions.push(
                            Inst::IMM(op1, op2)
                        );
                        p.buf.advance();
                    },
                    "mov" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::MOV(op1, op2)
                        );
                        p.buf.advance();
                    },
                    "add" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        let op3 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::ADD(op1, op2, op3)
                        );
                        p.buf.advance();
                    }
                    "rsh" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::RSH(op1, op2)
                        );
                        p.buf.advance();
                    }
                    "lod" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Memory(v) => Operand::Mem(v as u64), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::LOD(op1, op2)
                        );
                        p.buf.advance();
                    }
                    "str" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Memory(v) => Operand::Mem(v as u64), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::STR(op1, op2)
                        );
                        p.buf.advance();
                    }
                    "bge" => {
                        p.buf.advance(); // TODO: add labels
                        p.buf.advance();
                        p.buf.advance();
                        p.buf.advance();
                    }
                    "nor" => {
                        let op1 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), _ => {continue;} };
                        let op2 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        let op3 = match p.buf.next().kind { Kind::Reg(v) => Operand::Reg(v), Kind::Int(v) => Operand::Imm(v as u64), _ => {continue;} };
                        p.ast.instructions.push(
                            Inst::NOR(op1, op2, op3)
                        );
                        p.buf.advance();
                    }
                    _ => { jsprintln!("unhandled name"); p.buf.advance(); },
                }
            } // yes
            Kind::White => p.buf.advance(),
            _ => { logprintln!("Unhandled token type: {:#?}", p.buf.current());  p.buf.advance(); },
        }
    }

    p.ast
}

// impl Parser { // bram if i commit this can i go to sleep
    // fn operand(&mut self) -> Option<Operand> {
    //     // let op = self.buf.current().
    // }
// }

#[derive(Debug)]
pub struct Program {
    headers: Headers,
    instructions: Vec<Inst>,
    labels: HashMap<&'static str, u64>
}

impl Program {
    pub fn new() -> Self {
        Program { headers: Headers::new(), instructions: Vec::new(), labels: HashMap::new() }
    }
}

#[derive(Debug)]
pub enum Operand {
    Imm(u64),
    Reg(u64),
    Mem(u64),
}

#[derive(Debug)]
pub struct Headers {
    bits: u64,
    minheap: u64,
    minstack: u64,
    minram: u64,
    minreg: u64
}

impl Headers {
    pub fn new() -> Self {
        Headers { bits: 8, minheap: 16, minstack: 16, minram: 16, minreg: 8 }
    }
}

#[derive(Debug)]
pub enum Inst {
    IMM(Operand, Operand),
    ADD(Operand, Operand, Operand),
    RSH(Operand, Operand),
    LOD(Operand, Operand),
    STR(Operand, Operand),
    BGE(Operand, Operand, Operand),
    NOR(Operand, Operand, Operand),
    MOV(Operand, Operand),
}
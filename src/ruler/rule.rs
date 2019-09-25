pub mod rpn;
pub mod token;

use crate::facts::Fact;
use token::{Operand, Token};

#[derive(Copy, Clone, PartialEq)]
pub enum Side {
    Lhs,
    Rhs,
    Pending,
    Bidirectional,
}

// type Rule<'rule> = Vec<Token<'rule>>
pub struct Rule<'rule> {
    pub lhs: Vec<Token<'rule>>,
    pub rhs: Vec<Token<'rule>>,
    pub is_equivalent: bool,
}

impl<'rule> Rule<'rule> {
    pub fn new() -> Rule<'rule> {
        Rule {
            lhs: Vec::new(),
            rhs: Vec::new(),
            is_equivalent: false,
        }
    }

    pub fn to_rpn(&mut self) {
        self.lhs = rpn::apply_on_vec(&self.lhs);
        self.rhs = rpn::apply_on_vec(&self.rhs);
    }

    pub fn push(&mut self, side: &Side, operand: Option<Operand>, fact: Option<&'rule Fact>) {
        if *side == Side::Lhs {
            self.lhs.push(Token::new(operand, fact));
        } else {
            self.rhs.push(Token::new(operand, fact));
        }
    }

    pub fn print(&self) {
        println!("Rule: (LHS)");
        for token in &self.lhs {
            println!(
                "\tToken: operand {:?}, fact: {:?}",
                token.operand, token.fact
            );
        }
        println!("Implies (=> RHS) ; <=> : {}", self.is_equivalent);
        for token in &self.rhs {
            println!(
                "\tToken: operand {:?}, fact: {:?}",
                token.operand, token.fact
            );
        }
    }
}
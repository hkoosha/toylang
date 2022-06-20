use std::cell::RefCell;
use std::rc::Rc;

use crate::lang::lexer::token::TokenKind;
use crate::lang::parser::rule::{Rule, ToRule};

#[allow(dead_code)]
const GRAMMAR: &str = "

S               -> fn_call | fn_declaration
fn_call         -> IDT ( args ) ;
fn_declaration  -> fn ( args ) { statements }
args            -> args0 | args1
args0           -> arg , args
args1           -> arg
arg             -> IDT IDT
statements      -> statements0 | statements1
statements0     -> statement , statements
statements1     -> statement
statement       -> declaration | assignment
declaration     -> IDT IDT ;
assignment      -> IDT = expression ;
expression      -> expression0 | expression1 | expression2
expression0     -> expression + term
expression1     -> expression - term
expression2     -> term
term            -> term0 | term1 | term2
term0           -> term * factor
term1           -> term / factor
term2           -> factor
factor          -> factor0 | factor1 | factor2
factor0         -> ( expression )
factor1         -> INT
factor2         -> IDT

";

pub fn toylang_v0_rules() -> Rc<RefCell<Rule>> {
    let mut e_num = 0;
    let mut expandable = move |name: &'static str, rules: Vec<Rc<RefCell<Rule>>>| {
        e_num += 1;
        Rc::new(RefCell::new(Rule::Expandable {
            name: name.to_string(),
            num: e_num,
            sub_rules: rules,
        }))
    };
    let push = |into: &Rc<RefCell<Rule>>, item: &Rc<RefCell<Rule>>| match &mut *Rc::clone(into)
        .borrow_mut()
    {
        Rule::Expandable { sub_rules: rules, .. } => {
            rules.push(Rc::clone(item));
        }
        Rule::Alternative { sub_rules: rules, .. } => {
            rules.push(Rc::clone(item));
        }
        _ => panic!(),
    };

    let push_all = |into: &Rc<RefCell<Rule>>, item: Vec<&Rc<RefCell<Rule>>>| {
        for i in item {
            push(into, i);
        }
    };

    let mut a_num = 0;
    let mut alternative = move |name: &'static str, rules: Vec<Rc<RefCell<Rule>>>| {
        a_num += 1;
        Rc::new(RefCell::new(Rule::Alternative {
            name: name.to_string(),
            num: a_num,
            sub_rules: rules,
        }))
    };

    let rbc: Rc<RefCell<Rule>> = TokenKind::Rbc.to_rule();
    let lbc = TokenKind::Lbc.to_rule();
    let lpr = TokenKind::Lpr.to_rule();
    let rpr = TokenKind::Rpr.to_rule();
    let fun = TokenKind::Fun.to_rule();
    let semi = TokenKind::Smi.to_rule();
    let comma = TokenKind::Com.to_rule();
    let equ = TokenKind::Equ.to_rule();
    let identifier = TokenKind::Idt.to_rule();
    let int = TokenKind::Int.to_rule();
    let mul = TokenKind::Mul.to_rule();
    let div = TokenKind::Sls.to_rule();
    let plus = TokenKind::Pls.to_rule();
    let minus = TokenKind::Min.to_rule();

    let arg = expandable("arg", vec![Rc::clone(&identifier), Rc::clone(&identifier)]);
    let args0 = expandable("args0", vec![Rc::clone(&arg), Rc::clone(&comma)]);
    let args1 = expandable("args1", vec![Rc::clone(&arg)]);
    let args = alternative("args", vec![Rc::clone(&args0), Rc::clone(&args1)]);
    push(&args0, &args);

    // -----------------------------

    let term0 = expandable("term0", vec![]);
    let term1 = expandable("term1", vec![]);
    let term2 = expandable("term2", vec![]);
    let term = alternative(
        "term",
        vec![Rc::clone(&term0), Rc::clone(&term1), Rc::clone(&term2)],
    );

    let factor0 = expandable("factor0", vec![]);
    let factor1 = expandable("factor1", vec![Rc::clone(&int)]);
    let factor2 = expandable("factor2", vec![Rc::clone(&identifier)]);
    let factor = alternative(
        "factor",
        vec![
            Rc::clone(&factor0),
            Rc::clone(&factor1),
            Rc::clone(&factor2),
        ],
    );

    let expression0 = expandable("expression0", vec![]);
    let expression1 = expandable("expression1", vec![]);
    let expression2 = expandable("expression2", vec![Rc::clone(&term)]);
    let expression = alternative(
        "expression",
        vec![
            Rc::clone(&expression0),
            Rc::clone(&expression1),
            Rc::clone(&expression2),
        ],
    );

    push_all(&expression0, vec![&expression, &plus, &term]);
    push_all(&expression1, vec![&expression, &minus, &term]);

    push_all(&factor0, vec![&lpr, &expression, &rpr]);
    push_all(&term0, vec![&term, &mul, &factor]);
    push_all(&term1, vec![&term, &div, &factor]);
    push_all(&term2, vec![&factor]);

    // -----------------------------

    let declaration = expandable(
        "declaration",
        vec![
            Rc::clone(&identifier),
            Rc::clone(&identifier),
            Rc::clone(&semi),
        ],
    );
    let assignment = expandable(
        "assignment",
        vec![
            Rc::clone(&identifier),
            Rc::clone(&equ),
            Rc::clone(&expression),
            Rc::clone(&semi),
        ],
    );
    let statement = alternative(
        "statement",
        vec![Rc::clone(&declaration), Rc::clone(&assignment)],
    );
    let statements0 = expandable(
        "statements0",
        vec![Rc::clone(&statement), Rc::clone(&comma)],
    );
    let statements1 = expandable("statements1", vec![Rc::clone(&statement)]);
    let statements = alternative(
        "statements",
        vec![Rc::clone(&statements0), Rc::clone(&statements1)],
    );
    push(&statements0, &statements);

    let fn_declaration = expandable(
        "fn_declaration",
        vec![
            fun,
            Rc::clone(&lpr),
            Rc::clone(&args),
            Rc::clone(&rpr),
            lbc,
            statements,
            rbc,
        ],
    );

    let fn_call = expandable(
        "fn_call",
        vec![
            Rc::clone(&identifier),
            Rc::clone(&lpr),
            Rc::clone(&args),
            Rc::clone(&rpr),
            Rc::clone(&semi),
        ],
    );

    alternative("S", vec![fn_call, fn_declaration])
}
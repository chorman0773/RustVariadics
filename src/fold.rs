use syn::{Expr, Type, Error, ExprBinary, BinOp, ExprParen};
use proc_macro::Ident;
use syn::export::{Span, TokenStream};
use syn::token::Token;
use syn::parse::{Parse, ParseBuffer, Parser};
use crate::tokens;
use quote::ToTokens;
use crate::pattern::{ExpandablePattern, expand_pattern};


pub enum FoldContents{
    Left{
        expr: Expr,
        op: BinOp,
        expand: Token![...]
    },
    Right{
        expand: Token![...],
        op: BinOp,
        expr: Expr
    }
}

fn fold_left(op: BinOp,mut exprs: Vec<Expr>,span: Span) -> Option<Expr>{
    match exprs.len(){
        0 => None,
        1 => {
            exprs.into_iter().next()
        },
        2 => {
            let mut iter = exprs.into_iter();
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            Some(Expr::Binary(ExprBinary{attrs: vec![],left: box a,op,right:box b}))
        },
        _ => {
            let back = exprs.pop().unwrap();
            let inner = Expr::Paren(ExprParen{attrs: vec![],paren_token: Paren{span}, expr: box fold_left(op,exprs,span).unwrap()});
            Some(Expr::Binary(ExprBinary{attrs: vec![],left: box back,op,right:box inner}))
        }
    }
}

fn fold_right(op: BinOp,mut exprs: Vec<Expr>,span: Span) -> Option<Expr>{
    match exprs.len(){
        0 => None,
        1 => {
            exprs.into_iter().next()
        },
        2 => {
            let mut iter = exprs.into_iter();
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            Some(Expr::Binary(ExprBinary{attrs: vec![],left: box a,op,right:box b}))
        },
        _ => {
            let back = exprs.remove(0);
            let inner = Expr::Paren(ExprParen{attrs: vec![],paren_token: Paren{span}, expr: box fold_right(op,exprs,span).unwrap()});
            Some(Expr::Binary(ExprBinary{attrs: vec![],left: box inner,op,right:box back}))
        }
    }
}

impl Parse for FoldContents{
    fn parse(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        Ok(if input.peek(Token![...]){
            FoldContents::Right {
                expand: input.parse()?,
                op: input.parse()?,
                expr: input.parse()?
            }
        }else{
          FoldContents::Left {
              expr: input.parse()?,
              op: input.parse()?,
              expand: input.parse()?
          }
        })
    }
}

impl ToTokens for FoldContents{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self{
            FoldContents::Right { expr, op, expand } => {
                expr.to_tokens(tokens);
                op.to_tokens(tokens);
                expand.to_tokens(tokens);
            }
            FoldContents::Left { expand, op, expr} => {
                expand.to_tokens(tokens);
                op.to_tokens(tokens);
                expand.to_tokens(tokens);
            }
        }
    }
}

pub struct FoldExpression{
    fold: keyword::fold,
    paren: Paren,
    contents: FoldContents
}

impl Parse for FoldExpression{
    fn parse(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(tokens::Fold){
            let content;
            Ok(FoldExpression{
                fold: input.parse()?,
                paren: syn::parenthesized!(content in input),
                contents: content.parse()?
            })
        }else{
            Err(input.error("This is not a fold expression"))
        }
    }
}

impl ToTokens for FoldExpression{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.fold.to_tokens(tokens);
        self.paren.surround(tokens,|t|self.contents.to_tokens(t))
    }
}

impl ExpandablePattern for FoldExpression{
    type Pattern = FoldContents;
    type Expanded = Expr;

    fn expand(self, n: u8, pack: &mut Vec<Ident>) -> Option<Self::Expanded> {
        match self.contents{

        }
    }

    fn pattern(&self) -> &Self::Pattern {
        &self.contents
    }
}



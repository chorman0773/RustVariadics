
use crate::tokens;
use syn::token::Token;
use syn::parse::{Parse, ParseBuffer, Parser};
use syn::{Error, Expr, ExprLit, ExprRange, RangeLimits};
use syn::{LitInt,Lit};
use proc_macro2::Ident;
use crate::pattern::ExpandablePattern;
use syn::export::Span;

pub enum PackSize{
    Exact(u8),
    Range(u8,RangeLimits,Option<u8>),
    SizeOf(SizeOfPack)
}

impl Parse for PackSize{
    fn parse(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(tokens::SizeOf){
            Ok(PackSize::SizeOf(input.parse()?))
        }else{
            let expr: Expr = input.parse()?;
            match expr{
                Expr::Lit(ExprLit{attrs,lit: Lit::Int(lint)}) => {
                    PackSize::Exact(lint.base10_parse()?)
                },
                Expr::Range(ExprRange{attrs,from: Some(box Expr::Lit(ExprLit{attrs,lit: Lit::Int(lint)})), limits, to }) => {
                    PackSize::Range(lint.base10_parse()?,limits,if let Some(box expr) = to{
                        Some(if let Expr::Lit(ExprLit{attrs,lit: Lit::Int(lint)}) = expr{
                            lint.base10_parse()
                        }else{
                            Err(input.error("This is not a valid pack size expression"))
                        }?)
                    }else{None})
                }
                _ => Err(input.error("This is not a valid pack size expression"))
            }
        }
    }
}

pub enum SizeIndicator{
    Sized{size_token: tokens::SizedPack,paren: Paren,size: PackSize},
    NonEmpty(tokens::NonEmpty),
    Default
}


pub struct SizeOfPack{
    span: Span,
    sizeof: tokens::SizeOf,
    paren: Paren,
    pack: Ident
}

impl Parse for SizeOfPack{
    fn parse(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let content;
        Ok(SizeOfPack{
            span: input.span(),
            sizeof: input.parse()?,
            paren: syn::parenthesized!(content in input),
            pack: content.parse()?
        })
    }
}

impl ExpandablePattern for SizeOfPack{
    type Pattern = Ident;
    type Expanded = LitInt;

    fn expand(self, n: u8, pack: &mut Vec<Ident>) -> Option<Self::Expanded> {
        if pack.remove_item(&self.pack).is_some(){
            Some(LitInt::new(&format!("{}",n),self.span))
        }else{
            None
        }
    }

    fn pattern(&self) -> &Self::Pattern {
        &self.pack
    }
}

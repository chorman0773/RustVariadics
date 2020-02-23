
use syn::{Expr, ExprField, ExprPath, Index, ExprArray, Block, Stmt, Local, Type};
use proc_macro::{Ident};
use syn::token::Token;
use syn::export::{Span, ToTokens};
use core::panicking::panic;
use core::iter::{IntoIterator,Iterator};
use syn::parse::Parse;

fn replace_stmt_pattern(stmt: &mut Stmt,replace: &Ident,n: u8,s: Span){
    match stmt{
        Stmt::Local(Local{ init: Some((_,expr)), .. }) => {
            let tmp = std::mem::replace(expr.as_mut(),Expr::Verbatim("".into_token_stream()));
            **expr = replace_pattern(tmp,replace,n,s);
        },
        Stmt::Expr(e) => {
            let tmp = std::mem::replace(e,Expr::Verbatim("".into_token_stream()));
            *e = replace_pattern(tmp,replace,n,s);
        },
        Stmt::Semi(e,_) => {
            let tmp = std::mem::replace(e,Expr::Verbatim("".into_token_stream()));
            *e = replace_pattern(tmp,replace,n,s);
        }
        _ => {}
    }
}
fn replace_block_pattern(block: &mut Block,replace: &Ident,n :u8,s: Span){
    for stat in block.stmts.iter_mut(){
        replace_stmt_pattern(stat,replace,n,s)
    }
}

///
/// When a variadic pattern is found, replaces all instances of *replace* with
/// *replace*.*n*. Used by expand_pattern or fold_pattern
pub fn replace_pattern(pattern: Expr, replace: &Ident, n: u8,s: Span) -> Expr{
    match pattern{
        path @ Expr::Path(ExprPath{ref attrs,qself: None,ref path}) => {
            if path.is_ident(replace) {
                let member = Member::Unamed(Index{index: n as u32, span: s });
                Expr::Field(ExprField{attrs: vec![],base: box path,dot_token: Token![.], member })
            }else{
                path
            }
        },
        Expr::Field(mut field) => {
            *field.base = replace_pattern(*field.base,replace,n,s);
            Expr::Field(field)
        },
        a @ Expr::Array(ExprArray{ref attrs, ref bracket_token,ref mut elems }) => {
            for t in elems.iter_mut(){
                let tmp = std::mem::replace(t,Expr::Verbatim("".into_token_stream()));
                *t = replace_pattern(tmp,replace,n,s);
            }
            a
        },
        Expr::Reference(mut refr) =>{
            *refr.expr = replace_pattern(*refr.expr,replace,n,s);
            Expr::Reference(refr)
        },
        Expr::Paren(mut expr) => {
            *expr.expr = replace_pattern(*expr.expr,replace,n,s);
            Expr::Paren(expr)
        },
        expr @ Expr::Yield(_) | Expr::Return(_) => {
            panic!("Invalid expression {} in pattern, neither yield nor return expression may be or appear in a pattern",expr);
        }
        v => v
    }
}

///
/// Helper function to expand a pattern expression with a set of
pub fn expand_pattern<I: IntoIterator<Item=&Ident>+Clone>(pattern: Expr, replace: I,count: u8, s: Span) -> Vec<Expr>{
    (0..count).zip(std::iter::repeat(pattern))
        .map(|(n,mut expr)|{
            for i in replace.clone().into_iter(){
                expr = replace_pattern(expr,i,n,s)
            };
            expr
        }).collect()
}

pub trait ExpandablePattern{
    type Pattern;
    type Expanded: ToTokens;
    fn expand(self,n: u8,pack: &mut Vec<Ident>) -> Option<Self::Expanded>;
    fn pattern(&self) -> &Self::Pattern;
}


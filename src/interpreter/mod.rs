use std::{collections::HashMap, panic, rc::Rc};
use antlr_rust::{InputStream, Parser, common_token_stream::CommonTokenStream, parser_rule_context::{BaseParserRuleContext, ParserRuleContext}, token::Token, tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat}};
use crate::parser::{magilexer::MagiLexer, magiparser::{ADD, AccessContext, AccessContextAttrs, AndOrContext, AndOrContextAttrs, Arith1Context, Arith1ContextAttrs, Arith2Context, Arith2ContextAttrs, BoolContext, CmpExpContext, CmpExpContextAttrs, DIV, EQ, EqExpContext, EqExpContextAttrs, FloatContext, GT, GTE, IfContext, IfContextAttrs, IfElseContext, IfElseContextAttrs, IntContext, LT, LTE, MOD, MUL, MagiParser, MagiParserContextType, MainContext, MainContextAttrs, MainContextExt, NEQ, NotContext, NotContextAttrs, PowContextAttrs, PrintContext, PrintContextAttrs, SUB, SeqContext, SeqContextAttrs, VarContext, VarContextAttrs, WhileContext, WhileContextAttrs}, magivisitor::MagiVisitorCompat};
use crate::parser::magiparser::PowContext;

mod panic_error_listener;
mod value;

use crate::interpreter::panic_error_listener::PanicErrorListener;
use crate::interpreter::value::Value;

#[derive(Default, Debug)]
pub struct MagiInterpreter {
    res: Value,
    memory: HashMap<String, Value>
}

impl MagiInterpreter {
    /// Create a new instance of the interpreter
    pub fn new() -> Self {
        MagiInterpreter::default()
    }

    /// Parses the input file written in the Magi language.
    pub fn parse(input: &str) -> Rc<BaseParserRuleContext<'_, MainContextExt<'_>>> {
        let input = InputStream::new(input.trim());

        // Create a TokenSource from the CharStream using the Magi grammar
        let mut lexer = MagiLexer::new(input);
        lexer.remove_error_listeners();
        lexer.add_error_listener(Box::new(PanicErrorListener {}));

        // Obtain the tokens from the TokenSource as a TokenStream
        let tokens = CommonTokenStream::new(lexer);

        // Create a parser that parses the Magi grammar
        let mut parser = MagiParser::new(tokens);
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(PanicErrorListener {}));

        // Execute the grammar from the 'main' nonterminal symbol and prints the tree
        let tree = parser.main().unwrap();
        println!("{}", tree.to_string_tree(&*parser));

        tree
    }
}

impl ParseTreeVisitorCompat<'_> for MagiInterpreter {
    type Node = MagiParserContextType;
    type Return = Value;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.res
    }

    fn visit_error_node(&mut self, _node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        panic!("Error encountered: {}", _node.symbol)
    }
}

impl MagiVisitorCompat<'_> for MagiInterpreter {
    #[doc = "\n\t * Visit a parse tree produced by {@link MagiParser#main}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_main(&mut self, ctx: &MainContext<'_>) -> Self::Return {
        self.visit(&*ctx.com().unwrap())
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code print}\n\t * labeled alternative in {@link MagiParser#com}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_print(&mut self,ctx: &PrintContext<'_>) -> Self::Return {
        let val = self.visit(&*ctx.exp().unwrap());
        println!("{}", val);
        return Value::Void
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code var}\n\t * labeled alternative in {@link MagiParser#com}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_var(&mut self,ctx: &VarContext<'_>) -> Self::Return {
        let id = ctx.VAR().unwrap().get_text();
        let val = self.visit(&*ctx.exp().unwrap());
        self.memory.insert(id, val);
        return Value::Void
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code while}\n\t * labeled alternative in {@link MagiParser#com}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_while(&mut self, ctx: &WhileContext<'_>) -> Self::Return {

        let cond = self.visit(&*ctx.exp().unwrap());

        if let Value::Bool(true) = cond {
            self.visit(&*ctx.com().unwrap());
            self.visit_while(ctx);
        }

        Value::Void
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code if}\n\t * labeled alternative in {@link MagiParser#com}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_if(&mut self,ctx: &IfContext<'_>) -> Self::Return {
        let cond = self.visit(&*ctx.exp().unwrap());

        if let Value::Bool(true) = cond {
            self.visit(&*ctx.com().unwrap());
        }

        Value::Void
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code ifElse}\n\t * labeled alternative in {@link MagiParser#com}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_ifElse(&mut self,ctx: &IfElseContext<'_>) -> Self::Return {
        let cond = self.visit(&*ctx.exp().unwrap());

        match cond {
            Value::Bool(cond) => {
                if cond
                {
                    self.visit(&*ctx.com(0).unwrap())
                } else {
                    self.visit(&*ctx.com(1).unwrap())
                }
            },
            _ => { panic!("TypeMismatch for if else at {}", ctx.start().get_line()); }
        }
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code seq}\n\t * labeled alternative in {@link MagiParser#com}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_seq(&mut self,ctx: &SeqContext<'_>) -> Self::Return {
        self.visit(&*ctx.com(0).unwrap());
        self.visit(&*ctx.com(1).unwrap())
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code andOr}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_andOr(&mut self,ctx: &AndOrContext<'_>) -> Self::Return {
        let left = self.visit(&*ctx.exp(0).unwrap());
        let right = self.visit(&*ctx.exp(1).unwrap());

        left & right
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code not}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_not(&mut self,ctx: &NotContext<'_>) -> Self::Return {
        let exp = self.visit(&*ctx.exp().unwrap());

        !exp
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code eqExp}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_eqExp(&mut self,ctx: &EqExpContext<'_>) -> Self::Return {
        let left = self.visit(&*ctx.exp(0).unwrap());
        let right = self.visit(&*ctx.exp(1).unwrap());

        match ctx.op.clone().unwrap().token_type {
            EQ => {
                return Value::Bool(left == right)
            },
            NEQ => {
                return Value::Bool(left != right)
            },
            _ => { panic!("TypeMismatch for if else at {}", ctx.start().get_line()); }
        }
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code access}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_access(&mut self,ctx: &AccessContext<'_>) -> Self::Return {

        let id = ctx.VAR().unwrap().get_text();

        if !self.memory.contains_key(&id) {
            panic!("Variable used but not initialized at {}", ctx.start().get_line());
        }

        *self.memory.get(&id).unwrap()
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code bool}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_bool(&mut self,ctx: &BoolContext<'_>) -> Self::Return {
        Value::Bool(ctx.get_text().parse::<bool>().unwrap())
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code cmpExp}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_cmpExp(&mut self,ctx: &CmpExpContext<'_>) -> Self::Return {
        let left = self.visit(&*ctx.exp(0).unwrap());
        let right = self.visit(&*ctx.exp(1).unwrap());

        match ctx.op.clone().unwrap().token_type {
            LTE => {
                return Value::Bool(left <= right)
            },
            LT => {
                return Value::Bool(left < right)
            },
            GTE => {
                return Value::Bool(left >= right)
            },
            GT => {
                return Value::Bool(left > right)
            },
            _ => { panic!("TypeMismatch for if else at {}", ctx.start().get_line()); }
        }
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code arith2}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_arith2(&mut self,ctx: &Arith2Context<'_>) -> Self::Return {
        let left = self.visit(&*ctx.exp(0).unwrap());
        let right = self.visit(&*ctx.exp(1).unwrap());

        match ctx.op.clone().unwrap().token_type {
            MUL => {
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => return Value::Int(left * right),
                    (Value::Float(left), Value::Float(right)) => return Value::Float(left * right),
                    _ => { panic!("TypeMismatch for if else at {}", ctx.start().get_line()); }
                }
            },
            DIV => {
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => return Value::Int(left / right),
                    (Value::Float(left), Value::Float(right)) => return Value::Float(left / right),
                    _ => { panic!("TypeMismatch for if else at {}", ctx.start().get_line()); }
                }
            },
            MOD => {
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => return Value::Int(left % right),
                    (Value::Float(left), Value::Float(right)) => return Value::Float(left % right),
                    _ => { panic!("TypeMismatch for if else at {}", ctx.start().get_line()); }
                }
            },
            _ => { panic!("TypeMismatch for if else at {}", ctx.start().get_line()); }
        }
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code pow}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_pow(&mut self,ctx: &PowContext<'_>) -> Self::Return {
        let left = self.visit(&*ctx.exp(0).unwrap());
        let right = self.visit(&*ctx.exp(1).unwrap());

        Value::pow(left, right)
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code arith1}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_arith1(&mut self,ctx: &Arith1Context<'_>) -> Self::Return {
        let left = self.visit(&*ctx.exp(0).unwrap());
        let right = self.visit(&*ctx.exp(1).unwrap());

        match ctx.op.clone().unwrap().token_type {
            ADD => {
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => return Value::Int(left + right),
                    (Value::Float(left), Value::Float(right)) => return Value::Float(left + right),
                    _ => { panic!("TypeMismatch for if else at {}", ctx.start().get_line()); }
                }
            },
            SUB => {
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => return Value::Int(left - right),
                    (Value::Float(left), Value::Float(right)) => return Value::Float(left - right),
                    _ => { panic!("TypeMismatch for if else at {}", ctx.start().get_line()); }
                }
            },
            _ => { panic!("TypeMismatch for if else at {}", ctx.start().get_line()); }
        }
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code float}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_float(&mut self,ctx: &FloatContext<'_>) -> Self::Return {
        Value::Float(ctx.get_text().parse::<f32>().unwrap())
    }

    #[doc = "\n\t * Visit a parse tree produced by the {@code int}\n\t * labeled alternative in {@link MagiParser#exp}.\n\t * @param ctx the parse tree\n\t "]
    fn visit_int(&mut self,ctx: &IntContext<'_>) -> Self::Return {
        Value::Int(ctx.get_text().parse::<i32>().unwrap())
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment() {

        let mut intp = MagiInterpreter::new();
        let program = "a := 5;";
        let tree = MagiInterpreter::parse(program);

        intp.visit(&*tree);

        assert_eq!(Value::Int(5), *intp.memory.get("a").unwrap())
    }


    #[test]
    fn test_while() {
        let mut intp = MagiInterpreter::new();
        let program = "a := 10; while(a > 0) { a := a - 1; } ";
        let tree = MagiInterpreter::parse(program);

        intp.visit(&*tree);

        assert_eq!(Value::Int(0), *intp.memory.get("a").unwrap())
    }


    #[test]
    fn test_if() {
        let mut intp = MagiInterpreter::new();
        let program = "if (true) { a := 5; } ";
        let tree = MagiInterpreter::parse(program);

        intp.visit(&*tree);

        assert_eq!(Value::Int(5), *intp.memory.get("a").unwrap())
    }

    #[test]
    fn test_if_else() {
        let mut intp = MagiInterpreter::new();
        let program = "if (false) { a := 5; } else { a := 2; }";
        let tree = MagiInterpreter::parse(program);

        intp.visit(&*tree);

        assert_eq!(Value::Int(2), *intp.memory.get("a").unwrap())
    }
}

pub mod ast;

peg! grammar(r#"
use super::ast::{Value, Expression, AssignmentExpression, BinaryOp, BinaryExpression,
			ConditionalExpression, NegExpression, NotExpression, RangeExpression,
			VectorExpression, IndexExpression, Postfix, IdentifierExpression, Statement};

space = ' '
end_of_line = '\n'
digit = [0-9]
doublequote = '\"'
backslash = '\\'
quote = '\''

identifier -> String
    = !keyword [a-zA-Z_] [a-zA-Z0-9_]* { match_str.to_string() }

keyword
    = "mod" / "if" / "function" / "for" / "cube" / "sphere"
      / "cylinder" / "true" / "false" / "undef"

spacing
    = (space / end_of_line / comment)*

comment
    = "//" (!end_of_line .)* end_of_line

string_literal -> String
    = doublequote c:(DQChar)* doublequote { c.join("") }

DQChar -> String
    = escape_sequence
      / !doublequote . { match_str.to_string() }

escape_sequence -> String
    = backslash ( quote
                  / doublequote
                  /  backslash
                  / [abfnrtv]
			  ) { match_str[1..].to_string() }
integer
    = digit+

sign
    = '-' / '+'

float_literal -> f64
	= sign? integer ("." integer?)? (("e" / "E") sign? integer)? { match_str.parse().unwrap() }
	/ sign? "." integer (("e" / "E") sign? integer)? { match_str.parse().unwrap() }

bool_literal -> bool
	= "true" { true }
	/ "false" { false }

undef_literal = "undef"

range -> Box<Expression>
	= '[' s:expression spacing ':' i:expression spacing ':' e:expression spacing ']' {
		Box::new(RangeExpression{start: s, increment: i, end: e})
	}
	/ '[' s:expression spacing ':' e:expression spacing ']' {
		Box::new(RangeExpression{start: s, increment: Box::new(Value::Number(1.)), end: e})
	}

vector -> Box<Expression>
	= '[' v:expression ** (spacing ',') ']'  { Box::new(VectorExpression{v: v}) }

postfix -> Postfix
	= '[' e:expression ']' { Postfix::Index(e) }
    / '(' e:assignment_expression ** (spacing ',') spacing')' {
		Postfix::Call(e)
	}

postfix_expression -> Box<Expression>
	= e:primary_expression p:postfix* {
		let mut c = e;
		for pe in p {
			match pe {
				Postfix::Index(ref ind) => {
					c = Box::new(IndexExpression{index: ind.clone(), ex: c})
				},
				Postfix::Call(ref v) => c = Box::new(Value::Undef),
			}
		}
		c
	}

primary_expression -> Box<Expression>
	= i:identifier { Box::new(IdentifierExpression{id: i}) }
	/ s:string_literal { Box::new(Value::String(s)) }
	/ f:float_literal { Box::new(Value::Number(f)) }
	/ b:bool_literal { Box::new(Value::Bool(b)) }
	/ range
	/ vector
	/ undef_literal { Box::new(Value::Undef) }
	/ '(' e:expression ')' { e }

#[pub]
expression -> Box<Expression>
 	= spacing assignment_expression

unary_expression -> Box<Expression>
	= postfix_expression
	/ "!" e:unary_expression { Box::new(NotExpression{ex: e}) }
	/ "+" unary_expression
	/ "-" e:unary_expression { Box::new(NegExpression{ex: e}) }

assignment_expression -> Box<Expression>
 	= id:identifier spacing "=" spacing ce:conditional_expression {
		Box::new(AssignmentExpression{ id: id, ex: ce})
	}
    / conditional_expression

conditional_expression -> Box<Expression>
	= l:logical_or_expression '?' e:expression ':' alt:conditional_expression {
		Box::new(ConditionalExpression{ cond: l, ex: e, alt_ex: alt })
	}
	/ logical_or_expression

mul_op -> (BinaryOp, Box<Expression>)
	= '*' e:multiplicative_expression { (BinaryOp::MUL, e) }
	/ '/' e:multiplicative_expression { (BinaryOp::DIV, e) }
	/ '%' e:multiplicative_expression { (BinaryOp::MOD, e) }

multiplicative_expression -> Box<Expression>
	= s:unary_expression r:mul_op* {
		let mut c = s;
		for (o, e) in r {
			c = Box::new(BinaryExpression{op: o, a: c, b: e})
		}
		c
	}

add_op -> (BinaryOp, Box<Expression>)
	= '+' e:additive_expression { (BinaryOp::ADD, e) }
	/ '-' e:additive_expression { (BinaryOp::SUB, e) }

additive_expression -> Box<Expression>
	= s:multiplicative_expression r:add_op* {
		let mut c = s;
		for (o, e) in r {
			c = Box::new(BinaryExpression{op: o, a: c, b: e})
		}
		c
	}

rel_op -> (BinaryOp, Box<Expression>)
	= ">=" e:relational_expression { (BinaryOp::GE, e) }
	/ "<=" e:relational_expression { (BinaryOp::LE, e) }
	/ '>'  e:relational_expression { (BinaryOp::GT, e) }
	/ '<'  e:relational_expression { (BinaryOp::LT, e) }

relational_expression -> Box<Expression>
	= s:additive_expression r:rel_op* {
		let mut c = s;
		for (o, e) in r {
			c = Box::new(BinaryExpression{op: o, a: c, b: e})
		}
		c
	}

eq_op -> (BinaryOp, Box<Expression>)
	= "==" e:equality_expression { (BinaryOp::EQ, e) }
	/ "!=" e:equality_expression { (BinaryOp::NE, e) }
equality_expression -> Box<Expression>
	= s:relational_expression r:eq_op* {
		let mut c = s;
		for (o, e) in r {
			c = Box::new(BinaryExpression{op: o, a: c, b: e})
		}
		c
	}
logical_and_expression -> Box<Expression>
	= s:equality_expression r:("&&" logical_and_expression)* {
		let mut c = s;
		for e in r {
			c = Box::new(BinaryExpression{op: BinaryOp::AND, a: c, b: e})
		}
		c
	}
logical_or_expression -> Box<Expression>
	= s:logical_and_expression r:("||" logical_or_expression)* {
		let mut c = s;
		for e in r {
			c = Box::new(BinaryExpression{op: BinaryOp::OR, a: c, b: e})
		}
		c
	}

#[pub]
program -> Box<Statement>
	= l:statement_list { Box::new(Statement::CompoundStatement(l)) }

statement -> Box<Statement>
	= compound_statement
    / expression_statement

expression_statement -> Box<Statement>
 	= e:expression? ';' {
		match e {
			Some(ex) => Box::new(Statement::ExpressionStatement(ex)),
			None     => Box::new(Statement::CompoundStatement(vec![])),
		}
	}

compound_statement -> Box<Statement>
	= '{' spacing '}' { Box::new(Statement::CompoundStatement(vec![])) }
    / '{' spacing l:statement_list spacing '}' {
		Box::new(Statement::CompoundStatement(l))
	}

statement_list -> Vec<Box<Statement>>
	= s:statement ++ spacing { s }


"#);









#[cfg(test)]
mod tests {
	use super::ast::*;
	use super::grammar::*;
	use std::collections::HashMap;

	fn assert_ex_eq(ex: &'static str, v: Value) {
		let mut hm = HashMap::new();
		let pex = expression(ex);
		assert!(pex.is_ok(), format!("{:?} while parsing {:?}", pex, ex));
		let pex = pex.unwrap();
		let mut out = ::std::io::stdout();
		let result = pex.eval(&mut hm, &mut out);
		assert!(v == result, format!("{:?} == {:?} [{:?}]", v, result, pex));
	}

    #[test]
    fn expressions() {
		assert_ex_eq("undef", Value::Undef);
		assert_ex_eq("true", Value::Bool(true));
		assert_ex_eq("false", Value::Bool(false));
		assert_ex_eq("!false", Value::Bool(true));
		assert_ex_eq("-12345.6e-2", Value::Number(-123.456));
		assert_ex_eq("UnknownIdentifier", Value::Undef);
		assert_ex_eq("[1,undef,\"foo\" , bar]", Value::Vector(vec![Value::Number(1.), Value::Undef, Value::String("foo".to_string()), Value::Undef]));
		assert_ex_eq("[1:10]", Value::Range(1., 1., 10.));
		assert_ex_eq("[1 : 10:30]", Value::Range(1., 10., 30.));
		assert_ex_eq("\"bar\"", Value::String("bar".to_owned()));
		assert_ex_eq("[0,10,20,30][2]", Value::Number(20.));
		assert_ex_eq("[0,10,20,30][-2]", Value::Undef);
		assert_ex_eq("[0,10,20,30][22]", Value::Undef);
		assert_ex_eq("[0,10,20,30][\"foo\"]", Value::Undef);
		assert_ex_eq("\"foobar\"[1+2*.5+1.2]", Value::String("b".to_owned()));
    }

	fn assert_pgm_eq(pgm: &'static str, v: Value) {
		let ppgm = program(pgm);
		assert!(ppgm.is_ok(), format!("{:?} while parsing {:?}", ppgm, pgm));
		let ppgm = ppgm.unwrap();
		let mut out = ::std::io::stdout();
		let (result, _) = ppgm.execute(&mut out);
		assert!(v == result, format!("{:?} == {:?} [{:?}]", v, result, ppgm));
	}

	#[test]
    fn programs() {
		assert_pgm_eq("Unknown;", Value::Undef);
		assert_pgm_eq("1;", Value::Number(1.));
		assert_pgm_eq("27*3+17;", Value::Number(27.*3.+17.));
		assert_pgm_eq("foo=17;bar = 3.5; { bar = 100; }foo+bar;", Value::Number(20.5));
	}

}

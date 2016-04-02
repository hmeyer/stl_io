pub mod ast;

peg! grammar(r#"
use super::ast::{Value, Expression, AssignmentExpression, BinaryOp, BinaryExpression,
			ConditionalExpression, NegExpression, NotExpression, RangeExpression,
			VectorExpression, IdentifierExpression };

space = " "
end_of_line = "\n"
digit = [0-9]
doublequote = "\""
backslash = "\\"
quote = "\'"

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
    = "-" / "+"

float_literal -> f64
	= sign? integer ("." integer?)? (("e" / "E") sign? integer)? { match_str.parse().unwrap() }
	/ sign? "." integer (("e" / "E") sign? integer)? { match_str.parse().unwrap() }

bool_literal -> bool
	= "true" { true }
	/ "false" { false }

undef_literal = "undef"

range -> Box<Expression>
	= "[" s:expression ":" i:expression ":" e:expression "]" {
		Box::new(RangeExpression{start: s, increment: i, end: e})
	}
	/ "[" s:expression ":" e:expression "]" {
		Box::new(RangeExpression{start: s, increment: Box::new(Value::Number(1.)), end: e})
	}

vector -> Box<Expression>
	= "[" v:expression ** "," "]"  { Box::new(VectorExpression{v: v}) }

postfix_expression -> Box<Expression>
	= primary_expression //( '[' expression ']'
//    	/ '(' ')'
//    	/ '(' argument_expressionList ')'
//	)* { Box::new(Value::Undef) }

//argument_expressionList
//	= assignment_expression (',' assignment_expression)*

#[pub]
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
 	= assignment_expression

expression_statement
 	= expression? ';'

unary_expression -> Box<Expression>
	= postfix_expression
	/ "!" e:unary_expression { Box::new(NotExpression{ex: e}) }
	/ "+" unary_expression
	/ "-" e:unary_expression { Box::new(NegExpression{ex: e}) }

statement
	= compound_statement
    / expression_statement

statement_list = statement (spacing statement)*

compound_statement
	= '{' '}'
    / '{' statement_list '}'

assignment_expression -> Box<Expression>
 	= id:identifier "=" ce:conditional_expression {
		Box::new(AssignmentExpression{ id: id, ex: ce})
	}
    / conditional_expression

conditional_expression -> Box<Expression>
	= l:logical_or_expression '?' e:expression ':' alt:conditional_expression {
		Box::new(ConditionalExpression{ cond: l, ex: e, alt_ex: alt })
	}
	/ logical_or_expression

mul_op -> (BinaryOp, Box<Expression>)
	= "*" e:multiplicative_expression { (BinaryOp::MUL, e) }
	/ "/" e:multiplicative_expression { (BinaryOp::DIV, e) }
	/ "%" e:multiplicative_expression { (BinaryOp::MOD, e) }

multiplicative_expression -> Box<Expression>
	= s:unary_expression r:mul_op* {
		let mut c = s;
		for (o, e) in r {
			c = Box::new(BinaryExpression{op: o, a: c, b: e})
		}
		c
	}

add_op -> (BinaryOp, Box<Expression>)
	= "+" e:additive_expression { (BinaryOp::ADD, e) }
	/ "-" e:additive_expression { (BinaryOp::SUB, e) }

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
	/ ">"  e:relational_expression { (BinaryOp::GT, e) }
	/ "<"  e:relational_expression { (BinaryOp::LT, e) }

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
"#);









#[cfg(test)]
mod tests {
	use super::ast::*;
	use super::grammar::*;
	use std::collections::HashMap;

	#[test]
	fn value() {
		assert_eq!(Value::Undef, Value::Undef);

		let t = Value::Bool(true);
		let f = Value::Bool(false);
		assert_eq!(t, Value::Bool(true));
		assert!(t != f);

		let n1 = Value::Number(1.);
		let n2 = Value::Number(17.);
		assert_eq!(n1, Value::Number(1.));
		assert!(n1 != n2);
/*
		let r1 = Value::Range(1., 2., 3.);
		let r2 = Value::Range(4., 5., 6.);
		assert_eq!(r1.clone(), r1.clone());
		assert!(r1 != r2);

		assert_eq!(Value::Vector(Vec::new()), Value::Vector(Vec::new()));
		assert_eq!(Value::Vector(Vec::<Value>::from([t, f, n1, n2, r1, r2])),
		           Value::Vector(Vec::new(t, f, n1, n2, r1, r2)));
		assert!(Value::Vector([t, f, n1, n2, r1, r2]) != Value::Vector([t, n1, f, n2, r1, r2]));
		assert!(Value::Vector([t, f, n1]) != Value::Vector([t, f, n1, n2]));

		assert!(t != Value::Undef);
		assert!(f != Value::Undef);
		assert!(r1 != Value::Undef);
		assert!(Value::Vector(Vec::new()) != Value::Undef);
*/
	}

	#[test]
	fn sums() {
		/*
		let mut hm = HashMap::new();
		assert_eq!(ExprSum{ a: Box::new(Value::Number(1.)),
			                b: Box::new(Value::Number(3.))}.eval(&hm),
				   Value::Number(4.));
*/
/*
	    let mut v1 =
		    vec![Value::Number(17.), Value::Bool(true), Value::Vector(vec![]), Value::Number(5.)];
		let mut v2 = v1.clone();
		v2.push(Value::Undef);

		let mut vs =
		    vec![Value::Number(34.), Value::Undef, Value::Vector(vec![]), Value::Number(10.)];

		assert_eq!(ExprSum{ a: Box::new(Value::Vector(v1.clone())),
			                b: Box::new(Value::Vector(v2.clone()))}.eval(&hm),
				   Value::Vector(vs.clone()));

	   v1.push(Value::Number(1.));
	   v1.push(Value::Number(1.));
	   vs.push(Value::Undef);

	   assert_eq!(ExprSum{ a: Box::new(Value::Vector(v1)),
						   b: Box::new(Value::Vector(v2))}.eval(&hm),
				  Value::Vector(vs));
*/
	}

    #[test]
    fn parsing() {
		let mut hm = HashMap::new();

		assert_eq!(primary_expression("undef").unwrap().eval(&mut hm), Value::Undef);
		assert_eq!(primary_expression("true").unwrap().eval(&mut hm), Value::Bool(true));
		assert_eq!(primary_expression("false").unwrap().eval(&mut hm), Value::Bool(false));
//		assert_eq!(primary_expression("!false").unwrap().eval(&hm), Value::Bool(true));
		assert_eq!(primary_expression("-12345.6e-2").unwrap().eval(&mut hm), Value::Number(-123.456));
//		assert_eq!(primary_expression("[1:10]"), Ok(Value::Range(1., 10., 1.)));
//		assert_eq!(primary_expression("[1:10:3]"), Ok(Value::Range(1., 10., 3.)));
		assert_eq!(primary_expression("\"bar\"").unwrap().eval(&mut hm), Value::String("bar".to_owned()));

		let ex = expression("[1:foo]");
		println!("{:?}", ex);
		println!("{:?} {:?}", ex.unwrap().eval(&mut hm), hm);
//		assert_eq!(ex.err(), None);
//		assert_eq!(ex.unwrap().eval(&mut hm), Value::Number(19.));


/*
		assert_eq!(primary_expression("[]").unwrap().eval(&hm), Ok(Value::Vector(vec![])));
//		assert_eq!(primary_expression("[undef]"), Ok(Value::Vector(vec![Value::Undef])));
//		assert_eq!(primary_expression("[undef, undef]"),
//		           Ok(Value::Vector(vec![Value::Undef, Value::Undef])));
//	   assert_eq!(primary_expression("[undef, undef, undef]"),
//		           Ok(Value::Vector(vec![Value::Undef, Value::Undef, Value::Undef])));
		let mut v = Vec::new();
//		assert_eq!(primary_expression("[]"), Ok(Value::Vector(v.clone())));
		v.push(Value::Number(34.));
//		assert_eq!(primary_expression("[34]"), Ok(Value::Vector(v.clone())));
		v.push(Value::Undef);
//		assert_eq!(primary_expression("[34, undef]"), Ok(Value::Vector(v)));
*/
/*
        assert_eq!(float_literal("1+1"), Ok(2.));
        assert_eq!(float_literal("1-2"), Ok(-1.));
        assert_eq!(float_literal("-3*4+1"), Ok(-11.));
        assert_eq!(float_literal("-3*(4+1)"), Ok(-15.));
        assert!(float_literal("-3*((4+1)").is_err());
*/
    }

}

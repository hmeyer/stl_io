pub mod ast;

peg! grammar(r#"
use std::rc::Rc;
use super::ast::{Value, Expression, AssignmentExpression, ExpressionFn, BinaryFn, LambdaExpression,
			     ConditionalExpression, RangeExpression, CompoundExpression,
			     VectorExpression, IndexExpression, IdentifierExpression,
				 Callable, CallExpression, CallableDefinitionExpression};

space = ' '
end_of_line = '\n'
digit = [0-9]
doublequote = '\"'
backslash = '\\'
quote = '\''

identifier -> String
    = !keyword [a-zA-Z_] [a-zA-Z0-9_]* { match_str.to_string() }

keyword
    = "mod" / "if" / "function" / "for" / "true" / "false" / "undef"

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

index -> Box<Expression>
	= '[' e:expression ']' { e }

index_expression -> Box<Expression>
	= e:primary_expression i:index* {
		let mut c = e;
		for index_ex in i {
			c = Box::new(IndexExpression{index: index_ex, ex: c});
		}
		c
	}

argument -> (String, Box<Expression>)
	= id:identifier spacing '=' e:assignment_expression { (id, e) }
	/ e:assignment_expression { (String::new(), e) }

call_expression -> Box<CallExpression>
	= i:identifier '(' a:argument ** (spacing ',') spacing ')' {
		Box::new(CallExpression{id:i, arguments: a, sub: None})
	}

primary_expression -> Box<Expression>
	= ce:call_expression { ce as Box<Expression> }
	/ i:identifier { Box::new(IdentifierExpression{id: i}) }
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
	= index_expression
	/ "!" e:unary_expression {
		Box::new(LambdaExpression{op: Rc::new(
			|a, _, _| Value::Bool(!a.as_bool())), name: "unary !",
			a: e, b: Box::new(Value::Undef)})
	}
	/ "+" unary_expression
	/ "-" e:unary_expression {
		Box::new(LambdaExpression{op: Rc::new(
			|a, _, msg| match a {
				Value::Number(v) => Value::Number(-v),
				_ => {
					writeln!(msg, "Warning: cannot negate {:?}, using undef.", a).unwrap();
			        Value::Undef
			    },
			}), name: "unary -", a: e, b: Box::new(Value::Undef)})
	}

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

mul_op -> (Rc<BinaryFn>, &'static str, Box<Expression>)
	= '*' e:multiplicative_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| a * b);
		(op, "*", e)
	}
	/ '/' e:multiplicative_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| a / b);
		(op, "/", e)
	}
	/ '%' e:multiplicative_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| a % b);
		(op, "%", e)
	}

multiplicative_expression -> Box<Expression>
	= s:unary_expression r:mul_op* {
		let mut c = s;
		for (op, n, e) in r {
			c = Box::new(LambdaExpression{op: op, name:n, a: c, b: e})
		}
		c
	}

add_op -> (Rc<BinaryFn>, &'static str, Box<Expression>)
	= '+' e:additive_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| a + b);
		(op, "+", e)
	}
	/ '-' e:additive_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| a - b);
		(op, "-", e)
	}

additive_expression -> Box<Expression>
	= s:multiplicative_expression r:add_op* {
		let mut c = s;
		for (op, n, e) in r {
			c = Box::new(LambdaExpression{op: op, name:n, a: c, b: e})
		}
		c
	}

rel_op -> (Rc<BinaryFn>, &'static str, Box<Expression>)
	= ">=" e:relational_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| Value::Bool(a >= b));
		(op, ">=", e)
	}
	/ "<=" e:relational_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| Value::Bool(a <= b));
		(op, "<=", e)
	}
	/ '>'  e:relational_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| Value::Bool(a > b));
		(op, ">", e)
	}
	/ '<'  e:relational_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| Value::Bool(a < b));
		(op, "<", e)
	}

relational_expression -> Box<Expression>
	= s:additive_expression r:rel_op* {
		let mut c = s;
		for (op, n, e) in r {
			c = Box::new(LambdaExpression{op: op, name:n, a: c, b: e})
		}
		c
	}

eq_op -> (Rc<BinaryFn>, &'static str, Box<Expression>)
	= "==" e:equality_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| Value::Bool(a == b));
		(op, "==", e)
	}
	/ "!=" e:equality_expression {
		let op: Rc<BinaryFn> = Rc::new(|a, b, _| Value::Bool(a != b));
		(op, "!=", e)
	}
equality_expression -> Box<Expression>
	= s:relational_expression r:eq_op* {
		let mut c = s;
		for (op, n, e) in r {
			c = Box::new(LambdaExpression{op: op, name:n, a: c, b: e})
		}
		c
	}
logical_and_expression -> Box<Expression>
	= s:equality_expression r:("&&" logical_and_expression)* {
		let mut c = s;
		for e in r {
			c = Box::new(LambdaExpression{op: Rc::new(
				|a, b, _| Value::Bool(a.as_bool() && b.as_bool())), name:"&&", a: c, b: e})
		}
		c
	}
logical_or_expression -> Box<Expression>
	= s:logical_and_expression r:("||" logical_or_expression)* {
		let mut c = s;
		for e in r {
			c = Box::new(LambdaExpression{op: Rc::new(
				|a, b, _| Value::Bool(a.as_bool() || b.as_bool())), name:"||", a: c, b: e})
		}
		c
	}

#[pub]
program -> Box<Expression>
	= l:statement_list { Box::new(CompoundExpression{v: l}) }

statement -> Box<Expression>
	= call_statement
	/ compound_statement
    / expression_statement
	/ function_definition_statement

call_with_sub -> Box<CallExpression>
	= e:call_expression { e }

call_statement -> Box<Expression>
	= main:call_with_sub space sub:expression? ';' {
		let mut cp = main.clone();
		cp.set_sub(sub);
		cp
	}

expression_statement -> Box<Expression>
 	= e:expression ';' { e }

compound_statement -> Box<Expression>
    = '{' spacing l:statement_list spacing '}' {
		Box::new(CompoundExpression{v: l})
	}

single_parameter -> (String, Option<Box<Expression>>)
	= name:identifier default:(spacing '=' expression)? { (name, default) }

parameter_list -> Vec<(String, Option<Box<Expression>>)>
    = '(' spacing list:single_parameter ** (spacing ',') spacing ')' { list }

function_definition_statement -> Box<Expression>
	= "function" spacing id:identifier p:parameter_list spacing "=" spacing st:statement {
		let closure: Rc<ExpressionFn> = Rc::new(move |env, msg| st.eval(env, msg));
		let callable = Callable {interface: p, ex: closure};
		Box::new(CallableDefinitionExpression{id: id, callable: callable})
	}

statement_list -> Vec<Box<Expression>>
	= s:statement ++ spacing { s }
"#);

#[cfg(test)]
mod tests {
	use super::ast::*;
	use super::grammar::*;
	use ::primitive::Object;

	fn assert_ex_eq(ex: &'static str, v: Value) {
		let mut env = Environment::new_with_primitives();
		let pex = expression(ex);
		assert!(pex.is_ok(), format!("{:?} while parsing {:?}", pex, ex));
		let pex = pex.unwrap();
		let mut out = ::std::io::stdout();
		let result = pex.eval(&mut env, &mut out);
		assert!(v == result, format!("{:?} == {:?} [{:?}]", v, result, pex));
	}

    #[test]
    fn expressions() {
		assert_ex_eq("undef", Value::Undef);
		assert_ex_eq("true", Value::Bool(true));
		assert_ex_eq("false", Value::Bool(false));
		assert_ex_eq("!false", Value::Bool(true));
		assert_ex_eq("-[]", Value::Undef);
		assert_ex_eq("false||false", Value::Bool(false));
		assert_ex_eq("false||true", Value::Bool(true));
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
		let mut env = Environment::new_with_primitives();
		let mut out = ::std::io::stdout();
		let result = ppgm.eval(&mut env, &mut out);
		assert!(v == result, format!("{:?} == {:?} [{:?}]", v, result, ppgm));
	}

	#[test]
    fn programs() {
		assert_pgm_eq("Unknown;", Value::Undef);
		assert_pgm_eq("1;", Value::Number(1.));
		assert_pgm_eq("27*3+17;", Value::Number(27.*3.+17.));
		assert_pgm_eq("foo=17;bar = 3.5; { bar = 100; }foo+bar;", Value::Number(20.5));
		assert_pgm_eq("foo();", Value::Undef);
		assert_pgm_eq("baz=3;function foo(x=2+2)=17;bar();baz();foo();", Value::Number(17.));
		assert_pgm_eq("echo(\"foobar\");", Value::Undef);
	}

	#[test]
    fn objects() {
		assert_pgm_eq("sphere(15);",
		                   Value::Objects(vec![Box::new(::primitive::Sphere::new(15.))
						        as Box<::primitive::Object>]));

		let mut sphere = ::primitive::Sphere::new(7.);
		sphere.translate(::types::Vector::new(1., 2., 3.));
		assert_pgm_eq("translate([1,2,3]) sphere(7);",
		                   Value::Objects(vec![Box::new(sphere) as Box<::primitive::Object>]));
	}

}

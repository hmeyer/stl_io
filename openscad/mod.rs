pub mod ast;
pub mod functions;


peg_file! grammar("openscad.rustpeg");

pub use self::grammar::program;

#[cfg(test)]
mod tests {
	use super::ast::*;
	use super::grammar::*;
	use xplicit_primitive::{Object, Sphere};
	use xplicit_types::Vector;

	fn assert_ex_eq(ex: &'static str, v: Value) {
		let mut env = Environment::new();
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

	fn val_to_string(val: &Value) -> String {
		format!("{:?}", val)
	}

	fn assert_pgm_eq(pgm: &'static str, v: Value) {
		let ppgm = program(pgm);
		assert!(ppgm.is_ok(), format!("{:?} while parsing {:?}", ppgm, pgm));
		let ppgm = ppgm.unwrap();
		let mut env = Environment::new();
		let mut out = ::std::io::stdout();
		let result = ppgm.eval(&mut env, &mut out);
			assert!(val_to_string(&v) == val_to_string(&result),
		        format!("{:?} == {:?} [{:?}]", v, result, ppgm));
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
		                   Value::Objects(vec![Sphere::new(15.)]));
		let sphere = Sphere::new(7.).translate(Vector::new(1., 2., 3.));
		assert_pgm_eq("translate([1,2,3]) sphere(7);",
		                   Value::Objects(vec![sphere]));
	}

}

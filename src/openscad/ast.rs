use std::fmt::Debug;
use std::io::Write;
use std::rc::Rc;

pub type OptObject = Option<Box<::primitive::Object>>;
pub type BindMap = ::std::collections::HashMap<String, Binding>;

#[derive(Clone, Debug)]
pub struct FunctionMod {
    pub params: Vec<(String, Option<Box<Expression>>)>,
    pub body: Box<Statement>,
}

#[derive(Clone, Debug)]
pub enum Binding {
    Val(Value),
    Func(FunctionMod),
}


fn unify(a: OptObject, b: OptObject) -> OptObject {
        match a {
            Some(x) => match b {
                Some(y) => Some(Box::new(::primitive::Union::new(x, y))),
                None => Some(x),
            },
            None => match b {
                Some(y) => Some(y),
                None => None,
            }
        }
}

pub trait Expression : ExpressionClone + Debug {
    fn eval(&self, vars: &mut BindMap, msg: &mut Write) -> Value;
}

pub trait ExpressionClone {
    fn clone_box(&self) -> Box<Expression>;
}

impl<T> ExpressionClone for T where T: 'static + Expression + Clone {
    fn clone_box(&self) -> Box<Expression> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<Expression> {
    fn clone(&self) -> Box<Expression> {
        self.clone_box()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
	Undef,
	Bool(bool),
	Number(f64),
	String(String),
	Vector(Vec<Value>),
	Range(f64, f64, f64),
}

impl Value {
	pub fn as_bool(&self) -> bool {
		match self {
            &Value::Undef => false,
			&Value::Bool(b) => b,
			&Value::Number(n) => n != 0.,
            &Value::String(ref s) => s.len() != 0,
            &Value::Vector(ref v) => v.len() != 0,
			&Value::Range(_, _, _) => true,
		}
	}
}

impl Expression for Value {
	fn eval(&self, _: &mut BindMap, _: &mut Write) -> Value {
		self.clone()
	}
}

impl ::std::ops::Add for Value {
	type Output = Value;

    fn add(self, _rhs: Value) -> Value {
        if let Value::Number(sn) = self {
			if let Value::Number(rn) = _rhs {
				return Value::Number(sn + rn);
			}
		}
        if let Value::Vector(sv) = self {
			if let Value::Vector(rv) = _rhs {
                if sv.len() != rv.len() {return Value::Undef;}
                let mut result = Vec::with_capacity(sv.len());
                for i in 0..sv.len() {
                    let sum = sv[i].clone() + rv[i].clone();
                    result.push(sum);
                }
				return Value::Vector(result);
			}
		}
		Value::Undef
    }
}

impl ::std::ops::Sub for Value {
	type Output = Value;

    fn sub(self, _rhs: Value) -> Value {
		if let Value::Number(sn) = self {
			if let Value::Number(rn) = _rhs {
				return Value::Number(sn - rn);
			}
		}
        if let Value::Vector(sv) = self {
			if let Value::Vector(rv) = _rhs {
                if sv.len() != rv.len() {return Value::Undef;}
                let mut result = Vec::with_capacity(sv.len());
                for i in 0..sv.len() {
                    let sum = sv[i].clone() - rv[i].clone();
                    result.push(sum);
                }
				return Value::Vector(result);
			}
		}
		Value::Undef
    }
}

impl ::std::ops::Mul for Value {
	type Output = Value;

    fn mul(self, _rhs: Value) -> Value {
		if let Value::Number(sn) = self {
			if let Value::Number(rn) = _rhs {
				return Value::Number(sn * rn);
			}
		}
        // Implement numerical cross product, if both are vectors of numbers of
        // same length.
        if let Value::Vector(sv) = self {
			if let Value::Vector(rv) = _rhs {
                if sv.len() != rv.len() {return Value::Undef;}
                let mut cross_product = 0.;
                for i in 0..sv.len() {
                    if let Value::Number(sn) = sv[i] {
            			if let Value::Number(rn) = rv[i] {
                            cross_product += sn * rn;
                            continue;
                        }
                    }
                    return Value::Undef;
                }
				return Value::Number(cross_product);
			}
		}
		Value::Undef
    }
}

impl ::std::ops::Div for Value {
	type Output = Value;

    fn div(self, _rhs: Value) -> Value {
		if let Value::Number(sn) = self {
			if let Value::Number(rn) = _rhs {
				return Value::Number(sn / rn);
			}
		}
		Value::Undef
    }
}

impl ::std::ops::Rem for Value {
	type Output = Value;

    fn rem(self, _rhs: Value) -> Value {
		if let Value::Number(sn) = self {
			if let Value::Number(rn) = _rhs {
				return Value::Number(sn % rn);
			}
		}
		Value::Undef
    }
}

#[derive(Clone, Debug)]
pub struct AssignmentExpression {
	pub id: String,
	pub ex: Box<Expression>,
}

impl Expression for AssignmentExpression {
	fn eval(&self, vars: &mut BindMap, msg: &mut Write) -> Value {
		let v = self.ex.eval(vars, msg);
		vars.insert(self.id.clone(), Binding::Val(v.clone()));
		v
	}
}

#[derive(Clone, Debug)]
pub struct IdentifierExpression {
	pub id: String,
}

impl Expression for IdentifierExpression {
	fn eval(&self, vars: &mut BindMap, msg: &mut Write) -> Value {
		match vars.get(&self.id) {
            Some(&Binding::Val(ref x)) => x.clone(),
            Some(&Binding::Func(_)) => {
                writeln!(msg, "Warning: {:?} is a function or mod, using undef.",
                         self.id).unwrap();
                Value::Undef
            },
			None => {
                writeln!(msg,
                         "Warning: unknown {:?}, using undef.", self.id).unwrap();
                Value::Undef
            },
		}
	}
}

#[derive(Clone, Debug)]
pub struct ConditionalExpression {
	pub cond: Box<Expression>,
	pub ex: Box<Expression>,
	pub alt_ex: Box<Expression>,
}

impl Expression for ConditionalExpression {
	fn eval(&self, vars: &mut BindMap, msg: &mut Write) -> Value {
		if self.cond.eval(vars, msg).as_bool() {
			self.ex.eval(vars, msg)
		} else {
			self.alt_ex.eval(vars, msg)
		}
	}
}

pub type BinaryFn = Fn(Value, Value, &mut Write) -> Value;

#[derive(Clone)]
pub struct LambdaExpression {
    pub op: Rc<BinaryFn>,
    pub name: &'static str,
    pub a: Box<Expression>,
	pub b: Box<Expression>,
}

impl ::std::fmt::Debug for LambdaExpression {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "LambdaExpression[{:?}] {{ a:{:?} b:{:?} }}",
               self.name, self.a, self.b)
    }
}

impl Expression for LambdaExpression {
    fn eval(&self, vars: &mut BindMap, msg: &mut Write) -> Value {
		(self.op)(self.a.eval(vars, msg), self.b.eval(vars, msg), msg)
    }
}

#[derive(Clone, Debug)]
pub struct VectorExpression {
	pub v: Vec<Box<Expression>>,
}

impl Expression for VectorExpression {
	fn eval(&self, vars: &mut BindMap, msg: &mut Write) -> Value {
		Value::Vector(self.v.iter().map(|x| x.eval(vars, msg)).collect())
	}
}

#[derive(Clone, Debug)]
pub struct IndexExpression {
    pub index: Box<Expression>,
    pub ex: Box<Expression>,
}

impl Expression for IndexExpression {
	fn eval(&self, vars: &mut BindMap, msg: &mut Write) -> Value {
        let i: usize;
        let val = self.index.eval(vars, msg);
        match val {
            Value::Number(n) => i = n as usize,
            _ => {
                writeln!(msg, "Warning: invalid index: {:?}, using undef.",
                         val).unwrap();
                return Value::Undef
            },
        }
        match self.ex.eval(vars, msg) {
            Value::Vector(ref v) => {
                match v.get(i) {
                    Some(x) => x.clone(),
                    None => {
                        writeln!(msg, "Warning: index: out of bounds ({:?} vs \
                                {:?}), using undef.", i, v.len()).unwrap();
                        Value::Undef
                    },
                }
            },
            Value::String(ref s) => {
                match s.chars().nth(i) {
                    Some(x) => Value::String(x.to_string()),
                    None => {
                        writeln!(msg, "Warning: index: out of bounds ({:?} vs \
                                 {:?}), using undef.", i, s.len()).unwrap();
                        Value::Undef
                    },
                }
            },
            _ => Value::Undef,
        }
	}
}

#[derive(Clone, Debug)]
pub struct CallExpression {
    pub id: String,
    pub arguments: Vec<(String, Box<Expression>)>,
    pub sub: Option<Box<Statement>>,
}

impl Expression for CallExpression {
    fn eval(&self, vars: &mut BindMap, msg: &mut Write) -> Value {
        let (val, _) = self.execute_impl(vars, msg);
        val
    }
}

impl CallExpression {
    fn set_sub(&mut self, sub: Option<Box<Statement>>) {
        self.sub = sub;
    }
	fn execute_impl(&self, vars: &mut BindMap, msg: &mut Write) -> (Value, OptObject) {
        match vars.get(&self.id) {
            Some(&Binding::Func(FunctionMod{ref params, ref body })) => {
                let mut vars_copy = vars.clone();
                let mut args = self.arguments.clone();
                for &(ref p_name, ref opt_def_ex) in params {
                    let mut found = false;
                    // try to find parameter as named parameter
                    for &mut (ref a_name, ref ex) in &mut args {
                        if p_name == a_name {
                            let v = ex.eval(&mut vars_copy, msg);
                            vars_copy.insert(a_name.clone(), Binding::Val(v));
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        // take first anonymous parameter
                        for &mut (ref mut a_name, ref ex) in &mut args {
                            if a_name.is_empty() {
                                let v = ex.eval(&mut vars_copy, msg);
                                vars_copy.insert(p_name.clone(), Binding::Val(v));
                                *a_name = "0".to_string();  // de-anonymize
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        // take default, if any. return undef else.
                        if let &Some(ref def_ex) = opt_def_ex {
                            let v = def_ex.eval(&mut vars_copy, msg);
                            vars_copy.insert(p_name.clone(), Binding::Val(v));
                        } else {
                            writeln!(msg, "parameter {:?} not found for {:?}",
                                     p_name, self.id).unwrap();
                            return (Value::Undef, None);
                        }
                    }
                }
                return body.execute_impl(&mut vars_copy, msg);
            },
            Some(&Binding::Val(_)) => writeln!(
                 msg, "Warning: non callable {:?}, using undef.", self.id).unwrap(),
			None => writeln!(msg, "Warning: unknown {:?}, using undef.",
                             self.id).unwrap(),
        }
        (Value::Undef, None)
	}
}

#[derive(Clone, Debug)]
pub struct RangeExpression {
	pub start: Box<Expression>,
	pub increment: Box<Expression>,
	pub end: Box<Expression>,
}

impl Expression for RangeExpression {
	fn eval(&self, vars: &mut BindMap, msg: &mut Write) -> Value {
        let sv = self.start.eval(vars, msg);
		if let Value::Number(s) = sv {
            let iv = self.increment.eval(vars, msg);
			if let Value::Number(i) = iv {
                let ev  = self.end.eval(vars, msg);
				if let Value::Number(e) = ev {
			   		return Value::Range(s, i, e)
				} else {
                    writeln!(msg,
                             "Warning: non-number range end {:?}, using undef.",
                             ev).unwrap();
                }
			} else {
                writeln!(msg,
                         "Warning: non-number range increment {:?}, using undef.",
                         iv).unwrap();
            }
		} else {
            writeln!(msg, "Warning: non-number range start {:?}, using undef.",
                     sv).unwrap();
        }
	   	Value::Undef
	}
}

#[derive(Clone, Debug)]
pub enum Statement {
    ExpressionStatement(Box<Expression>),
    CompoundStatement(Vec<Box<Statement>>),
    FuncModDefinition(String, FunctionMod),
    ModCall(Box<CallExpression>),
}

impl Statement {
    pub fn set_sub(&mut self, sub: Option<Box<Statement>>) -> bool {
        match self {
            &mut Statement::ModCall(ref mut call_ex) => {
                call_ex.set_sub(sub);
                true
            },
            &mut _ => false,
        }
    }
	pub fn execute(&self, msg: &mut Write) -> (Value, OptObject) {
        let mut vars = BindMap::new();
        self.execute_impl(&mut vars, msg)
    }

    pub fn execute_impl(&self, vars: &mut BindMap, msg: &mut Write)
        -> (Value, OptObject) {
		match self {
            &Statement::ExpressionStatement(ref ex) => (ex.eval(vars, msg), None),
			&Statement::CompoundStatement(ref b) => {
                let mut v = Value::Undef;
                let mut o = Option::None;
                let mut vars_copy = vars.clone();
                for ex in b {
                    let (tmp_v, tmp_o)  = ex.execute_impl(&mut vars_copy, msg);
                    v = tmp_v;
                    o = unify(o, tmp_o);
                }
                (v, o)
            },
            &Statement::FuncModDefinition(ref name, ref func) => {
                let expanded_params = func.params.iter().map(
                    |&(ref name, ref opt_ex)| {
                        (name.clone(), match opt_ex {
                            &Some(ref ex) => Some(Box::new(ex.eval(vars, msg)) as Box<Expression>),
                            &None => None,
                        })
                    }).collect();
                let expanded_func = FunctionMod {
                    params: expanded_params,
                    body: func.body.clone()
                };
                if let Some(old) = vars.insert(name.clone(),
                                               Binding::Func(expanded_func)) {
                    writeln!(msg, "Warning: overwriting {:?} with function or mod.",
                             old).unwrap();
                }
                (Value::Undef, Option::None)
            },
            &Statement::ModCall(ref call_ex) => call_ex.execute_impl(vars, msg),
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn value() {
		assert_eq!(Value::Undef, Value::Undef);

		assert_eq!(Value::Bool(true), Value::Bool(true));
		assert!(Value::Bool(true) != Value::Bool(false));

		assert_eq!(Value::Number(1.), Value::Number(1.));
		assert!(Value::Number(17.) != Value::Number(1.));

		assert_eq!(Value::Range(1., 2., 3.), Value::Range(1., 2., 3.));
		assert!(Value::Range(1., 2., 3.) != Value::Range(1., 2., 3.7));

		assert_eq!(Value::Vector(vec![]), Value::Vector(vec![]));
		assert_eq!(Value::Vector(vec![Value::Bool(true), Value::Number(17.), Value::Range(1., 2., 3.), Value::Vector(vec![])]),
		           Value::Vector(vec![Value::Bool(true), Value::Number(17.), Value::Range(1., 2., 3.), Value::Vector(vec![])]));
		assert!(Value::Vector(vec![Value::Bool(true)]) != Value::Vector(vec![Value::Bool(false)]));
		assert!(Value::Vector(vec![Value::Bool(true)]) != Value::Vector(vec![Value::Bool(true), Value::Bool(true)]));

        assert!(Value::Undef != Value::Vector(vec![]));

        assert!(!Value::Undef.as_bool());
        assert!(Value::Bool(true).as_bool());
        assert!(!Value::Bool(false).as_bool());
        assert!(!Value::Number(0.).as_bool());
        assert!(Value::Number(-17.).as_bool());
        assert!(!Value::String("".to_string()).as_bool());
        assert!(Value::String("foo".to_string()).as_bool());
        assert!(!Value::Vector(vec![]).as_bool());
        assert!(Value::Vector(vec![Value::Undef]).as_bool());
        assert!(Value::Range(1., 2., 3.).as_bool());
	}

	#[test]
	fn binary_ops() {
        assert_eq!(Value::Undef, Value::Undef + Value::Undef);
        assert_eq!(Value::Number(4.), Value::Number(1.) + Value::Number(3.));
        assert_eq!(Value::Undef,
                  Value::String("foo".to_string()) + Value::String("bar".to_string()));
        assert_eq!(Value::Vector(vec![Value::Number(4.)]),
                   Value::Vector(vec![Value::Number(1.)]) + Value::Vector(vec![Value::Number(3.)]));

        assert_eq!(Value::Vector(vec![Value::Undef, Value::Number(-2.)]),
                   Value::Vector(vec![Value::Number(1.), Value::Number(1.)]) -
			       Value::Vector(vec![Value::String("foo".to_string()), Value::Number(3.)]));

        assert_eq!(Value::Number(1. * 3. + 2. * 4.),
                   Value::Vector(vec![Value::Number(1.), Value::Number(2.)]) *
			       Value::Vector(vec![Value::Number(3.), Value::Number(4.)]));
        assert_eq!(Value::Undef,
                   Value::Vector(vec![Value::Number(1.), Value::Vector(vec![])]) *
			       Value::Vector(vec![Value::Number(3.), Value::Number(4.)]));
        assert_eq!(Value::Number(2.), Value::Number(17.) % Value::Number(3.));
	}
}

use std::fmt::Debug;
use std::io::Write;

pub type OptObject = Option<Box<::primitive::Object>>;
pub type Identifiers = ::std::collections::HashMap<String, Value>;

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
    fn eval(&self, vars: &mut Identifiers, msg: &mut Write) -> Value;
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
	fn as_bool(&self) -> bool {
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
	fn eval(&self, _: &mut Identifiers, _: &mut Write) -> Value {
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
        // Implement numberical cross product, if both are vectors of numbers of
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
	fn eval(&self, hm: &mut Identifiers, msg: &mut Write) -> Value {
		let v = self.ex.eval(hm, msg);
		hm.insert(self.id.clone(), v.clone());
		v
	}
}

#[derive(Clone, Debug)]
pub struct IdentifierExpression {
	pub id: String,
}

impl Expression for IdentifierExpression {
	fn eval(&self, hm: &mut Identifiers, msg: &mut Write) -> Value {
		match hm.get(&self.id) {
			Some(x) => x.clone(),
			None => {
                writeln!(msg,
                         "Warning: unknown {:?}, using undef.", self.id).unwrap();
                Value::Undef
            },
		}
	}
}

#[derive(Clone, Debug)]
pub struct NotExpression {
	pub ex: Box<Expression>,
}

impl Expression for NotExpression {
	fn eval(&self, hm: &mut Identifiers, msg: &mut Write) -> Value {
		Value::Bool(!self.ex.eval(hm, msg).as_bool())
	}
}


#[derive(Clone, Debug)]
pub struct NegExpression {
	pub ex: Box<Expression>,
}

impl Expression for NegExpression {
	fn eval(&self, hm: &mut Identifiers, msg: &mut Write) -> Value {
        let val = self.ex.eval(hm, msg);
		match val {
			Value::Number(v) => Value::Number(-v),
			_ => {
                writeln!(msg, "Warning: cannot negate {:?}, using undef.",
                         val).unwrap();
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
	fn eval(&self, hm: &mut Identifiers, msg: &mut Write) -> Value {
		if self.cond.eval(hm, msg).as_bool() {
			self.ex.eval(hm, msg)
		} else {
			self.alt_ex.eval(hm, msg)
		}
	}
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum BinaryOp { AND, OR, EQ, NE, GT, LT, GE, LE, ADD, SUB, MUL, DIV, MOD }

#[derive(Clone, Debug)]
pub struct BinaryExpression {
	pub op: BinaryOp,
	pub a: Box<Expression>,
	pub b: Box<Expression>,
}

impl Expression for BinaryExpression {
	fn eval(&self, hm: &mut Identifiers, msg: &mut Write) -> Value {
		let va = self.a.eval(hm, msg);
		let vb = self.b.eval(hm, msg);
		match self.op {
			BinaryOp::AND => Value::Bool(va.as_bool() && va.as_bool()),
			BinaryOp::OR => Value::Bool(va.as_bool() || va.as_bool()),
			BinaryOp::EQ => Value::Bool(va == vb),
			BinaryOp::NE => Value::Bool(va != vb),
			BinaryOp::GT => Value::Bool(va > vb),
			BinaryOp::LT => Value::Bool(va < vb),
			BinaryOp::GE => Value::Bool(va >= vb),
			BinaryOp::LE => Value::Bool(va <= vb),
			BinaryOp::ADD => va + vb,
			BinaryOp::SUB => va - vb,
			BinaryOp::MUL => va * vb,
			BinaryOp::DIV => va / vb,
			BinaryOp::MOD => va % vb,
		}
	}
}

#[derive(Clone, Debug)]
pub struct VectorExpression {
	pub v: Vec<Box<Expression>>,
}

impl Expression for VectorExpression {
	fn eval(&self, hm: &mut Identifiers, msg: &mut Write) -> Value {
		Value::Vector(self.v.iter().map(|x| x.eval(hm, msg)).collect())
	}
}

pub enum Postfix {
    Index(Box<Expression>),
    Call(Vec<Box<Expression>>),
}

#[derive(Clone, Debug)]
pub struct IndexExpression {
    pub index: Box<Expression>,
    pub ex: Box<Expression>,
}

impl Expression for IndexExpression {
	fn eval(&self, hm: &mut Identifiers, msg: &mut Write) -> Value {
        let i: usize;
        let val = self.index.eval(hm, msg);
        match val {
            Value::Number(n) => i = n as usize,
            _ => {
                writeln!(msg, "Warning: invalid index: {:?}, using undef.",
                         val).unwrap();
                return Value::Undef
            },
        }
        match self.ex.eval(hm, msg) {
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
pub struct RangeExpression {
	pub start: Box<Expression>,
	pub increment: Box<Expression>,
	pub end: Box<Expression>,
}

impl Expression for RangeExpression {
	fn eval(&self, hm: &mut Identifiers, msg: &mut Write) -> Value {
        let sv = self.start.eval(hm, msg);
		if let Value::Number(s) = sv {
            let iv = self.increment.eval(hm, msg);
			if let Value::Number(i) = iv {
                let ev  = self.end.eval(hm, msg);
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

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Box<Expression>),
    CompoundStatement(Vec<Box<Statement>>),
}

impl Statement {
	pub fn execute(&self, msg: &mut Write) -> (Value, OptObject) {
        let mut hm = Identifiers::new();
        self.execute_impl(&mut hm, msg)
    }

    pub fn execute_impl(&self, hm: &mut Identifiers, msg: &mut Write) -> (Value, OptObject) {
		match self {
            &Statement::ExpressionStatement(ref ex) => (ex.eval(hm, msg), None),
			&Statement::CompoundStatement(ref b) => {
                let mut v = Value::Undef;
                let mut o = Option::None;
                let mut hm_copy = hm.clone();
                for ex in b {
                    let (tmp_v, tmp_o)  = ex.execute_impl(&mut hm_copy, msg);
                    v = tmp_v;
                    o = unify(o, tmp_o);
                }
                (v, o)
            },
		}
	}
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
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
        let mut hm = HashMap::new();
        let mut out = ::std::io::stdout();
        assert_eq!(BinaryExpression{ op: BinaryOp::ADD,
                                     a: Box::new(Value::Undef),
			                         b: Box::new(Value::Undef)}.eval(&mut hm, &mut out),
				   Value::Undef);
        assert_eq!(BinaryExpression{ op: BinaryOp::ADD,
                                     a: Box::new(Value::Number(1.)),
			                         b: Box::new(Value::Number(3.))}.eval(&mut hm, &mut out),
				   Value::Number(4.));
        assert_eq!(BinaryExpression{ op: BinaryOp::ADD,
                                     a: Box::new(Value::String("foo".to_string())),
			                         b: Box::new(Value::String("bar".to_string()))}.eval(&mut hm, &mut out),
				   Value::Undef);
        assert_eq!(BinaryExpression{ op: BinaryOp::ADD,
                                     a: Box::new(Value::Vector(vec![Value::Number(1.)])),
			                         b: Box::new(Value::Vector(vec![Value::Number(3.)]))}.eval(&mut hm, &mut out),
				   Value::Vector(vec![Value::Number(4.)]));

        assert_eq!(BinaryExpression{ op: BinaryOp::SUB,
                                     a: Box::new(Value::Vector(vec![Value::Number(1.), Value::Number(1.)])),
			                         b: Box::new(Value::Vector(vec![Value::String("foo".to_string()), Value::Number(3.)]))}.eval(&mut hm, &mut out),
				   Value::Vector(vec![Value::Undef, Value::Number(-2.)]));

        assert_eq!(BinaryExpression{ op: BinaryOp::MUL,
                                     a: Box::new(Value::Vector(vec![Value::Number(1.), Value::Number(2.)])),
			                         b: Box::new(Value::Vector(vec![Value::Number(3.), Value::Number(4.)]))}.eval(&mut hm, &mut out),
				   Value::Number(1. * 3. + 2. * 4.));
        assert_eq!(BinaryExpression{ op: BinaryOp::MUL,
                                     a: Box::new(Value::Vector(vec![Value::Number(1.), Value::Vector(vec![])])),
			                         b: Box::new(Value::Vector(vec![Value::Number(3.), Value::Number(4.)]))}.eval(&mut hm, &mut out),
				   Value::Undef);
        assert_eq!(BinaryExpression{ op: BinaryOp::MOD,
                                     a: Box::new(Value::Number(17.)),
		   	                         b: Box::new(Value::Number(3.))}.eval(&mut hm, &mut out),
				   Value::Number(2.));
	}
}

use std::collections::HashMap;
use std::fmt::Debug;

pub trait Expression : ExpressionClone + Debug {
    fn eval(&self, vars: &mut HashMap<String, Value>) -> Value;
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
			&Value::Bool(b) => b,
			&Value::Undef => false,
			&Value::Number(n) => n != 0.,
			&Value::Vector(ref v) => v.len() != 0,
			_ => true,
		}
	}
}

impl Expression for Value {
	fn eval(&self, _: &mut HashMap<String, Value>) -> Value {
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
	fn eval(&self, hm: &mut HashMap<String, Value>) -> Value {
		let v = self.ex.eval(hm);
		hm.insert(self.id.clone(), v.clone());
		v
	}
}

#[derive(Clone, Debug)]
pub struct IdentifierExpression {
	pub id: String,
}

impl Expression for IdentifierExpression {
	fn eval(&self, hm: &mut HashMap<String, Value>) -> Value {
		match hm.get(&self.id) {
			Some(x) => x.clone(),
			None => Value::Undef,
		}
	}
}

#[derive(Clone, Debug)]
pub struct NotExpression {
	pub ex: Box<Expression>,
}

impl Expression for NotExpression {
	fn eval(&self, hm: &mut HashMap<String, Value>) -> Value {
		Value::Bool(!self.ex.eval(hm).as_bool())
	}
}


#[derive(Clone, Debug)]
pub struct NegExpression {
	pub ex: Box<Expression>,
}

impl Expression for NegExpression {
	fn eval(&self, hm: &mut HashMap<String, Value>) -> Value {
		match self.ex.eval(hm) {
			Value::Number(v) => Value::Number(-v),
			_ => Value::Undef,
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
	fn eval(&self, hm: &mut HashMap<String, Value>) -> Value {
		if self.cond.eval(hm).as_bool() {
			self.ex.eval(hm)
		} else {
			self.alt_ex.eval(hm)

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
	fn eval(&self, hm: &mut HashMap<String, Value>) -> Value {
		let va = self.a.eval(hm);
		let vb = self.b.eval(hm);
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
	fn eval(&self, hm: &mut HashMap<String, Value>) -> Value {
		Value::Vector(self.v.iter().map(|x| x.eval(hm)).collect())
	}
}

#[derive(Clone, Debug)]
pub struct RangeExpression {
	pub start: Box<Expression>,
	pub increment: Box<Expression>,
	pub end: Box<Expression>,
}

impl Expression for RangeExpression {
	fn eval(&self, hm: &mut HashMap<String, Value>) -> Value {
		if let Value::Number(s) = self.start.eval(hm) {
			if let Value::Number(i) = self.increment.eval(hm) {
				if let Value::Number(e) = self.end.eval(hm) {
			   		return Value::Range(s, i, e)
				}
			}
		}
	   	Value::Undef
	}
}

pub struct Statement {

}

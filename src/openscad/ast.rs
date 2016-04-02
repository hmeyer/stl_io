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
        assert_eq!(BinaryExpression{ op: BinaryOp::ADD,
                                     a: Box::new(Value::Undef),
			                         b: Box::new(Value::Undef)}.eval(&mut hm),
				   Value::Undef);
        assert_eq!(BinaryExpression{ op: BinaryOp::ADD,
                                     a: Box::new(Value::Number(1.)),
			                         b: Box::new(Value::Number(3.))}.eval(&mut hm),
				   Value::Number(4.));
        assert_eq!(BinaryExpression{ op: BinaryOp::ADD,
                                     a: Box::new(Value::String("foo".to_string())),
			                         b: Box::new(Value::String("bar".to_string()))}.eval(&mut hm),
				   Value::Undef);
        assert_eq!(BinaryExpression{ op: BinaryOp::ADD,
                                     a: Box::new(Value::Vector(vec![Value::Number(1.)])),
			                         b: Box::new(Value::Vector(vec![Value::Number(3.)]))}.eval(&mut hm),
				   Value::Vector(vec![Value::Number(4.)]));

        assert_eq!(BinaryExpression{ op: BinaryOp::SUB,
                                     a: Box::new(Value::Vector(vec![Value::Number(1.), Value::Number(1.)])),
			                         b: Box::new(Value::Vector(vec![Value::String("foo".to_string()), Value::Number(3.)]))}.eval(&mut hm),
				   Value::Vector(vec![Value::Undef, Value::Number(-2.)]));

        assert_eq!(BinaryExpression{ op: BinaryOp::MUL,
                                     a: Box::new(Value::Vector(vec![Value::Number(1.), Value::Number(2.)])),
			                         b: Box::new(Value::Vector(vec![Value::Number(3.), Value::Number(4.)]))}.eval(&mut hm),
				   Value::Number(1. * 3. + 2. * 4.));
        assert_eq!(BinaryExpression{ op: BinaryOp::MUL,
                                     a: Box::new(Value::Vector(vec![Value::Number(1.), Value::Vector(vec![])])),
			                         b: Box::new(Value::Vector(vec![Value::Number(3.), Value::Number(4.)]))}.eval(&mut hm),
				   Value::Undef);
        assert_eq!(BinaryExpression{ op: BinaryOp::MOD,
                                     a: Box::new(Value::Number(17.)),
		   	                         b: Box::new(Value::Number(3.))}.eval(&mut hm),
				   Value::Number(2.));
	}
}

use std::fmt::Debug;
use std::io::Write;
use std::rc::Rc;
use primitive::Object;
use super::functions;

pub struct Environment {
    pub vars: ::std::collections::HashMap<String, Binding>,
    pub objs: Vec<Box<::primitive::Object>>,
}

impl Environment {
    fn clone_vars(&self) -> Environment {
        Environment {
            vars: self.vars.clone(),
            objs: vec![],
        }
    }
    pub fn new() -> Environment {
        let mut basic_bindings = ::std::collections::HashMap::new();
        functions::add_bindings(&mut basic_bindings);
        Environment {
            vars: basic_bindings,
            objs: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub enum Binding {
    Val(Value),
    Call(Callable),
}

pub type ExpressionFn = Fn(&mut Environment, &mut Write) -> Value;

#[derive(Clone)]
pub struct Callable {
    pub interface: Vec<(String, Option<Box<Expression>>)>,
    pub ex: Rc<ExpressionFn>,
}

impl ::std::fmt::Debug for Callable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Callable{{ interface:{:?} }}", self.interface)
    }
}


pub trait Expression: ExpressionClone + Debug {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value;
}

pub trait ExpressionClone {
    fn clone_box(&self) -> Box<Expression>;
}

impl<T> ExpressionClone for T
    where T: 'static + Expression + Clone
{
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
    Objects(Vec<Box<Object>>),
}

impl Value {
    pub fn as_f64(&self) -> f64 {
        match self {
            &Value::Number(x) => x,
            _ => ::std::f64::NAN,
        }
    }
    pub fn as_f64_or(&self, default: f64) -> f64 {
        match self {
            &Value::Number(x) => x,
            _ => default,
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            &Value::Undef => false,
            &Value::Bool(b) => b,
            &Value::Number(n) => n != 0.,
            &Value::String(ref s) => s.len() != 0,
            &Value::Vector(ref v) => v.len() != 0,
            &Value::Range(_, _, _) => true,
            &Value::Objects(ref v) => v.len() != 0,
        }
    }
}

impl Expression for Value {
    fn eval(&self, _: &mut Environment, _: &mut Write) -> Value {
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
                if sv.len() != rv.len() {
                    return Value::Undef;
                }
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
                if sv.len() != rv.len() {
                    return Value::Undef;
                }
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
                if sv.len() != rv.len() {
                    return Value::Undef;
                }
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
pub struct ValueIterator {
    v: Value,
    i: isize,
}

impl ValueIterator {
    fn reset(&mut self) {
        self.i = 0;
    }
}

impl Iterator for ValueIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        if self.i < 0 {
            return None;
        }
        let i = self.i as usize;
        self.i += 1;
        match self.v {
            Value::Vector(ref v) => {
                if let Some(val) = v.get(i) {
                    Some(val.clone())
                } else {
                    None
                }
            }
            Value::Range(s, inc, e) => {
                let n = s + i as f64 * inc;
                if n <= e {
                    Some(Value::Number(n))
                } else {
                    self.i = -1;
                    None
                }
            }
            Value::Undef => None,
            Value::Bool(_) | Value::Number(_) | Value::String(_) => {
                self.i = -1;
                Some(self.v.clone())
            }
            _ => None,
        }
    }
}

impl IntoIterator for Value {
    type Item = Value;
    type IntoIter = ValueIterator;

    fn into_iter(self) -> Self::IntoIter {
        ValueIterator { v: self, i: 0 }
    }
}


#[derive(Clone, Debug)]
pub struct AssignmentExpression {
    pub id: String,
    pub ex: Box<Expression>,
}

impl Expression for AssignmentExpression {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
        let v = self.ex.eval(env, msg);
        env.vars.insert(self.id.clone(), Binding::Val(v.clone()));
        v
    }
}

#[derive(Clone, Debug)]
pub struct IdentifierExpression {
    pub id: String,
}

impl Expression for IdentifierExpression {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
        match env.vars.get(&self.id) {
            Some(&Binding::Val(ref x)) => x.clone(),
            Some(&Binding::Call(_)) => {
                writeln!(msg,
                         "Warning: {:?} is a callable, not a var, using undef.",
                         self.id)
                    .unwrap();
                Value::Undef
            }
            None => {
                writeln!(msg, "Warning: unknown {:?}, using undef.", self.id).unwrap();
                Value::Undef
            }
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
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
        if self.cond.eval(env, msg).as_bool() {
            self.ex.eval(env, msg)
        } else {
            self.alt_ex.eval(env, msg)
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
        write!(f,
               "LambdaExpression[{:?}] {{ a:{:?} b:{:?} }}",
               self.name,
               self.a,
               self.b)
    }
}

impl Expression for LambdaExpression {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
        (self.op)(self.a.eval(env, msg), self.b.eval(env, msg), msg)
    }
}

#[derive(Clone, Debug)]
pub struct VectorExpression {
    pub v: Vec<Box<Expression>>,
}

impl Expression for VectorExpression {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
        Value::Vector(self.v.iter().map(|x| x.eval(env, msg)).collect())
    }
}

#[derive(Clone, Debug)]
pub struct IndexExpression {
    pub index: Box<Expression>,
    pub ex: Box<Expression>,
}

impl Expression for IndexExpression {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
        let i: usize;
        let val = self.index.eval(env, msg);
        match val {
            Value::Number(n) => i = n as usize,
            _ => {
                writeln!(msg, "Warning: invalid index: {:?}, using undef.", val).unwrap();
                return Value::Undef;
            }
        }
        match self.ex.eval(env, msg) {
            Value::Vector(ref v) => {
                match v.get(i) {
                    Some(x) => x.clone(),
                    None => {
                        writeln!(msg,
                                 "Warning: index: out of bounds ({:?} vs {:?}), using undef.",
                                 i,
                                 v.len())
                            .unwrap();
                        Value::Undef
                    }
                }
            }
            Value::String(ref s) => {
                match s.chars().nth(i) {
                    Some(x) => Value::String(x.to_string()),
                    None => {
                        writeln!(msg,
                                 "Warning: index: out of bounds ({:?} vs {:?}), using undef.",
                                 i,
                                 s.len())
                            .unwrap();
                        Value::Undef
                    }
                }
            }
            _ => Value::Undef,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CallExpression {
    pub id: String,
    pub arguments: Vec<(String, Box<Expression>)>,
    pub sub: Option<Box<Expression>>,
}


struct ForLoopArgumentIterator {
    v: Vec<(String, ValueIterator)>,
    current: ::std::collections::HashMap<String, Binding>,
}

impl ForLoopArgumentIterator {
    fn new(arg: &Vec<(String, Box<Expression>)>,
           env: &mut Environment,
           msg: &mut Write)
           -> Option<ForLoopArgumentIterator> {
        let mut iv = Vec::new();
        for &(ref id, ref ex) in arg {
            iv.push((id.clone(), ex.eval(&mut env.clone_vars(), msg).into_iter()));
        }
        let mut hm = ::std::collections::HashMap::new();
        for &mut (ref id, ref mut vi) in &mut iv {
            if let Some(v) = vi.next() {
                hm.insert(id.clone(), Binding::Val(v));
            } else {
                return None;
            }
        }
        Some(ForLoopArgumentIterator {
            v: iv,
            current: hm,
        })
    }
}

impl Iterator for ForLoopArgumentIterator {
    type Item = ::std::collections::HashMap<String, Binding>;

    fn next(&mut self) -> Option<::std::collections::HashMap<String, Binding>> {
        if self.current.len() == 0 {
            return None;
        }
        let old = self.current.clone();
        let len = self.v.len();
        for idx in 0..self.v.len() {
            let &mut (ref id, ref mut value_it) = &mut self.v[idx];
            if let Some(v) = value_it.next() {
                self.current.insert(id.clone(), Binding::Val(v));
                break;
            } else {
                if idx == len - 1 {
                    self.current.clear();
                    break;
                }
                value_it.reset();
                self.current.insert(id.clone(), Binding::Val(value_it.next().unwrap()));
            }
        }
        return Some(old);
    }
}

impl CallExpression {
    pub fn set_sub(&mut self, sub: Option<Box<Expression>>) {
        self.sub = sub;
    }
    // Special impl for for-loops
    fn eval_for(&self, env: &mut Environment, msg: &mut Write) -> Value {
        if let Some(ref subex) = self.sub {
            let for_loop_it = ForLoopArgumentIterator::new(&self.arguments,
                                                           &mut env.clone_vars(),
                                                           msg);
            if for_loop_it.is_none() {
                return Value::Undef;
            }
            let mut for_loop_it = for_loop_it.unwrap();

            let mut result_objects = Vec::new();

            loop {
                if let Some(loop_vars) = for_loop_it.next() {
                    let mut env_copy = env.clone_vars();
                    for (key, value) in loop_vars.into_iter() {
                        env_copy.vars.insert(key, value);
                    }
                    let maybe_objs = subex.eval(&mut env_copy, msg);
                    if let Value::Objects(mut o) = maybe_objs {
                        result_objects.append(&mut o);
                    }
                } else {
                    break;
                }
            }
            Value::Objects(result_objects)
        } else {
            Value::Undef
        }
    }
}

impl Expression for CallExpression {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
        if self.id == "for" {
            return self.eval_for(env, msg);
        }
        match env.vars.get(&self.id) {
            Some(&Binding::Call(Callable { ref interface, ref ex })) => {
                let mut env_copy = env.clone_vars();
                let mut args = self.arguments.clone();
                for &(ref p_name, ref opt_def_ex) in interface {
                    let mut found = false;
                    // try to find parameter as named parameter
                    for &mut (ref a_name, ref a_ex) in &mut args {
                        if p_name == a_name {
                            let v = a_ex.eval(&mut env_copy, msg);
                            env_copy.vars.insert(a_name.clone(), Binding::Val(v));
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        // take first anonymous parameter
                        for &mut (ref mut a_name, ref a_ex) in &mut args {
                            if a_name.is_empty() {
                                let v = a_ex.eval(&mut env_copy, msg);
                                env_copy.vars.insert(p_name.clone(), Binding::Val(v));
                                *a_name = "0".to_string();  // de-anonymize
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        // take default, if any. return undef else.
                        if let &Some(ref def_ex) = opt_def_ex {
                            let v = def_ex.eval(&mut env_copy, msg);
                            env_copy.vars.insert(p_name.clone(), Binding::Val(v));
                        } else {
                            writeln!(msg,
                                     "{:?} not given for call of {:?}. returning undef.",
                                     p_name,
                                     self.id)
                                .unwrap();
                            return Value::Undef;
                        }
                    }
                }
                if let Some(ref subex) = self.sub {
                    let maybe_objs = subex.eval(&mut env.clone_vars(), msg);
                    if let Value::Objects(o) = maybe_objs {
                        env_copy.objs = o;
                    }
                }
                return ex(&mut env_copy, msg);
            }
            Some(&Binding::Val(ref x)) => {
                writeln!(msg,
                         "Warning: non callable {:?}={:?}, using undef.",
                         self.id,
                         x)
                    .unwrap()
            }
            None => writeln!(msg, "Warning: unknown {:?}, using undef.", self.id).unwrap(),
        }
        Value::Undef
    }
}

#[derive(Clone, Debug)]
pub struct RangeExpression {
    pub start: Box<Expression>,
    pub increment: Box<Expression>,
    pub end: Box<Expression>,
}

impl Expression for RangeExpression {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
        let sv = self.start.eval(env, msg);
        if let Value::Number(s) = sv {
            let iv = self.increment.eval(env, msg);
            if let Value::Number(i) = iv {
                let ev = self.end.eval(env, msg);
                if let Value::Number(e) = ev {
                    return Value::Range(s, i, e);
                } else {
                    writeln!(msg, "Warning: non-number range end {:?}, using undef.", ev).unwrap();
                }
            } else {
                writeln!(msg,
                         "Warning: non-number range increment {:?}, using undef.",
                         iv)
                    .unwrap();
            }
        } else {
            writeln!(msg,
                     "Warning: non-number range start {:?}, using undef.",
                     sv)
                .unwrap();
        }
        Value::Undef
    }
}


#[derive(Clone, Debug)]
pub struct CompoundExpression {
    pub v: Vec<Box<Expression>>,
}

impl Expression for CompoundExpression {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
        let mut v = Value::Undef;
        let mut env_copy = env.clone_vars();
        let mut objs = Vec::new();
        for ex in &self.v {
            v = ex.eval(&mut env_copy, msg);
            if let Value::Objects(o) = v {
                if let Some(union) = ::primitive::Union::from_vec(o, 0.) {
                    objs.push(union);
                }
                v = Value::Undef;
            }
        }
        if objs.len() > 0 {
            Value::Objects(objs)
        } else {
            v
        }
    }
}

#[derive(Clone, Debug)]
pub struct CallableDefinitionExpression {
    pub id: String,
    pub callable: Callable,
}

impl Expression for CallableDefinitionExpression {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
        let evaluated_interface =
            self.callable
                .interface
                .iter()
                .map(|&(ref name, ref opt_ex)| {
                    (name.clone(),
                     match opt_ex {
                        &Some(ref ex) => Some(Box::new(ex.eval(env, msg)) as Box<Expression>),
                        &None => None,
                    })
                })
                .collect();
        let evaluated_callable = Callable {
            interface: evaluated_interface,
            ex: self.callable.ex.clone(),
        };
        if let Some(old) = env.vars.insert(self.id.clone(), Binding::Call(evaluated_callable)) {
            writeln!(msg,
                     "Warning: overwriting {:?}={:?} with callable.",
                     self.id,
                     old)
                .unwrap();
        }
        Value::Undef
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

        let mut it = Value::Number(17.).into_iter();
        assert_eq!(Some(Value::Number(17.)), it.next());
        assert_eq!(None, it.next());

        assert_eq!(Value::Range(1., 2., 3.), Value::Range(1., 2., 3.));
        assert!(Value::Range(1., 2., 3.) != Value::Range(1., 2., 3.7));
        let mut it = Value::Range(1., 1.1, 4.).into_iter();
        assert_eq!(Some(Value::Number(1.)), it.next());
        assert_eq!(Some(Value::Number(2.1)), it.next());
        assert_eq!(Some(Value::Number(3.2)), it.next());
        assert_eq!(None, it.next());

        assert_eq!(Value::Vector(vec![]), Value::Vector(vec![]));
        assert_eq!(Value::Vector(vec![Value::Bool(true),
                                      Value::Number(17.),
                                      Value::Range(1., 2., 3.),
                                      Value::Vector(vec![])]),
                   Value::Vector(vec![Value::Bool(true),
                                      Value::Number(17.),
                                      Value::Range(1., 2., 3.),
                                      Value::Vector(vec![])]));
        assert!(Value::Vector(vec![Value::Bool(true)]) != Value::Vector(vec![Value::Bool(false)]));
        assert!(Value::Vector(vec![Value::Bool(true)]) !=
                Value::Vector(vec![Value::Bool(true), Value::Bool(true)]));
        let mut it = Value::Vector(vec![Value::Bool(true), Value::String("foo".to_string())])
                         .into_iter();
        assert_eq!(Some(Value::Bool(true)), it.next());
        assert_eq!(Some(Value::String("foo".to_string())), it.next());
        assert_eq!(None, it.next());

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

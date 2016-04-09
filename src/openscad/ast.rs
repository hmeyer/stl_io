use std::fmt::Debug;
use std::io::Write;
use std::rc::Rc;

pub struct Environment {
    vars: ::std::collections::HashMap<String, Binding>,
    pub objs: Vec<Box<::primitive::Object>>,
}

macro_rules! add_func {
    ( $func_name:expr, $closure:expr, $param:ident, $default:expr, $vars:expr ) => {
        {
            let mut interface = Vec::new();
            const PARAM_NAME: &'static str = stringify!($param);
            let func_closure: Rc<ExpressionFn> = Rc::new(|env, msg| {
                if let &Binding::Val(ref $param) = env.vars.get(PARAM_NAME).unwrap() {
                    let (result, mut objs) = ($closure)($param.clone(), msg);
                    env.objs.append(&mut objs);
                    result
                } else {
                    panic!("did not find expected param!");
                }
            });
            interface.push((PARAM_NAME.to_string(), Some(Box::new($default) as Box<Expression>)));
            $vars.insert($func_name.to_string(),
                                  Binding::Call(Callable {
                                      interface: interface,
                                      ex: func_closure,
                                  }));

        }
    };
}

impl Environment {
    fn clone_vars(&self) -> Environment {
        Environment {
            vars: self.vars.clone(),
            objs: vec![],
        }
    }
    pub fn new_with_primitives() -> Environment {
        let mut basic_bindings = ::std::collections::HashMap::new();
        add_func!("echo",
                  |text: Value, msg: &mut Write| {
                      writeln!(msg, "echo: {:?}", text).unwrap();
                      (Value::Undef, vec![])
                  },
                  text,
                  Value::String("".to_string()),
                  basic_bindings);
        add_func!("sphere",
                  |r: Value, _| {
                      (Value::Undef,
                       vec![Box::new(::primitive::Sphere::new(r.as_f64())) as Box<::primitive::Object>])
                  },
                  r,
                  Value::Number(1.),
                  basic_bindings);


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
}

impl Value {
    pub fn as_f64(&self) -> f64 {
        match self {
            &Value::Number(x) => x,
            _ => ::std::f64::NAN,
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

impl CallExpression {
    pub fn set_sub(&mut self, sub: Option<Box<Expression>>) {
        self.sub = sub;
    }
}

impl Expression for CallExpression {
    fn eval(&self, env: &mut Environment, msg: &mut Write) -> Value {
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
                let result = ex(&mut env_copy, msg);
                // Append all objects from the call them to the current env.
                env.objs.append(&mut env_copy.objs);
                return result;
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
        for ex in &self.v {
            v = ex.eval(&mut env_copy, msg);
        }
        if let Some(union) = ::primitive::Union::from_vec(env_copy.objs) {
            env.objs.push(union)
        }
        v
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

        assert_eq!(Value::Range(1., 2., 3.), Value::Range(1., 2., 3.));
        assert!(Value::Range(1., 2., 3.) != Value::Range(1., 2., 3.7));

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

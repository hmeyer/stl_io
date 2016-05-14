use super::ast::{Binding, Callable, Expression, ExpressionFn, Value};
use super::super::primitive::Object;
use std::io::Write;
use std::rc::Rc;

macro_rules! add_func {
    ( $func_name:expr, $closure:expr, $param:ident, $default:expr, $vars:expr ) => {
        {
            let mut interface = Vec::new();
            const PARAM_NAME: &'static str = stringify!($param);
            let func_closure: Rc<ExpressionFn> = Rc::new(|env, msg| {
                if let &Binding::Val(ref $param) = env.vars.get(PARAM_NAME).unwrap() {
                    ($closure)($param.clone(), &env.objs, msg)
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


pub fn add_functions(env: &mut ::std::collections::HashMap<String, Binding>) {
    add_func!("echo",
              |text: Value, _, msg: &mut Write| {
                  writeln!(msg, "echo: {:?}", text).unwrap();
                  Value::Undef
              },
              text,
              Value::String("".to_string()),
              env);
    add_func!("sphere",
              |r: Value, _, _| Value::Objects(vec![::primitive::Sphere::new(r.as_f64())]),
              r,
              Value::Number(1.),
              env);
    add_func!("translate",
              |t: Value, subs: &Vec<Box<Object>>, _| {
                  if subs.len() > 0 {
                      if let Value::Vector(tv) = t {
                          let mut v = Vec::new();
                          for i in 0..3 {
                              v.push(if let Some(x) = tv.get(i) {
                                  x.as_f64_or(0.)
                              } else {
                                  0.
                              });
                          }
                          let mut union_of_subs = ::primitive::Union::from_vec(subs.clone(), 0.)
                                                      .unwrap();
                          union_of_subs.translate(::types::Vector::new(v[0], v[1], v[2]));
                          return Value::Objects(vec![union_of_subs]);
                      }
                  }
                  return Value::Undef;
              },
              t,
              Value::Vector(vec![Value::Number(0.), Value::Number(0.), Value::Number(0.)]),
              env);
    add_func!("union",
              |r: Value, subs: &Vec<Box<Object>>, _| {
                  if subs.len() > 0 {
                      if let Value::Number(rf) = r {
                          return Value::Objects(vec![::primitive::Union::from_vec(subs.clone(),
                                                                                  rf)
                                                         .unwrap()]);
                      }
                  }
                  return Value::Undef;
              },
              r,
              Value::Number(0.),
              env);
    add_func!("intersection",
              |r: Value, subs: &Vec<Box<Object>>, _| {
                  if subs.len() > 0 {
                      if let Value::Number(rf) = r {
                          return Value::Objects(vec![::primitive::Intersection::from_vec(subs.clone(), rf)
                                                              .unwrap()]);
                      }
                  }
                  return Value::Undef;
              },
              r,
              Value::Number(0.),
              env);
    add_func!("difference",
              |r: Value, subs: &Vec<Box<Object>>, _| {
                  if subs.len() > 0 {
                      if let Value::Number(rf) = r {
                          return Value::Objects(vec![::primitive::Difference::from_vec(subs.clone(), rf).unwrap()]);
                      }
                  }
                  return Value::Undef;
              },
              r,
              Value::Number(0.),
              env);
}

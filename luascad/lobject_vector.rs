use hlua;
use truescad_primitive::Object;
use lobject::LObject;

pub struct LObjectVector {
    pub v: Vec<Box<Object>>,
}


// this macro implements the required trait so that we can *push* the object to lua
// (ie. move it inside lua)
implement_lua_push!(LObjectVector, |mut metatable| {
    // we create a `__index` entry in the metatable
    let mut index = metatable.empty_array("__index");

    index.set("push",
              ::hlua::function2(|v: &mut LObjectVector, o: &mut LObject| {
                  v.push(o.into_object());
              }));
});

// this macro implements the require traits so that we can *read* the object back
implement_lua_read!(LObjectVector);


impl LObjectVector {
    pub fn new(o: Box<Object>) -> LObjectVector {
        LObjectVector { v: vec![o] }
    }
    pub fn push(&mut self, o: Box<Object>) {
        self.v.push(o);
    }
    pub fn into_vec(&self) -> Vec<Box<Object>> {
        self.v.clone()
    }
}

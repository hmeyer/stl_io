use hlua;
use truescad_primitive::{Intersection, Object, Union};
use truescad_types::Float;
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
    pub fn export_factories(lua: &mut hlua::Lua, env_name: &str) {
        lua.set("__new_object_vector",
                hlua::function1(|o: &LObject| LObjectVector::new(o.into_object())));
        lua.set("__new_union",
                hlua::function2(|o: &LObjectVector, smooth: Float| {
                    LObject { o: Union::from_vec(o.into_vec(), smooth).unwrap() as Box<Object> }
                }));
        lua.set("__new_intersection",
                hlua::function2(|o: &LObjectVector, smooth: Float| {
                    LObject {
                        o: Intersection::from_vec(o.into_vec(), smooth).unwrap() as Box<Object>,
                    }
                }));
        lua.set("__new_difference",
                hlua::function2(|o: &LObjectVector, smooth: Float| {
                    LObject {
                        o: Intersection::difference_from_vec(o.into_vec(),
                                                             smooth)
                               .unwrap() as Box<Object>,
                    }
                }));
        lua.execute::<()>(&format!("
            function __array_to_ov(lobjects)
              ov = __new_object_vector(lobjects[1])
              for i = 2, #lobjects do
                ov:push(lobjects[i])
              end
              return ov
            end

            function Union(lobjects, smooth)
              return __new_union(__array_to_ov(lobjects), smooth)
            end

            function Intersection(lobjects, smooth)
              return __new_intersection(__array_to_ov(lobjects), smooth)
            end

            function Difference(lobjects, smooth)
              return __new_difference(__array_to_ov(lobjects), smooth)
            end

            {}.Union = Union;
            {}.Intersection = Intersection;
            {}.Difference = Difference;",
                                   env_name,
                                   env_name,
                                   env_name))
           .unwrap();
    }
    pub fn push(&mut self, o: Box<Object>) {
        self.v.push(o);
    }
    pub fn into_vec(&self) -> Vec<Box<Object>> {
        self.v.clone()
    }
}

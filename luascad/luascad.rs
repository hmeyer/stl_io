use hlua;
use hlua::{Lua, LuaError};
use truescad_types::Float;
use lobject::LObject;
use lobject_vector::LObjectVector;

pub fn eval(script: &str) -> Result<Option<Box<::truescad_primitive::Object>>, LuaError> {
    let mut result = None;
    {
        let mut lua = Lua::new();
        lua.openlibs();
        // we create and fill an array named `LObject` which will be used as a class-like interface
        {
            let mut object_namespace = lua.empty_array("LObject");

            object_namespace.set("__new_cube",
                                 hlua::function4(|x: Float, y: Float, z: Float, smooth: Float| {
                                     LObject::new_cube(x, y, z, smooth)
                                 }));
            object_namespace.set("__new_sphere",
                                 hlua::function1(|radius: Float| LObject::new_sphere(radius)));
            object_namespace.set("__new_icylinder",
                                 hlua::function1(|radius: Float| LObject::new_icylinder(radius)));
            object_namespace.set("__new_icone",
                                 hlua::function1(|slope: Float| LObject::new_icone(slope)));
            object_namespace.set("__new_cylinder",
                                 hlua::function4(|length: Float,
                                                  radius1: Float,
                                                  radius2: Float,
                                                  smooth: Float| {
                                     LObject::new_cylinder(length, radius1, radius2, smooth)
                                 }));
            object_namespace.set("__new_bend",
                                 hlua::function2(|o: &LObject, width: Float| {
                                     LObject::new_bend(o.into_object(), width)
                                 }));
            object_namespace.set("__new_twist",
                                 hlua::function2(|o: &LObject, height: Float| {
                                     LObject::new_twist(o.into_object(), height)
                                 }));
            object_namespace.set("__new_object_vector",
                                 hlua::function1(|o: &LObject| {
                                     LObjectVector::new(o.into_object())
                                 }));
            object_namespace.set("__new_union",
                                 hlua::function2(|o: &LObjectVector, smooth: Float| {
                                     LObject::new_union(o.into_vec(), smooth)
                                 }));
            object_namespace.set("__new_intersection",
                                 hlua::function2(|o: &LObjectVector, smooth: Float| {
                                     LObject::new_intersection(o.into_vec(), smooth)
                                 }));
            object_namespace.set("__new_difference",
                                 hlua::function2(|o: &LObjectVector, smooth: Float| {
                                     LObject::new_difference(o.into_vec(), smooth)
                                 }));
        }
        lua.set("build",
                hlua::function1(|o: &LObject| result = Some(o.into_object())));
        if let Err(e) = lua.execute::<()>(script) {
            return Err(e);
        }
    }
    return Ok(result);
}

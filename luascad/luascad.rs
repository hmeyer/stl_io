use hlua;
use hlua::{Lua, LuaError};
use lobject::LObject;
use lobject_vector::LObjectVector;

pub fn eval(script: &str) -> Result<Option<Box<::truescad_primitive::Object>>, LuaError> {
    let mut result = None;
    {
        let mut lua = Lua::new();
        lua.openlibs();
        LObject::export_factories(&mut lua);
        LObjectVector::export_factories(&mut lua);
        lua.set("build",
                hlua::function1(|o: &LObject| result = Some(o.into_object())));
        if let Err(e) = lua.execute::<()>(script) {
            return Err(e);
        }
    }
    return Ok(result);
}

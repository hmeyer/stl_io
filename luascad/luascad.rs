use hlua;
use hlua::{Lua, LuaError};
use lobject::LObject;
use lobject_vector::LObjectVector;
use sandbox;

pub const USER_FUNCTION_NAME: &'static str = "__luscad_user_function__";
pub const SANDBOX_ENV_NAME: &'static str = "__luascad_sandbox_env__";

pub fn eval(script: &str) -> Result<Option<Box<::truescad_primitive::Object>>, LuaError> {
    let mut result = None;
    {
        let mut lua = Lua::new();
        lua.openlibs();
        sandbox::set_sandbox_env(&mut lua, SANDBOX_ENV_NAME);
        {
            let mut sandbox_env = lua.get::<hlua::LuaTable<_>, _>(SANDBOX_ENV_NAME).unwrap();
            LObject::export_factories(&mut sandbox_env);
            sandbox_env.set("build",
                            hlua::function1(|o: &LObject| result = Some(o.into_object())));
        }
        // LObjectVector needs access to full lua object and the SANDBOX_ENV_NAME.
        LObjectVector::export_factories(&mut lua, SANDBOX_ENV_NAME);

        // Store the script in the Lua var USER_FUNCTION_NAME.
        try!(lua.checked_set(USER_FUNCTION_NAME, hlua::LuaCode(script)));
        // Use this script wrapper to execute USER_FUNCTION_NAME with sandbox env.
        try!(lua.execute::<()>(&format!("debug.setupvalue({}, 1, {}); return {}();",
                                        USER_FUNCTION_NAME,
                                        SANDBOX_ENV_NAME,
                                        USER_FUNCTION_NAME)));
    }
    return Ok(result);
}

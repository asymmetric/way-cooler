//! TODO Fill in

use std::fmt::{self, Display, Formatter};
use std::default::Default;
use rlua::{self, Table, Lua, UserData, ToLua, Value};
use rustwlc::{Point, input};

const INDEX_MISS_FUNCTION: &'static str = "__index_miss_function";
const NEWINDEX_MISS_FUNCTION: &'static str = "__newindex_miss_function";

#[derive(Clone, Debug)]
pub struct MouseState {
    // TODO Fill in
    dummy: i32
}

impl Default for MouseState {
    fn default() -> Self {
        MouseState {
            dummy: 0
        }
    }
}

impl Display for MouseState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Mouse: {:p}", self)
    }
}

impl UserData for MouseState {}

pub fn init(lua: &Lua) -> rlua::Result<()> {
    let mouse_table = lua.create_table();
    state_setup(lua, &mouse_table)?;
    meta_setup(lua, &mouse_table)?;
    method_setup(lua, &mouse_table)?;
    let globals = lua.globals();
    globals.set("mouse", mouse_table)
}

fn state_setup(lua: &Lua, mouse_table: &Table) -> rlua::Result<()> {
    mouse_table.set("__data", MouseState::default().to_lua(lua)?)
}

fn meta_setup(lua: &Lua, mouse_table: &Table) -> rlua::Result<()> {
    let meta_table = lua.create_table();
    meta_table.set("__tostring", lua.create_function(|_, val: Table| {
        Ok(format!("{}", val.get::<_, MouseState>("__data")?))
    }))?;
    meta_table.set("__index", lua.create_function(index))?;
    mouse_table.set_metatable(Some(meta_table));
    Ok(())
}

fn method_setup(lua: &Lua, mouse_table: &Table) -> rlua::Result<()> {
    mouse_table.set("coords", lua.create_function(coords))?;
    mouse_table.set("set_index_miss_handler", lua.create_function(set_index_miss))?;
    mouse_table.set("set_newindex_miss_handler", lua.create_function(set_newindex_miss))?;
    Ok(())
}


fn coords<'lua>(lua: &'lua Lua, (coords, _ignore_enter): (rlua::Value<'lua>, rlua::Value<'lua>))
                -> rlua::Result<Table<'lua>> {
    match coords {
        rlua::Value::Table(coords) => {
            let (x, y) = (coords.get("x")?, coords.get("y")?);
            // TODO The ignore_enter is supposed to not send a send event to the client
            // That's not possible, at least until wlroots is complete.
            input::pointer::set_position(Point { x, y });
            Ok(coords)
        },
        _ => {
            // get the coords
            let coords = lua.create_table();
            let Point { x, y } = input::pointer::get_position();
            coords.set("x", x)?;
            coords.set("y", y)?;
            // TODO It expects a table of what buttons were pressed.
            coords.set("buttons", lua.create_table())?;
            Ok(coords)
        }
    }
}

fn set_index_miss(lua: &Lua, func: rlua::Function) -> rlua::Result<()> {
    lua.globals().get::<_, Table>("button")?.set(INDEX_MISS_FUNCTION, func)
}

fn set_newindex_miss(lua: &Lua, func: rlua::Function) -> rlua::Result<()> {
    lua.globals().get::<_, Table>("button")?.set(NEWINDEX_MISS_FUNCTION, func)
}

fn index<'lua>(lua: &'lua Lua,
               (obj_table, index): (Table<'lua>, Value<'lua>))
               -> rlua::Result<Value<'lua>> {
    use rustwlc::WlcOutput;
    use super::screen::SCREENS_HANDLE;
    match index {
        Value::String(ref string) => {
            let string = string.to_str()?;
            if string != "screen" {
                // TODO call miss index handler if it exists
            }
            // TODO Might need a more robust way to get the current output...
            // E.g they look at where the cursor is, I don't think we need to do that.
            let index = WlcOutput::list().iter()
                .position(|&output| output == WlcOutput::focused())
                // NOTE Best to just lie because no one handles nil screens properly
                .unwrap_or(0);
            let screens = lua.globals().get::<_, Vec<Table>>(SCREENS_HANDLE)?;
            if index < screens.len() {
                return screens[index].clone().to_lua(lua)
            }
            // TODO Return screen even in bad case, see how awesome does it for maximal compatibility
        },
        _ => {}
    }
    let meta = obj_table.get_metatable().unwrap();
    meta.get(index)
}

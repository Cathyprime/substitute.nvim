mod case;
mod rules;
mod utils;

use crate::case::CaseBuf;
use nvim_oxi::{Dictionary, Function};

type CResult = Result<CaseBuf, ()>;

#[nvim_oxi::plugin]
fn substitute_utils() -> nvim_oxi::Result<Dictionary> {
    let get_regex = Function::from_fn(utils::get_regex_lua);
    let replace = Function::from_fn(utils::replace_lua);

    let mut api = Dictionary::new();
    api.insert("find_regex", get_regex);
    api.insert("replace", replace);
    Ok(api)
}

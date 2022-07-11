use std::{env::current_dir, fs::read_dir};

use proc_macro::TokenStream;
extern crate proc_macro;

#[proc_macro]
pub fn auto_import(item: TokenStream) -> TokenStream {
    let mut dir = current_dir().unwrap();
    dir.push("src");
    dir.push(item.to_string().replace(' ', ""));
    read_dir(dir)
        .unwrap()
        .filter_map(|i| i.ok())
        .filter_map(|e| {
            let file_name = e.file_name().into_string().unwrap();
            if file_name.ends_with(".rs") && !["mod.rs", "main.rs", "lib.rs"].contains(&&*file_name)
            {
                let name = file_name.strip_suffix(".rs").unwrap();
                Some(format!("pub mod {name};"))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn auto_define(item: TokenStream) -> TokenStream {
    let mut dir = current_dir().unwrap();
    dir.push("src");
    dir.push(item.to_string().replace(' ', ""));
    read_dir(dir)
        .unwrap()
        .filter_map(|i| i.ok())
        .filter_map(|e| {
            let file_name = e.file_name().into_string().unwrap();
            if file_name.ends_with(".rs") && !["mod.rs", "main.rs", "lib.rs"].contains(&&*file_name)
            {
                let name = file_name.strip_suffix(".rs").unwrap();
                Some(format!("cmds.push({}::{name}::get_command());", item.to_string().replace(' ', "")))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        .parse()
        .unwrap()
}

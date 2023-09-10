use std::env;
use std::fs::create_dir;
use std::path::Path;
use fs_extra;
use fs_extra::copy_items;

fn main() {
    let build_type = env::var_os("PROFILE").expect("e: get profile");
    let project_dir = env::var_os("CARGO_MANIFEST_DIR").expect("e: build PROJECT dir");

    let out_dir = Path::new(&project_dir).join("target").join(&build_type);
    let res_dir = Path::new(&project_dir).join("res");
    let res_out_dir = Path::new(&out_dir).join("res");

    println!("o:{:?}\nr:{:?}\nro:{:?}", out_dir, res_dir, res_out_dir);
    if !res_out_dir.is_dir() {
        create_dir(&res_out_dir).expect("e: create RES_OUT_DIR");
    }

    let options = fs_extra::dir::CopyOptions::new().overwrite(true);
    copy_items(&[res_dir], res_out_dir, &options).expect("e: copy RES dir");

    println!("cargo:rerun-if-changed=build.rs")
}
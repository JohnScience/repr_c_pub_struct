use repr_c_pub_struct::parse_for_repr_c_pub_structs;

fn main() {
    let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let repr_c_structs = parse_for_repr_c_pub_structs(crate_root.as_str());
    println!("{:#?}", repr_c_structs);
}

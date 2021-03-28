pub mod oca;

fn main() {
    let res = oca::parse::untyped();
    let oca_branch = res.unwrap();
    println!("{}", oca_branch["schema_base"]);
    println!("{}", oca_branch["overlays"]);
}

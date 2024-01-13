use pe2table::parse_impl;

fn main() {
    env_logger::init();

    //
    //

    let file = include_bytes!("../../target/release/example.exe");
    let pdb = include_bytes!("../../target/release/example.pdb");
    let result = parse_impl(file,  pdb,true);

    log::info!("result: {:?}", result);
}
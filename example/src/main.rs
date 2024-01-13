use defmt_kernel as _;

#[link_section = ".test"]
#[export_name = "asdf"]
static MAGIC_STR: &'static str = "this is in test";

#[link_section = ".test"]
#[export_name = "asdf2"]
static  MAGIC_STR2: &'static str = "this is also in test";

fn main() {
    // defmt::println!("1Hello, world!");
    // defmt::trace!("2Hello, world!");
    // defmt::debug!("3Hello, world!");
    // defmt::info!("4Hello, world!");
    // defmt::warn!("5Hello, world!");
    // defmt::error!("6Hello, world!");
    defmt::error!("7Hello, world!");
    // defmt::error!("8Hello, world!");
    // defmt::error!("9Hello, world!");
    // defmt::error!("Now {} with {} some {} data {}", 1, u64::MAX, true, 42.424242);
}

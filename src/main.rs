use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
enum Flags {
    asc = 0,
    desc = 1,
}

fn c_get_flags() -> u32 {
    1
}

fn get_flags() -> Flags {
    let flags = c_get_flags();
    Flags::try_from(flags).unwrap()
}

fn main() {
    println!("Flags: {:?}", get_flags());
}

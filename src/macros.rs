use global::*;
use errors::*;
use types::*;

#[macro_export]
macro_rules! debug_println {
    () => (if *DEBUG {print!("\n")});
    ($fmt:expr) => (if *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}
#[macro_export]
macro_rules! info_println {
    () => (if *INFO {print!("\n")});
    ($fmt:expr) => (if *INFO {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *INFO {print!(concat!($fmt, "\n"), $($arg)*)});
}
#[macro_export]
macro_rules! bits_println {
    () => (if *BITSTREAM | *DEBUG {print!("\n")});
    ($fmt:expr) => (if *BITSTREAM | *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *BITSTREAM | *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! vv_bits_println {
    () => (if *BITSTREAM & *DEBUG {print!("\n")});
    ($fmt:expr) => (if *BITSTREAM & *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *BITSTREAM & *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! blif_println {
    () => (if *BLIF_DEBUG | *DEBUG {print!("\n")});
    ($fmt:expr) => (if *BLIF_DEBUG | *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *BLIF_DEBUG | *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}
#[macro_export]
macro_rules! vv_blif_println {
    () => (if *BLIF_DEBUG & *DEBUG {print!("\n")});
    ($fmt:expr) => (if *BLIF_DEBUG & *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *BLIF_DEBUG & *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! route_println {
    () => (if *ROUTE_DEBUG | *INFO {print!("\n")});
    ($fmt:expr) => (if *ROUTE_DEBUG | *INFO {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *ROUTE_DEBUG | *INFO {print!(concat!($fmt, "\n"), $($arg)*)});
}
#[macro_export]
macro_rules! vv_route_println {
    () => (if *ROUTE_DEBUG & *INFO | *DEBUG {print!("\n")});
    ($fmt:expr) => (if *ROUTE_DEBUG & *INFO | *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *ROUTE_DEBUG & *INFO | *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}
/// Return with a boxed error.
macro_rules! bail {
    ($err:expr) => {
        return Err(Box::new($err));
    };
}

/// Assert that a value is what it's supposed to be.
/// Returns with a boxed error in case the assertion fails.
///
/// I really don't like how this is written, but for now it's good enough.
macro_rules! require {
    ($actual:expr => $expected:expr; else $err:expr) => {{
        if $expected != $actual {
            bail!($err);
        } else {
            $actual
        }
    }};
}

/// Print an address range and hex value.
/// Supports optional comments / remarks.
///
/// # Example
/// ```rust
/// // Let's pretend we read these from a file
/// let magic = 0x1337;
/// let version = 0x0001;
///
/// // Simple
/// printhex!(0x0000: u16 = magic);
/// printhex!(0x0004: u16 = version);
///
/// // With annotations
/// printhex!(0x0000: u16 = magic; "Magic");
/// printhex!(0x0004: u16 = version; "Version");
/// ```
macro_rules! printhex {
    ($addr:literal: $width:ty = $val:expr; $comment:literal) => {{
        use colored::*;
        let padcount = <$width>::BITS as usize / 4;
        let padding = format!("{:_>padding$}", "", padding = 8 - padcount);
        let hexvalue = format!("{value:0width$x}", value = $val, width = padcount);
        let value = format!("0x{}{}", hexvalue.white(), padding.black());
        let address = {
            if <$width>::BITS == 8 {
                format!("{: <9}{:#06x}", "", $addr)
            } else {
                let offset = (<$width>::BITS - 8) / 8;
                format!("{:#06x} - {:#06x}", $addr, $addr + offset)
            }
        };
        println!(
            "{address} {value} {comment}",
            address = address.blue(),
            value = value.white(),
            comment = $comment.green()
        );
    }};
    ($addr:literal: $width:ty = $val:expr) => {{
        printhex!($addr: $width = $val; "");
    }};
}

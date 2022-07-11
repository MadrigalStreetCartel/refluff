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
    (__print_table) => {{
        use colored::*;
        println!(
            "{}",
            format!(
                "{start} {hex_value} {dec_value} {end} {comment}",
                start = format!("{: <6}", "start"),
                end = format!("{: <6}", "end"),
                hex_value = format!("{: <10}", "hex"),
                dec_value = format!("{: <10}", "dec"),
                comment = "comment"
            ).bright_black()
        );
    }};
    ($addr:literal: $width:ty = $val:expr; $comment:literal) => {{
        use colored::*;
        let val = { $val };
        let addr = { $addr };
        let hex_pad_count = <$width>::BITS as usize / 4;
        let hex_value = format!("{value:0width$x}", value = val, width = hex_pad_count);
        let hex_padding = format!("{:_>padding$}", "", padding = 8 - hex_pad_count);
        let dec_value = val.to_string();
        let dec_padding = format!("{:_>padding$}", "", padding = 10 - dec_value.len());
        let padded_hex_value = format!("0x{}{}", hex_value.white(), hex_padding.black());
        let padded_dec_value = format!("{}{}", dec_value.white(), dec_padding.black());
        let offset_start = format!("{:#06x}", addr);
        let offset_end = format!("{:#06x}", addr + (<$width>::BITS - 8) / 8);
        println!(
            "{offset_start} {hex_value} {dec_value} {offset_end} {comment}",
            offset_start = offset_start.blue(),
            offset_end = offset_end.blue(),
            hex_value = padded_hex_value,
            dec_value = padded_dec_value,
            comment = $comment.green()
        );
        val
    }};
    ($addr:literal: $width:ty = $val:expr) => {{
        printhex!($addr: $width = $val; "")
    }};
}

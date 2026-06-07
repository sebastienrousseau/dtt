#![allow(missing_docs)]

// Timezone — disambiguated abbreviations and custom offsets.

use dtt::prelude::*;

fn main() -> Result<(), AppError> {
    // Disambiguated codes — `IST` is intentionally not accepted because
    // it could mean Indian, Irish, or Israel time.
    let mumbai = DateTime::new_with_tz("IST_INDIA")?;
    let dublin = DateTime::new_with_tz("IST_IRELAND")?;
    let tel_aviv = DateTime::new_with_tz("IST_ISRAEL")?;

    println!("Mumbai   : {mumbai}");
    println!("Dublin   : {dublin}");
    println!("Tel Aviv : {tel_aviv}");

    // Convert between zones.
    let utc = DateTime::parse("2024-06-15T12:00:00Z")?;
    let nyc = utc.convert_to_tz("EST_USA")?;
    let tokyo = utc.convert_to_tz("JST")?;
    println!("\nSame instant in three zones:");
    println!("  UTC   : {utc}");
    println!("  NYC   : {nyc}");
    println!("  Tokyo : {tokyo}");

    // Equality is offset-independent: same instant compares equal.
    assert_eq!(utc, nyc);
    assert_eq!(utc, tokyo);

    // Custom offset for any zone not in the table.
    let kathmandu = DateTime::new_with_custom_offset(5, 45)?;
    println!("\nKathmandu (custom +05:45): {kathmandu}");

    // Mixed-sign offsets are rejected as a footgun.
    assert!(DateTime::new_with_custom_offset(5, -30).is_err());

    Ok(())
}

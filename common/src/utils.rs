use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

pub fn parse_buddhist_date(
    date_str: &str,
) -> Result<OffsetDateTime, Box<dyn std::error::Error>> {
    // Parse the input date string: "01/01/2568"
    let parts: Vec<u8> = date_str
        .split('/')
        .map(|s| s.parse::<u8>())
        .collect::<Result<_, _>>()?;

    if parts.len() != 3 {
        return Err("Invalid date format".into());
    }

    let day = parts[0];
    let month = parts[1];
    let year_be = parts[2];

    let year_ad = (year_be as i32) - 543;
    let month_enum = Month::try_from(month)?;

    let date = Date::from_calendar_date(year_ad, month_enum, day)?;
    let time = Time::MIDNIGHT; // or any specific time if you have one
    let primitive_dt = PrimitiveDateTime::new(date, time);

    // Apply UTC or your desired offset
    let offset_dt = primitive_dt.assume_offset(UtcOffset::UTC);

    Ok(offset_dt)
}

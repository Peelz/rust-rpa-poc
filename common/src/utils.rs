use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

pub fn parse_buddhist_date(
    date_str: &str,
) -> Result<OffsetDateTime, Box<dyn std::error::Error>> {
    // Parse the input date string: "01/01/2568"
    let parts: Vec<&str> = date_str.split('/').collect();

    if parts.len() != 3 {
        return Err("Invalid date format".into());
    }

    let day: u8 = parts[0].parse()?;
    let month: u8 = parts[1].parse()?;
    let year_be: i32 = parts[2].parse()?;

    let year_ad = year_be - 543;
    let month_enum = Month::try_from(month)?;

    let date = Date::from_calendar_date(year_ad, month_enum, day)?;
    let time = Time::MIDNIGHT; // or any specific time if you have one
    let primitive_dt = PrimitiveDateTime::new(date, time);

    // Apply UTC or your desired offset
    let offset_dt = primitive_dt.assume_offset(UtcOffset::UTC);

    Ok(offset_dt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_parse_buddhist_date_success_1() {
        let date_str = "01/01/2568";
        let expected_dt = datetime!(2025-01-01 00:00:00 UTC);
        let result = parse_buddhist_date(date_str).unwrap();
        assert_eq!(result, expected_dt);
    }


        #[test]
    fn test_parse_buddhist_date_success_2() {
        let date_str = "31/12/2568";
        let expected_dt = datetime!(2025-12-31 00:00:00 UTC);
        let result = parse_buddhist_date(date_str).unwrap();
        assert_eq!(result, expected_dt);
    }


    #[test]
    fn test_parse_buddhist_date_invalid_format() {
        let date_str = "01-01-2568";
        let result = parse_buddhist_date(date_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_buddhist_date_invalid_month() {
        let date_str = "01/13/2568";
        let result = parse_buddhist_date(date_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_buddhist_date_invalid_day() {
        let date_str = "32/01/2568";
        let result = parse_buddhist_date(date_str);
        assert!(result.is_err());
    }
}

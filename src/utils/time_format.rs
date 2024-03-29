use chrono::{NaiveDate, NaiveDateTime};

const TIME_FORMAT: &str = "%Y-%m-%d %T";
const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn naive_date_format(date: NaiveDate) -> String {
    date.format(DATE_FORMAT).to_string()
}

pub fn naive_date_time_format(date_time: NaiveDateTime) -> String {
    date_time.format(TIME_FORMAT).to_string()
}

pub fn bson_date_time_format(date: bson::DateTime) -> String {
    date.to_chrono().format(TIME_FORMAT).to_string()
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use super::{DATE_FORMAT, TIME_FORMAT};

    #[test]
    // #[should_panic]
    fn test_time_format() {
        let time = bson::DateTime::now();
        let st = time.to_chrono().format(TIME_FORMAT).to_string();
        println!("{st}")
    }

    #[test]
    // #[should_panic]
    fn test_naive_format() {
        let time = NaiveDate::from_ymd_opt(2022, 1, 21).unwrap();
        let format = time.format(DATE_FORMAT).to_string();
        println!("{format}")
    }
}

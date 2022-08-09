use chrono::NaiveDateTime;
use http::HeaderValue;

use crate::error::VerifyResult;

const MODIFY_TIME_FORMAT: &str = "%a, %d %b %Y %H:%M:%S GMT";

pub(crate) fn from_request_head(
    header: &HeaderValue,
) -> VerifyResult<NaiveDateTime> {
    log::info!("取得请求头\"If-Modified-Since\"信息 {header:?}");
    let data_time = header.to_str()?;
    Ok(NaiveDateTime::parse_from_str(
        data_time,
        MODIFY_TIME_FORMAT,
    )?)
}

pub(crate) fn to_request_header(
    time: &NaiveDateTime,
) -> VerifyResult<HeaderValue> {
    let data_time = time.format(MODIFY_TIME_FORMAT).to_string();
    Ok(HeaderValue::from_str(&data_time)?)
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use http::HeaderValue;

    use super::{from_request_head, to_request_header};

    #[test]
    fn test_from() {
        let value =
            HeaderValue::from_str("Wed, 21 Oct 2015 07:28:00 GMT").unwrap();
        let time = from_request_head(&value).unwrap();

        assert_eq!(
            time,
            NaiveDateTime::new(
                NaiveDate::from_ymd(2015, 10, 21),
                NaiveTime::from_hms(7, 28, 0)
            )
        );

        println!("{time}")
    }
    #[test]
    fn test_to() {
        let time = NaiveDateTime::new(
            NaiveDate::from_ymd(2015, 10, 21),
            NaiveTime::from_hms(7, 28, 0),
        );

        let v = to_request_header(&time).unwrap();

        println!("{v:?}")
    }
}

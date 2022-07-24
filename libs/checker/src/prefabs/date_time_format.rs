use chrono::{NaiveDate, NaiveDateTime};
use futures::future::{ready, Ready};

use crate::Checker;

const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub struct DateTimeFormatChecker;

impl Checker for DateTimeFormatChecker {
    type Args = ();
    type Checked = NaiveDateTime;
    type Err = chrono::ParseError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready(NaiveDateTime::parse_from_str(&uncheck, DATE_TIME_FORMAT))
    }
}

const DATE_FORMAT: &str = "%Y-%m-%d";

pub struct DateFormatChecker;

impl Checker for DateFormatChecker {
    type Args = ();
    type Checked = NaiveDate;
    type Err = chrono::ParseError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready(NaiveDate::parse_from_str(&uncheck, DATE_FORMAT))
    }
}

const TIME_FORMAT: &str = "%H:%M:%S";

pub struct TimeFormatChecker;

impl Checker for TimeFormatChecker {
    type Args = ();
    type Checked = NaiveDate;
    type Err = chrono::ParseError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready(NaiveDate::parse_from_str(&uncheck, TIME_FORMAT))
    }
}

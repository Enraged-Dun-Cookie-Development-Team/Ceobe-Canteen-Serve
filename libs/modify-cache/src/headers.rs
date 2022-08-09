use std::collections::HashSet;

use chrono::NaiveDateTime;
use nom::{
    bytes::complete::{tag, take_till1},
    combinator::opt,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub enum ControlHeaders {
    IfNoneMatch(HashSet<String>),
    IfModifySince(NaiveDateTime),
    None,
}
/// function action
///
/// https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md
pub fn etag(header_value: &str) -> IResult<&str, HashSet<String>> {
    // W/"abcabcabc"
    let handle = tuple((
        opt(tag("W/")),
        tag("\""),
        take_till1(|c: char| c == '"'),
        tag("\""),
    ));

    let (remain, data) = separated_list1(tag(", "), handle)(header_value)?;

    Ok((
        remain,
        data.into_iter().map(|(_, _, v, _)| v.to_owned()).collect(),
    ))
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::etag;

    #[test]
    fn test_etag() {
        let header = r#""33a64df551425fcc55e4d42a148795d9f25f89d4""#;
        let header2 = r#"W/"67ab43", "54ed21", "7892dd""#;

        let re = etag(header).expect("");
        assert_eq!(
            re,
            (
                "",
                HashSet::from_iter([
                    "33a64df551425fcc55e4d42a148795d9f25f89d4".to_string()
                ])
            )
        );
        println!("{:?} , {:?}", re.0, re.1);

        let re = etag(header2).expect("");
        assert_eq!(
            re,
            (
                "",
                HashSet::from_iter([
                    "67ab43".to_string(),
                    "54ed21".to_string(),
                    "7892dd".into()
                ])
            )
        );
        println!("{:?} , {:?}", re.0, re.1);
    }
}

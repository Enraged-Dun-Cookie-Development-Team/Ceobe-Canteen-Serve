/// # Example
/// * handle Option
/// ```rust
/// let a = Some(11);
/// let b = Option::<i32>::None;
///
/// let f = move || {
///     let t = to_rresult!(op a, [Status::Ok],"None Data");
///
///     RResult::Success(t)
/// };
///
/// assert_eq!(f(), RResult::ok(11));
///
/// let f = move || {
///     let t = to_rresult!(op b,"None Data");
///     RResult::ok(t)
/// };
///
/// assert_eq!(f(), RResult::err("None Data"))
/// ```
/// * handle Result
///
/// ```rust
/// let a = Result::<_, ()>::Ok(11);
/// let b = Result::<i32, _>::Err(());
///
/// let f = move || {
///     let t = to_rresult!(rs a);
///
///     RResult::Success(t)
/// };
///
/// assert_eq!(f(), RResult::ok(11));
///
/// let f = move || {
///     let t = to_rresult!(rs b);
///     RResult::ok(t)
/// };
///
/// assert_eq!(f(), RResult::err(()))
/// ```
#[macro_export]
macro_rules! to_rresult {
    (op $x:expr, $s:expr) => {
        match $x {
            Some(d) => d,
            None => return $crate::RResult::err($s),
        }
    };

    (rs $x:expr) => {
        match $x {
            Ok(d) => d,
            Err(err) => return $crate::RResult::err(err),
        }
    };

    (rs $x:expr, [$sta:expr]) => {
        match $x {
            Ok(d) => d,
            Err(err) => return $crate::RResult::status_err($sta, err),
        }
    };

    (rs $x:expr, $info:expr) => {
        match $x {
            Ok(d) => d,
            Err(err) => return $crate::RResult::err(format!("{} => {}", $info, err)),
        }
    };

    (rs $x:expr, [$sta:expr], $info:expr) => {
        match $x {
            Ok(d) => d,
            Err(err) => return $crate::RResult::status_err($sta, format!("{} => {}", $info, err)),
        }
    };
}

#[cfg(test)]
mod test_macro {

    use crate::RResult;

    #[test]
    fn test_option() {
        let a = Some(11);
        let b = Option::<i32>::None;

        let f = move || {
            let t = to_rresult!(op a, "None Data");

            RResult::Success(t)
        };

        assert_eq!(f(), RResult::ok(11));

        let f = move || {
            let t = to_rresult!(op b,"None Data");
            RResult::ok(t)
        };

        assert_eq!(f(), RResult::err("None Data"))
    }

    #[test]
    fn test_result() {
        let a = Result::<_, ()>::Ok(11);
        let b = Result::<i32, _>::Err(());

        let f = move || {
            let t = to_rresult!(rs a);

            RResult::Success(t)
        };

        assert_eq!(f(), RResult::ok(11));

        let f = move || {
            let t = to_rresult!(rs b);
            RResult::ok(t)
        };

        assert_eq!(f(), RResult::err(()))
    }
}

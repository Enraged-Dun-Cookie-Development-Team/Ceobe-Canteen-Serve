//! 辅助服务器构建相关工具包

use rocket::Route;

pub trait Controller {
    fn base()->&'static str;
    fn routes()->Vec<Route>;
}

#[macro_export]
/// 快捷构造Controller
macro_rules! generate_controller {
    ($name:ident,$base:literal,$($routes:path),*) => {
        pub struct $name;
        impl crate::controllers::Controller for $name  {
            fn routes()->Vec<rocket::Route>{
                rocket::routes![
                    $(
                        $routes
                    ),*
                ]
            }

            fn base<'s>()->&'s str{
                $base
            }
        }
    };
}

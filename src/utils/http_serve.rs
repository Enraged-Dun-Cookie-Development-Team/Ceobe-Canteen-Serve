//! 辅助服务器构建相关工具包

use actix_web::dev::HttpServiceFactory;

pub trait Controller {
    type Service: HttpServiceFactory + 'static;
    fn base<'s>() -> &'s str;
    fn serve() -> Self::Service;
}

#[macro_export]
/// 快捷构造 [Controller](Controller)
/// ```rust
///     generate_controller!(
///         CeobeController, // 新建的 Controller 类型名称
///         "/ceobe",        // Controller 的根路由
///         update,          //--|
///         save_setting,    //  |------ Controller 的路由
///         get_setting      //--|
///     );
/// ```
macro_rules! generate_controller {
    ($name:ident,$base:literal$(,$routes:path)*) => {
        pub struct $name;
        impl $crate::utils::http_serve::Controller for $name  {
            type Service=actix_web::Scope;
            fn serve()->Self::Service{
                actix_web::web::scope($base)
                $(
                    .service($routes)
                )*
            }

            fn base<'s>()->&'s str{
                $base
            }
        }

        impl actix_web::dev::HttpServiceFactory for $name {
            fn register(self, config: &mut actix_web::dev::AppService) {
                <Self as $crate::utils::http_serve::Controller>::serve().register(config)
            }
        }
    };
}

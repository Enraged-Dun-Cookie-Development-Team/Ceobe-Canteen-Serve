//! 辅助服务器构建相关工具包

use actix_web::dev::HttpServiceFactory;

use super::mongodb_utils::{self, db_manager::DbBuild};

pub trait Controller {
    type Service: HttpServiceFactory + 'static;
    fn base<'s>() -> &'s str;
    fn serve() -> Self::Service;
}

pub trait MongoRegister: Controller {
    type Register: mongodb_utils::module_register::MongoRegister;
    fn register() -> Self::Register;
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
    ($name:ident,$base:literal$(,$routes:path)*,[$($data:expr),*]) => {
        pub struct $name;
        impl $crate::utils::http_serve::Controller for $name  {
            type Service=actix_web::Scope;
            fn serve()->Self::Service{
                actix_web::web::scope($base)
                $(
                    .service($routes)
                )*
                $(
                    .app_data(Data::new($data))
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
    ($name:ident,$base:literal$(,$routes:path)*) =>{
        $crate::generate_controller!($name,$base $(,$routes)*,[]);
    }
}

/// 为控制器提供额外模型信息
/// ## example
/// ```rust
/// crate::generate_controller!(Mocker, "/mock");
///
///crate::db_selector!(pub Mansion="mansion_data");
///
///fn do_mock(_: &mut DbBuild) {}
/// // 为现有Controller 扩展 模型注册器
///crate::extra_module!(Mocker=>crate::generate_collection_register!{Mansion=>do_mock});
/// ```
#[macro_export]
macro_rules! extra_module {
    ($c:ty=>$register:expr) => {
        impl crate::utils::http_serve::MongoRegister for $c {
            type Register = impl $crate::utils::mongodb_utils::module_register::MongoRegister;
            fn register() -> Self::Register {
                $register
            }
        }
    };
}

crate::generate_controller!(Mocker, "/mock");

crate::db_selector!(pub Mansion="mansion_data");

fn do_mock(_: &mut DbBuild) {}

crate::extra_module!(Mocker=>crate::generate_collection_register!{Mansion=>do_mock});

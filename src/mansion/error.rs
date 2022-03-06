use http::StatusCode;
use rresult::coded_error;

use crate::error_generate;

error_generate!(
    pub MansionError
    Orm=sea_orm::DbErr
    Id=UnknownId
    NotFound=NotFound
    Actix=actix_web::error::Error
);

error_generate!(
    pub UnknownId="请求的id不是 XX.X 格式"
);

coded_error!(UnknownId[1145: StatusCode::UNPROCESSABLE_ENTITY]);

error_generate!(
    pub NotFound="指定饼学大厦ID未找到"
);
coded_error!(NotFound[1146: StatusCode::NOT_FOUND]);
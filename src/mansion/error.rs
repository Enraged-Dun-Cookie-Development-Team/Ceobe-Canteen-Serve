use http::StatusCode;
use rresult::coded_error;

use crate::error_generate;

error_generate!(
    pub MansionError
    Orm=sea_orm::DbErr
    Id=UnknownId
    NotFound=NotFound
    Fraction=BadFraction
    Actix=actix_web::error::Error
);

error_generate!(
    pub UnknownId="请求的id不是 XX.X 格式"
);

coded_error!(UnknownId[1145: StatusCode::NOT_ACCEPTABLE]);

error_generate!(
    pub NotFound="指定饼学大厦ID未找到"
);
coded_error!(NotFound[1146: StatusCode::NOT_FOUND]);

error_generate!(
    pub BadFraction="错误的打分范围(0~5)"
);

coded_error!(BadFraction[1147: StatusCode::NOT_ACCEPTABLE]);
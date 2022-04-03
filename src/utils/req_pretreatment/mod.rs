pub mod prefabs {

    pub use request_pretreat::prefabs::{
        DefaultValue, JsonError, JsonPayload as Json, MapError as MapErr,
        PathError, PathValue as Path, QueryArgs as Query,
        ToRespResult as ToRResult,
    };
}

pub use request_pretreat::{
    Pretreatment as ReqPretreatment, Treater as Pretreatment,
};

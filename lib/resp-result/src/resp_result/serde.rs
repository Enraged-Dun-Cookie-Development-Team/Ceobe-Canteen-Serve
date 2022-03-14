use serde::{ser::SerializeStruct, Serialize};

use crate::{get_config, resp_error::RespError};

use super::RespResult;



impl<T, E> Serialize for RespResult<T, E>
where
    T: Serialize,
    E: RespError,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let cfg = &get_config().serde;
        let (mut ok_size, mut err_size) = (1, 1);
        // 简易状态标记
        if cfg.signed_base_status.is_some() {
            ok_size += 1;
            err_size += 1;
        }
        //额外的异常码
        #[cfg(feature = "extra-code")]
        if cfg.extra_code.is_some() {
            if cfg.full_field {
                ok_size += 1;
            }
            err_size += 1;
        }

        if cfg.full_field {
            ok_size += 1;
            err_size += 1;
        }

        let resp = match self {
            RespResult::Success(data) => {
                let mut body = serializer.serialize_struct("RespResult", ok_size)?;
                if let Some(n)=cfg.signed_base_status {
                    body.serialize_field(n, &true)?;
                }
                if cfg.full_field {
                    #[cfg(feature = "extra-code")]
                    if let Some(ecl)= cfg.extra_code {
                        body.serialize_field(ecl, &Option::<()>::None)?;
                    }
                    body.serialize_field(cfg.err_msg_name, &Option::<()>::None)?;
                }

                body.serialize_field(cfg.body_name, data)?;

                body.end()?
            }
            RespResult::Err(err) => {
                let mut body = serializer.serialize_struct("RespResult", err_size)?;
                if let Some(n)=cfg.signed_base_status {
                    body.serialize_field(n, &true)?;
                }
                #[cfg(feature = "extra-code")]
                if let Some(ecl)= cfg.extra_code{
                    body.serialize_field(ecl, &err.extra_code())?;
                }
                body.serialize_field(cfg.err_msg_name, &err.description())?;

                if cfg.full_field {
                    body.serialize_field(cfg.body_name, &Option::<()>::None)?;
                }
                body.end()?
            }
        };
        Ok(resp)
    }
}

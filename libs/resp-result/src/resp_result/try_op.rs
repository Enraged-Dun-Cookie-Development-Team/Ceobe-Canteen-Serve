use std::ops::{ControlFlow, FromResidual, Try};

use super::RespResult;

impl<T, E> Try for RespResult<T, E> {
    type Output = T;

    type Residual = E;

    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Self::Success(output)
    }
    #[inline]
    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            RespResult::Success(data) => ControlFlow::Continue(data),
            RespResult::Err(e) => ControlFlow::Break(e),
        }
    }
}

impl<T, E> FromResidual for RespResult<T, E> {
    #[inline]
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        Self::Err(residual)
    }
}

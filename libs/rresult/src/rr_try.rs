use std::ops::{ControlFlow, FromResidual, Try};

use crate::RResult;

impl<T, E> Try for RResult<T, E> {
    type Output = T;

    type Residual = E;

    fn from_output(output: Self::Output) -> Self {
        Self::ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            RResult::Success(data) => ControlFlow::Continue(data),
            RResult::Error(e) => ControlFlow::Break(e),
        }
    }
}

impl<T, E> FromResidual for RResult<T, E> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        Self::err(residual)
    }
}

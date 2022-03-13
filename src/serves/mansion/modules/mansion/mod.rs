use chrono::NaiveDate;

use crate::serves::mansion::db_ops;

pub mod checkers;

#[derive(Debug, serde::Serialize)]
pub struct Mansion {
    id: String,
    cvlink: String,
    description: String,
    fraction: i16,
    daily: Vec<DailyMansion>,
}
#[derive(Debug, serde::Serialize)]
pub struct DailyMansion {
    at: NaiveDate,
    content: String,
    inners: Vec<Inner>,
}
#[derive(Debug, serde::Serialize)]
pub struct Inner {
    predict: Predict,
    info: String,
}

#[derive(Debug, serde::Serialize)]
pub enum Predict {
    False,
    Unknown,
    True,
}

impl
    From<(
        db_ops::mansion::Model,
        Vec<(
            db_ops::daily_mansion::Model,
            Vec<db_ops::mansion_info::Model>,
        )>,
    )> for Mansion
{
    fn from(
        (m, e): (
            db_ops::mansion::Model,
            Vec<(
                db_ops::daily_mansion::Model,
                Vec<db_ops::mansion_info::Model>,
            )>,
        ),
    ) -> Self {
        let db_ops::mansion::Model {
            mid,
            sub_mid,
            link,
            description,
            fraction,
            ..
        } = m;

        let daily: Vec<DailyMansion> = e.into_iter().map(Into::into).collect();
        Self {
            id: format!("{}.{}", mid, sub_mid),
            cvlink:link,
            description,
            daily: daily,
            fraction,
        }
    }
}

impl
    From<(
        db_ops::daily_mansion::Model,
        Vec<db_ops::mansion_info::Model>,
    )> for DailyMansion
{
    fn from(
        (each, inner): (
            db_ops::daily_mansion::Model,
            Vec<db_ops::mansion_info::Model>,
        ),
    ) -> Self {
        let db_ops::daily_mansion::Model { date, content, .. } = each;
        let inners = inner.into_iter().map(Into::into).collect();
        Self {
            at: date,
            content: content.unwrap_or_default(),
            inners,
        }
    }
}

impl From<db_ops::mansion_info::Model> for Inner {
    fn from(model: db_ops::mansion_info::Model) -> Self {
        let db_ops::mansion_info::Model {
            predict_level,
            info,
            ..
        } = model;
        Self {
            predict: match predict_level {
                db_ops::sea_orm_active_enums::PredictLevel::False => Predict::False,
                db_ops::sea_orm_active_enums::PredictLevel::Unknown => Predict::Unknown,
                db_ops::sea_orm_active_enums::PredictLevel::True => Predict::True,
            },
            info,
        }
    }
}

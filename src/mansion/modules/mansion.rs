use chrono::NaiveDate;

use crate::mansion::db_ops;
#[derive(Debug, serde::Serialize)]
pub struct Mansion {
    id: String,
    from: NaiveDate,
    to: NaiveDate,
    link: String,
    children: Vec<EachMansion>,
}
#[derive(Debug, serde::Serialize)]
pub struct EachMansion {
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
            db_ops::each_mansion::Model,
            Vec<db_ops::inner_mansion::Model>,
        )>,
    )> for Mansion
{
    fn from(
        (m, e): (
            db_ops::mansion::Model,
            Vec<(
                db_ops::each_mansion::Model,
                Vec<db_ops::inner_mansion::Model>,
            )>,
        ),
    ) -> Self {
        let db_ops::mansion::Model {
            mid, sub_mid, link, ..
        } = m;

        let each: Vec<EachMansion> = e.into_iter().map(Into::into).collect();
        let start = each
            .iter()
            .map(|e| e.at)
            .min()
            .unwrap_or(NaiveDate::from_ymd(2017, 5, 1));
        let end = each
            .iter()
            .map(|e| e.at)
            .max()
            .unwrap_or(NaiveDate::from_ymd(2017, 5, 1));
        Self {
            id: format!("{}.{}", mid, sub_mid),
            from: start,
            to: end,
            link,
            children: each,
        }
    }
}

impl
    From<(
        db_ops::each_mansion::Model,
        Vec<db_ops::inner_mansion::Model>,
    )> for EachMansion
{
    fn from(
        (each, inner): (
            db_ops::each_mansion::Model,
            Vec<db_ops::inner_mansion::Model>,
        ),
    ) -> Self {
        let db_ops::each_mansion::Model { date, content, .. } = each;
        let inners = inner.into_iter().map(Into::into).collect();
        Self {
            at: date,
            content: content.unwrap_or_default(),
            inners,
        }
    }
}

impl From<db_ops::inner_mansion::Model> for Inner {
    fn from(model: db_ops::inner_mansion::Model) -> Self {
        let db_ops::inner_mansion::Model {
            id: _,
            eid: _,
            predict_level,
            info,
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

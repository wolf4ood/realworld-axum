use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use sea_orm::{
    sea_query::Nullable, sea_query::ValueType, DbErr, QueryResult, TryGetError, TryGetable,
};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Json<T>(pub T);

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for Json<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Json<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
impl<T: Serialize> From<Json<T>> for sea_orm::Value {
    fn from(json: Json<T>) -> Self {
        sea_orm::Value::Json(Some(Box::new(
            serde_json::to_value(json).expect("It should not fail"),
        )))
    }
}

impl<T> TryGetable for Json<T>
where
    for<'a> T: Deserialize<'a>,
{
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        let val: Option<serde_json::Value> = res.try_get(pre, col).map_err(TryGetError::DbErr)?;
        val.map(|v| {
            serde_json::from_value(v).map_err(|e| TryGetError::DbErr(DbErr::Custom(e.to_string())))
        })
        .ok_or(TryGetError::Null)?
    }
}

impl<T> ValueType for Json<T>
where
    for<'a> T: Deserialize<'a>,
{
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            sea_orm::sea_query::Value::Json(Some(payload)) => {
                serde_json::from_value(*payload).map_err(|_| sea_orm::sea_query::ValueTypeErr)
            }
            sea_orm::sea_query::Value::Json(None) => {
                serde_json::from_value(serde_json::Value::Null)
                    .map_err(|_| sea_orm::sea_query::ValueTypeErr)
            }
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        format!("Json<{}>", std::any::type_name::<T>())
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::JsonBinary
    }
}

impl<T> Nullable for Json<T> {
    fn null() -> sea_orm::Value {
        sea_orm::Value::Json(None)
    }
}

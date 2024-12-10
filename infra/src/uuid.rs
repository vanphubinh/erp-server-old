use sea_orm::TryFromU64;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use short_uuid::ShortUuid;
use uuid::Uuid as OriginalUuid;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
pub struct Uuid(OriginalUuid);

impl<'de> Deserialize<'de> for Uuid {
  fn deserialize<D>(deserializer: D) -> Result<Uuid, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    ShortUuid::parse_str(&s)
      .map(|short_uuid| Uuid(short_uuid.to_uuid()))
      .map_err(serde::de::Error::custom)
  }
}

impl Serialize for Uuid {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&ShortUuid::from_uuid(&self.0).to_string())
  }
}

impl Uuid {
  pub fn new() -> Self {
    Uuid(OriginalUuid::now_v7())
  }
}

impl Into<OriginalUuid> for Uuid {
  fn into(self) -> OriginalUuid {
    self.0.into()
  }
}

impl From<OriginalUuid> for Uuid {
  fn from(v: OriginalUuid) -> Self {
    Uuid(v)
  }
}

impl AsRef<Uuid> for Uuid {
  fn as_ref(&self) -> &Uuid {
    self
  }
}

impl Into<sea_orm::Value> for Uuid {
  fn into(self) -> sea_orm::Value {
    self.0.into()
  }
}

impl sea_orm::TryGetable for Uuid {
  fn try_get_by<I: sea_orm::ColIdx>(
    res: &sea_orm::QueryResult,
    idx: I,
  ) -> Result<Self, sea_orm::TryGetError> {
    let uuid = <OriginalUuid as sea_orm::TryGetable>::try_get_by(res, idx)?;
    Ok(Uuid(uuid))
  }
}

impl TryFromU64 for Uuid {
  fn try_from_u64(n: u64) -> Result<Self, sea_orm::DbErr> {
    Ok(Uuid(OriginalUuid::from_u64_pair(n, 0)))
  }
}

impl sea_orm::sea_query::ValueType for Uuid {
  fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
    <OriginalUuid as sea_orm::sea_query::ValueType>::try_from(v).map(|v| Uuid(v))
  }

  fn type_name() -> String {
    stringify!(Uuid).to_owned()
  }

  fn array_type() -> sea_orm::sea_query::ArrayType {
    sea_orm::sea_query::ArrayType::Uuid
  }

  fn column_type() -> sea_orm::sea_query::ColumnType {
    sea_orm::sea_query::ColumnType::Uuid
  }
}

impl sea_orm::sea_query::Nullable for Uuid {
  fn null() -> sea_orm::Value {
    sea_orm::Value::Uuid(None)
  }
}

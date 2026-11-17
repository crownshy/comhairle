use std::fmt;

use serde::Deserialize;

/// A value that must be explicitly exposed before it can be serialized or described in OpenAPI.
///
/// `Pii` supports deserialization, SQLx, and sea-query, but deliberately does not implement
/// `Serialize` or `JsonSchema`. It does not encrypt data or protect copies made with `expose`.
///
/// ```
/// use comhairle::pii::Pii;
///
/// let email = Pii::new("person@example.org".to_string());
/// assert_eq!(email.expose(), "person@example.org");
/// ```
///
/// A response containing PII cannot be serialized:
///
/// ```compile_fail,E0277
/// use comhairle::pii::Pii;
///
/// #[derive(serde::Serialize)]
/// struct Response {
///     email: Option<Pii<String>>,
/// }
/// ```
///
/// Nor can it be described in OpenAPI:
///
/// ```compile_fail,E0277
/// use comhairle::pii::Pii;
///
/// #[derive(schemars::JsonSchema)]
/// struct Response {
///     email: Option<Pii<String>>,
/// }
/// ```
pub struct Pii<T>(T);

impl<T> Pii<T> {
    /// Wrap a value that should not be serialized implicitly.
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Borrow the protected value for an explicitly reviewed use.
    pub fn expose(&self) -> &T {
        &self.0
    }

    /// Consume the wrapper to expose the protected value.
    pub fn into_exposed(self) -> T {
        self.0
    }
}

impl<T> From<T> for Pii<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> fmt::Debug for Pii<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Pii([REDACTED])")
    }
}

impl<T> fmt::Display for Pii<T>
where
    T: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Pii([REDACTED])")
    }
}

impl<T> Clone for Pii<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for Pii<T> where T: Copy {}

impl<T> Default for Pii<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(T::default())
    }
}

impl<T> PartialEq<T> for Pii<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.0 == other
    }
}

impl<T> PartialEq for Pii<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Pii<T> where T: Eq {}

impl<T: Into<sea_query::Value>> From<Pii<T>> for sea_query::Value {
    fn from(value: Pii<T>) -> Self {
        value.0.into()
    }
}

impl<T: sea_query::Nullable> sea_query::Nullable for Pii<T> {
    fn null() -> sea_query::Value {
        T::null()
    }
}

impl<T, DB: sqlx::Database> sqlx::Type<DB> for Pii<T>
where
    T: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        T::type_info()
    }
}

impl<'conn, T, DB: sqlx::Database> sqlx::Encode<'conn, DB> for Pii<T>
where
    T: sqlx::Encode<'conn, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::Database>::ArgumentBuffer<'conn>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        self.0.encode_by_ref(buf)
    }
}

impl<'conn, T, DB: sqlx::Database> sqlx::Decode<'conn, DB> for Pii<T>
where
    T: sqlx::Decode<'conn, DB>,
{
    fn decode(
        value: <DB as sqlx::Database>::ValueRef<'conn>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(Pii(T::decode(value)?))
    }
}

impl<'conn, T, R: sqlx::Row> sqlx::FromRow<'conn, R> for Pii<T>
where
    T: sqlx::FromRow<'conn, R>,
{
    fn from_row(row: &'conn R) -> Result<Self, sqlx::error::Error> {
        Ok(Pii(T::from_row(row)?))
    }
}

impl<'de, T> Deserialize<'de> for Pii<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Pii(T::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use sea_query::{Alias, PostgresQueryBuilder, Query};
    use sea_query_binder::SqlxBinder;
    use sqlx::PgPool;

    use super::Pii;

    #[test]
    fn deserialized_values_are_redacted_in_debug_output() -> Result<(), Box<dyn Error>> {
        let email: Pii<String> = serde_json::from_str("\"person@example.org\"")?;

        assert_eq!(email.expose(), "person@example.org");
        assert_eq!(format!("{email:?}"), "Pii([REDACTED])");
        assert_eq!(format!("{email:#?}"), "Pii([REDACTED])");

        Ok(())
    }

    #[sqlx::test(migrations = false)]
    async fn database_round_trip_preserves_nullable_values(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let mut connection = pool.acquire().await?;
        sqlx::query("CREATE TEMP TABLE pii_roundtrip (id INTEGER, email TEXT, note TEXT)")
            .execute(&mut *connection)
            .await?;

        sqlx::query("INSERT INTO pii_roundtrip (id, email, note) VALUES ($1, $2, $3)")
            .bind(1_i32)
            .bind(Some(Pii::new("sqlx@example.org".to_string())))
            .bind(None::<Pii<String>>)
            .execute(&mut *connection)
            .await?;

        let (sql, values) = Query::insert()
            .into_table(Alias::new("pii_roundtrip"))
            .columns([Alias::new("id"), Alias::new("email"), Alias::new("note")])
            .values([
                2_i32.into(),
                Pii::new("sea-query@example.org".to_string()).into(),
                Some(Pii::new("private note".to_string())).into(),
            ])?
            .values([
                3_i32.into(),
                Pii::new("nullable@example.org".to_string()).into(),
                None::<Pii<String>>.into(),
            ])?
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values)
            .execute(&mut *connection)
            .await?;

        let rows: Vec<(i32, Pii<String>, Option<Pii<String>>)> =
            sqlx::query_as("SELECT id, email, note FROM pii_roundtrip ORDER BY id")
                .fetch_all(&mut *connection)
                .await?;
        assert_eq!(
            rows,
            vec![
                (1, Pii::new("sqlx@example.org".to_string()), None),
                (
                    2,
                    Pii::new("sea-query@example.org".to_string()),
                    Some(Pii::new("private note".to_string())),
                ),
                (3, Pii::new("nullable@example.org".to_string()), None),
            ]
        );

        Ok(())
    }
}

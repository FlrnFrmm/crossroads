pub(crate) mod error;

use anyhow::{Result, anyhow};
use chrono::NaiveDateTime;
use libsql::{Builder, Row, Rows, params, params::IntoParams};

use crate::configuration::database::Configuration;
use runtime::proxy::{Proxy, ProxyMetadata};

pub struct Database {
    handle: libsql::Database,
}

impl Database {
    pub async fn new(configuration: &Configuration) -> Result<Self> {
        let Configuration { name, path } = configuration;
        let path = format!("{}/{}.sqlite", path, name);
        let handle = Builder::new_local(path).build().await?;
        let database = Database { handle };
        database.init().await?;
        Ok(database)
    }

    async fn init(&self) -> Result<()> {
        let connection = self.handle.connect()?;
        let statements = [
            r#"
            CREATE TABLE IF NOT EXISTS proxys (
                tag TEXT PRIMARY KEY,
                created_at TEXT NOT NULL DEFAULT current_timestamp,
                updated_at TEXT NOT NULL DEFAULT current_timestamp,
                component BLOB NOT NULL
            );"#,
            r#"CREATE TRIGGER IF NOT EXISTS update_proxys_updated_at
            AFTER UPDATE ON proxys
            FOR EACH ROW
            BEGIN
                UPDATE proxys SET updated_at = CURRENT_TIMESTAMP WHERE tag = OLD.tag;
            END;"#,
            r#"CREATE TABLE IF NOT EXISTS current_proxy (
                singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                selected_tag TEXT,
                FOREIGN KEY (selected_tag) REFERENCES proxys(tag)
                    ON DELETE SET NULL
                    ON UPDATE CASCADE
            );"#,
            r#"INSERT OR IGNORE INTO current_proxy (singleton, selected_tag) VALUES (1, NULL);"#,
            r#"CREATE VIEW IF NOT EXISTS proxy AS
            SELECT p.*
            FROM   proxys p
            JOIN   current_proxy s ON p.tag = s.selected_tag
            WHERE  s.singleton = 1;"#,
        ];
        for statement in statements {
            connection.execute(statement, ()).await?;
        }
        Ok(())
    }

    async fn query(&self, statement: &str, params: impl IntoParams) -> Result<Rows> {
        let connection = self.handle.connect()?;
        connection
            .query(statement, params)
            .await
            .map_err(|_| anyhow!("Failed to execute query: {}", statement))
    }

    pub async fn get_current_proxy(&self) -> Result<Option<ProxyMetadata>> {
        let statement = "SELECT * FROM proxy;";
        let mut rows = self.query(statement, ()).await?;
        Self::try_to_proxy_metadata(&mut rows).await
    }

    pub async fn set_current_proxy(&self, tag: &str) -> Result<Option<Proxy>> {
        if self.proxy_exists(tag).await?.is_none() {
            println!("Proxy does not exist");
            return Ok(None);
        }
        let statement = r#"UPDATE current_proxy SET selected_tag = ? WHERE singleton = 1;"#;
        let _ = self.query(statement, params!(tag)).await?;
        self.get_proxy(tag).await
    }

    pub async fn all_proxys(&self) -> Result<Vec<ProxyMetadata>> {
        let statement = "SELECT tag, created_at, updated_at FROM proxys;";
        let mut rows = self.query(statement, ()).await?;
        let mut result = Vec::with_capacity(rows.column_count() as usize);
        while let Some(row) = rows.next().await? {
            let proxy_metadata = Self::try_row_to_proxy_metadata(&row).await?;
            result.push(proxy_metadata);
        }
        Ok(result)
    }

    pub async fn create_proxy(
        &self,
        tag: String,
        component: Vec<u8>,
    ) -> Result<Option<ProxyMetadata>> {
        if self.proxy_exists(&tag).await?.is_some() {
            return Ok(None);
        }
        let statement = "INSERT INTO proxys (tag, component) VALUES (?, ?) RETURNING *;";
        let mut rows = self.query(statement, params![tag, component]).await?;
        Self::try_to_proxy_metadata(&mut rows).await
    }

    pub async fn update_proxy(
        &self,
        tag: String,
        component: Vec<u8>,
    ) -> Result<Option<ProxyMetadata>> {
        let statement = "UPDATE proxys SET component = ? WHERE tag = ? RETURNING *;";
        let mut rows = self.query(statement, params![component, tag]).await?;
        Self::try_to_proxy_metadata(&mut rows).await
    }

    pub async fn delete_proxy(&self, tag: String) -> Result<Option<ProxyMetadata>> {
        let statement = "DELETE FROM proxys WHERE tag = ? RETURNING *;";
        let mut rows = self.query(statement, params![tag]).await?;
        Self::try_to_proxy_metadata(&mut rows).await
    }

    pub async fn proxy_exists(&self, tag: &str) -> Result<Option<ProxyMetadata>> {
        let statement = "SELECT * FROM proxys WHERE tag = ?;";
        let mut rows = self.query(statement, params![tag]).await?;
        Self::try_to_proxy_metadata(&mut rows).await
    }

    pub async fn get_proxy(&self, tag: &str) -> Result<Option<Proxy>> {
        let statement = "SELECT tag, created_at, updated_at, component FROM proxys WHERE tag = ?;";
        let mut rows = self.query(statement, params![tag]).await?;
        Self::try_to_proxy(&mut rows).await
    }

    async fn try_to_proxy_metadata(rows: &mut Rows) -> Result<Option<ProxyMetadata>> {
        if let Some(row) = rows.next().await? {
            let proxy_metdata = Self::try_row_to_proxy_metadata(&row).await?;
            Ok(Some(proxy_metdata))
        } else {
            Ok(None)
        }
    }

    async fn try_row_to_proxy_metadata(row: &Row) -> Result<ProxyMetadata> {
        let tag = row.get::<String>(0)?;
        let date_as_string = row.get::<String>(1)?;
        let native_date = NaiveDateTime::parse_from_str(&date_as_string, "%Y-%m-%d %H:%M:%S")?;
        let created_at = native_date.and_utc().timestamp();
        let date_as_string = row.get::<String>(2)?;
        let native_date = NaiveDateTime::parse_from_str(&date_as_string, "%Y-%m-%d %H:%M:%S")?;
        let updated_at = native_date.and_utc().timestamp();
        let proxy_metadata = ProxyMetadata {
            tag,
            created_at,
            updated_at,
        };
        Ok(proxy_metadata)
    }

    async fn try_to_proxy(rows: &mut Rows) -> Result<Option<Proxy>> {
        if let Some(row) = rows.next().await? {
            let proxy = Self::try_row_to_proxy(&row).await?;
            Ok(Some(proxy))
        } else {
            Ok(None)
        }
    }

    async fn try_row_to_proxy(row: &Row) -> Result<Proxy> {
        let tag = row.get::<String>(0)?;
        let date_as_string = row.get::<String>(1)?;
        let native_date = NaiveDateTime::parse_from_str(&date_as_string, "%Y-%m-%d %H:%M:%S")?;
        let created_at = native_date.and_utc().timestamp();
        let date_as_string = row.get::<String>(2)?;
        let native_date = NaiveDateTime::parse_from_str(&date_as_string, "%Y-%m-%d %H:%M:%S")?;
        let updated_at = native_date.and_utc().timestamp();
        let component = row.get::<Vec<u8>>(3)?;
        let proxy = Proxy {
            metadata: ProxyMetadata {
                tag,
                created_at,
                updated_at,
            },
            component,
        };
        Ok(proxy)
    }
}

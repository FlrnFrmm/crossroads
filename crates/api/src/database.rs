pub mod errors;

use anyhow::{anyhow, Result};
use libsql::{params, params::IntoParams, Builder, Rows};

use crate::{configuration::database::Configuration, road::Road};

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
        let statement = r#"
            CREATE TABLE IF NOT EXISTS roads (
                host TEXT PRIMARY KEY NOT NULL,
                component BLOB NOT NULL
            );
        "#;
        connection.execute(statement, ()).await?;
        Ok(())
    }

    async fn query(&self, statement: &str, params: impl IntoParams) -> Result<Rows> {
        let connection = self.handle.connect()?;
        connection
            .query(statement, params)
            .await
            .map_err(|_| anyhow!("Failed to execute query: {}", statement))
    }

    pub async fn all_roads(&self) -> Result<Vec<Road>> {
        let statement = "SELECT * FROM roads;";
        let mut rows = self.query(statement, ()).await?;
        let mut result = Vec::with_capacity(rows.column_count() as usize);
        while let Some(row) = rows.next().await? {
            let road = Road {
                host: row.get::<String>(0)?,
                component: row.get::<Vec<u8>>(1)?,
            };
            result.push(road);
        }
        Ok(result)
    }

    pub async fn create_road(&self, Road { host, component }: Road) -> Result<Option<Road>> {
        if self.road_exists(&host).await?.is_some() {
            return Ok(None);
        }
        let statement = "INSERT INTO roads (host, component) VALUES (?, ?) RETURNING *";
        let mut rows = self.query(statement, params![host, component]).await?;
        Self::maybe_road(&mut rows).await
    }

    pub async fn update_road(&self, Road { host, component }: Road) -> Result<Option<Road>> {
        let statement = "UPDATE roads SET component = ? WHERE host = ? RETURNING *;";
        let mut rows = self.query(statement, params![component, host]).await?;
        Self::maybe_road(&mut rows).await
    }

    pub async fn delete_road(&self, host: String) -> Result<Option<Road>> {
        let statement = "DELETE FROM roads WHERE host = ? RETURNING *;";
        let mut rows = self.query(statement, params![host]).await?;
        Self::maybe_road(&mut rows).await
    }

    pub async fn road_exists(&self, host: &str) -> Result<Option<Road>> {
        let statement = "SELECT * FROM roads WHERE host = ?;";
        let mut rows = self.query(statement, params![host]).await?;
        Self::maybe_road(&mut rows).await
    }

    async fn maybe_road(rows: &mut Rows) -> Result<Option<Road>> {
        if let Some(row) = rows.next().await? {
            let road = Road {
                host: row.get::<String>(0)?,
                component: row.get::<Vec<u8>>(1)?,
            };
            Ok(Some(road))
        } else {
            Ok(None)
        }
    }
}

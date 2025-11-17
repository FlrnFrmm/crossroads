use anyhow::{Result, anyhow, bail};
use runtime::proxy::Proxy;
use tokio::fs;
use uuid::Uuid;

use api::{configuration::database::Configuration, database::Database};

#[tokio::test]
async fn after_setup_all_proxies_empty() -> Result<()> {
    let database = &DatabaseWrapper::setup().await?.database;

    let proxies = database.all_proxys().await?;
    proxies.iter().for_each(|f| println!("{}", f.tag));
    assert_eq!(proxies.len(), 0);

    Ok(())
}

#[tokio::test]
async fn create_new_proxy() -> Result<()> {
    let database = &DatabaseWrapper::setup().await?.database;

    const TAG: &str = "alpha:v1.0.0";
    let component = vec![0; 10];
    let maybe_proxy_metadata = database
        .create_proxy(TAG.to_string(), component.clone())
        .await?;
    assert!(maybe_proxy_metadata.is_some());
    let Some(proxy_metadata) = maybe_proxy_metadata else {
        bail!("Impossible ... Some is None after check");
    };
    assert_eq!(proxy_metadata.tag, TAG.to_string());
    assert_eq!(proxy_metadata.created_at, proxy_metadata.updated_at);

    let maybe_proxy = database.get_proxy(TAG).await?;
    let Some(proxy) = maybe_proxy else {
        bail!("Proxy does not exist after creation");
    };
    for (left, right) in component.iter().zip(proxy.component.iter()) {
        assert_eq!(*left, *right);
    }

    Ok(())
}

#[tokio::test]
async fn single_proxy_exists() -> Result<()> {
    let database = &DatabaseWrapper::setup().await?.database;

    const TAG: &str = "alpha:v1.0.0";

    let proxy = database.proxy_exists(TAG).await?;
    assert!(proxy.is_none());

    let component = vec![0; 10];
    let maybe_proxy_metadata = database.create_proxy(TAG.to_string(), component).await?;
    assert!(maybe_proxy_metadata.is_some());

    let maybe_proxy_metadata = database.proxy_exists(TAG).await?;
    assert!(maybe_proxy_metadata.is_some());

    Ok(())
}

#[tokio::test]
async fn all_proxies() -> Result<()> {
    let database = &DatabaseWrapper::setup().await?.database;

    let road = database.all_proxys().await.unwrap();
    assert_eq!(road.len(), 0);

    let tags = vec!["alpha:v1.0.0", "beta:v1.0.0", "gamma:v1.0.0"];
    for tag in tags.iter() {
        let component = vec![0; 10];
        let maybe_proxy_metadata = database.create_proxy(tag.to_string(), component).await?;
        assert!(maybe_proxy_metadata.is_some());
    }

    let proxies_metadata = database.all_proxys().await?;
    assert_eq!(proxies_metadata.len(), tags.len());
    for proxy_metadata in proxies_metadata.iter() {
        assert!(tags.contains(&proxy_metadata.tag.as_str()));
    }

    Ok(())
}

#[tokio::test]
async fn update_road() -> Result<()> {
    let database = &DatabaseWrapper::setup().await?.database;

    const TAG: &str = "alpha:v1.0.0";
    let component = vec![0; 10];
    let maybe_proxy_metadata = database
        .create_proxy(TAG.to_string(), component.clone())
        .await?;
    assert!(maybe_proxy_metadata.is_some());

    let maybe_proxy = database.get_proxy(TAG).await?;
    let Some(proxy) = maybe_proxy else {
        bail!("Proxy does not exist after creation");
    };
    for (left, right) in component.iter().zip(proxy.component.iter()) {
        assert_eq!(*left, *right);
    }

    let component = vec![1; 10];

    let proxy_metadata = database
        .update_proxy(TAG.to_string(), component.clone())
        .await?;
    assert!(proxy_metadata.is_some());

    let maybe_proxy = database.get_proxy(TAG).await?;
    let Some(proxy) = maybe_proxy else {
        bail!("Proxy does not exist after creation");
    };
    for (left, right) in component.iter().zip(proxy.component.iter()) {
        assert_eq!(*left, *right);
    }

    Ok(())
}

#[tokio::test]
async fn delete_proxy() -> Result<()> {
    let database = &DatabaseWrapper::setup().await?.database;

    const TAG: &str = "alpha:v1.0.0";
    let proxy_metadata = database.delete_proxy(TAG.to_string()).await?;
    assert!(proxy_metadata.is_none());

    let component = vec![0; 10];
    let maybe_proxy_metadata = database
        .create_proxy(TAG.to_string(), component.clone())
        .await?;
    assert!(maybe_proxy_metadata.is_some());

    let maybe_proxy_metadata = database.proxy_exists(TAG).await?;
    assert!(maybe_proxy_metadata.is_some());

    let proxy_metadata = database.delete_proxy(TAG.to_string()).await?;
    assert!(proxy_metadata.is_some());

    let maybe_proxy_metadata = database.proxy_exists(TAG).await?;
    assert!(maybe_proxy_metadata.is_none());

    Ok(())
}

#[tokio::test]
async fn get_not_set_current_proxy() -> Result<()> {
    let database = &DatabaseWrapper::setup().await?.database;

    const TAG: &str = "alpha:v1.0.0";
    let maybedatabase = database.get_proxy(TAG).await?;
    assert!(maybedatabase.is_none());

    Ok(())
}

#[tokio::test]
async fn set_current_proxy_to_nonexisting_proxy() -> Result<()> {
    let database = &DatabaseWrapper::setup().await?.database;

    const TAG: &str = "alpha:v1.0.0";
    let maybe_proxy = database.set_current_proxy(TAG).await?;
    assert!(maybe_proxy.is_none());

    Ok(())
}

#[tokio::test]
async fn set_current_proxy() -> Result<()> {
    let database = &DatabaseWrapper::setup().await?.database;

    const TAG: &str = "alpha:v1.0.0";
    let component = vec![0; 10];
    let maybe_proxy_metadata = database
        .create_proxy(TAG.to_string(), component.clone())
        .await?;
    assert!(maybe_proxy_metadata.is_some());

    let Some(created_current_proxy) = database.set_current_proxy(TAG).await? else {
        panic!("Created current proxy does not exist after setting")
    };

    let Some(current_proxy_metadata) = database.get_current_proxy().await? else {
        panic!("Current proxy metadata does not exist after setting")
    };

    let Some(current_proxy) = database.get_proxy(&current_proxy_metadata.tag).await? else {
        panic!("Current proxy does not exist after setting")
    };

    assert_eq!(
        created_current_proxy.metadata.tag,
        current_proxy.metadata.tag
    );

    for (left, right) in created_current_proxy
        .component
        .iter()
        .zip(current_proxy.component.iter())
    {
        assert_eq!(*left, *right);
    }
    Ok(())
}

struct DatabaseWrapper {
    uuid: Uuid,
    database: Database,
}

impl DatabaseWrapper {
    pub async fn setup() -> Result<Self> {
        let uuid = Uuid::new_v4();
        let configuration = Configuration {
            name: uuid.to_string(),
            path: ".".to_string(),
        };
        let database = Database::new(&configuration).await?;
        Ok(Self { uuid, database })
    }
}

impl Drop for DatabaseWrapper {
    fn drop(&mut self) {
        let path = format!("./{}.sqlite", self.uuid);
        match std::fs::remove_file(path) {
            Ok(_) => (),
            Err(e) => println!("Error removing sqlite db file: {}", e),
        }
    }
}

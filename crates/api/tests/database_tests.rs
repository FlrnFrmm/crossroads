use std::{fs, io};

use api::{configuration::database::Configuration, database::Database, proxy::Proxy};

 
#[tokio::test]
async fn test_all_proxies_empty()  {
    let _ = cleanup();
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();

    let proxies = database.all_proxies().await.unwrap();
    let _ = proxies.iter().map(|f| format!("{}",f.host));
    assert_eq!(proxies.len(),0);

 
}

 
#[tokio::test]
async fn test_create_proxy() {
    let _ = cleanup();
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();
 
    let proxy = database.create_proxy("example.com".to_string(), Vec::new()).await.unwrap();
    assert!(proxy.is_some());
    
    let proxy = database.create_proxy("example.com".to_string(), Vec::new()).await.unwrap();
    assert!(proxy.is_none());
}


#[tokio::test]
async fn test_single_proxy_exists() {
    let _ = cleanup();
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();

    let proxy = database.proxy_exists("example.com").await.unwrap();
    assert!(proxy.is_none());

     let proxy = database.create_proxy("example.com".to_string(), Vec::new()).await.unwrap();
    assert!(proxy.is_some());
    
    let proxy = database.proxy_exists("example.com").await.unwrap();
    assert!(proxy.is_some());
 
}

#[tokio::test]
async fn test_all_proxies_populated()  {
    let _ = cleanup();
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();

    let proxies = database.all_proxies().await.unwrap();
    assert_eq!(proxies.len(), 0);

    let proxy = database.create_proxy("example.com".to_string(), Vec::new()).await.unwrap();
    assert!(proxy.is_some());
 
    let proxies = database.all_proxies().await.unwrap();
    assert_eq!(proxies.len(), 1);
 
}

#[tokio::test]
async fn test_update_proxy(){
    let _ = cleanup();
    let config = Configuration{name:"tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();
 
    let proxy = database.create_proxy("example.com".to_string(), Vec::new()).await.unwrap();
    assert!(proxy.is_some());

    let proxies = database.all_proxies().await.unwrap();
    assert_eq!(proxies.len(), 1);

    let mut component = Vec::new();
    component.push(1);

     let proxy = database.update_proxy("example.com",component).await.unwrap();
    assert!(proxy.is_some());
    
 
}

#[tokio::test]
async fn test_delete_proxy(){
    let _ = cleanup();
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();

    let proxy =  database.delete_proxy("example.com".to_string()).await.unwrap();
    assert!(proxy.is_none());

    let proxy = database.create_proxy("example.com".to_string(), Vec::new()).await.unwrap();
    assert!(proxy.is_some());

    let proxy = database.proxy_exists("example.com").await.unwrap();
    assert!(proxy.is_some());
 
    let result = database.delete_proxy("example.com".to_string()).await.unwrap();
    assert!(result.is_some());
 
    let proxy = database.proxy_exists("example.com").await.unwrap();
    assert!(proxy.is_some());
}
 

  fn cleanup() -> io::Result<()> {
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let path = format!("{}/{}.sqlite", config.path, config.name);
    fs::remove_file(path)
}


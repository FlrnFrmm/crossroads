use std::{fs, io};

use api::{configuration::database::Configuration, database::Database, road::Road};

 
#[tokio::test]
async fn test_all_roads_empty()  {
    let _ = cleanup();
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();

    let road = database.all_roads().await.unwrap();
    let _ = road.iter().map(|f| format!("{}",f.host));
    assert_eq!(road.len(),0);

 
}

 
#[tokio::test]
async fn test_create_road_() {
    let _ = cleanup();
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();
 
    let another_road = Road{component: Vec::new(),host: "example.com".to_string()};
    let road = database.create_road(another_road).await.unwrap();
    assert!(road.is_some());
    
    let another_road = Road{component: Vec::new(),host: "example.com".to_string()};
    let road: Option<Road> = database.create_road(another_road).await.unwrap();
    assert!(road.is_none());
 
}


#[tokio::test]
async fn test_single_road_exists() {
    let _ = cleanup();
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();

    let road = database.road_exists("example.com").await.unwrap();
    assert!(road.is_none());

    let another_road = Road{component: Vec::new(),host: "example.com".to_string()};
    let road = database.create_road(another_road).await.unwrap();
    assert!(road.is_some());
    
    let road = database.road_exists("example.com").await.unwrap();
    assert!(road.is_some());
 
}

#[tokio::test]
async fn test_all_roads_populated()  {
    let _ = cleanup();
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();

    let road = database.all_roads().await.unwrap();
    assert_eq!(road.len(), 0);

    let another_road = Road{component: Vec::new(),host: "example.com".to_string()};
    let road = database.create_road(another_road).await.unwrap();
    assert!(road.is_some());

    let road = database.all_roads().await.unwrap();
    assert_eq!(road.len(), 1);
 
}

#[tokio::test]
async fn test_update_road(){
    let _ = cleanup();
    let config = Configuration{name:"tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();
 
    let another_road = Road{component: Vec::new(),host: "example.com".to_string()};
    let road = database.create_road(another_road).await.unwrap();
    assert!(road.is_some());

    let road = database.all_roads().await.unwrap();
    assert_eq!(road.len(), 1);

    let mut component = Vec::new();
    component.push(1);

    let another_road = Road{component: component,host: "example.com".to_string()};
    let road = database.update_road(another_road).await.unwrap();
    assert!(road.is_some());
    let road = road.unwrap();
    assert_eq!(road.component.len(), 1);

 
}

#[tokio::test]
async fn test_delete_road(){
    let _ = cleanup();
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let database : Database =  Database::new(&config).await.unwrap();

    let road =  database.delete_road("example.com".to_string()).await;
    assert!(road.is_ok());
    assert!(road.unwrap().is_none());

    let another_road = Road{component: Vec::new(),host: "example.com".to_string()};
    let road = database.create_road(another_road).await.unwrap();
    assert!(road.is_some());

    let road = database.road_exists("example.com").await;
    assert!(road.is_ok());
    assert!(road.unwrap().is_some());

    let result = database.delete_road("example.com".to_string()).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());

    let road = database.road_exists("example.com").await;
    assert!(road.is_ok());
    assert!(road.unwrap().is_none());
}
 

  fn cleanup() -> io::Result<()> {
    let config = Configuration{name: "tests".to_string(),path: ".".to_string()};
    let path = format!("{}/{}.sqlite", config.path, config.name);
    fs::remove_file(path)
}


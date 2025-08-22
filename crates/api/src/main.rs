use anyhow::Result;

use api::road::event::Event as RoadEvent;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Default::default();
    let (api, mut receiver) = api::API::new(&configuration).await?;
    tokio::spawn(async move {
        while let Some(event) = receiver.recv().await {
            match event {
                RoadEvent::Create(road) => println!("Road created: {:?}", road),
                RoadEvent::Update(road) => println!("Road updated: {:?}", road),
                RoadEvent::Delete(road) => println!("Road deleted: {:?}", road),
            }
        }
    });
    api.run().await
}

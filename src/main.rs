use kube::Client;
use kube::CustomResource;
use serde::{Serialize, Deserialize};
use kube::api::{Api, Meta, ListParams};
use kube_runtime::watcher;
use futures::{StreamExt, TryStreamExt};

#[derive(CustomResource, Serialize, Deserialize, Default, Debug, PartialEq, Clone)]
#[kube(group = "ramonberrutti.github.com", version = "v1", namespaced)]
pub struct BookSpec {
    title: String,
    author: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    let books = Api::<Book>::namespaced(client, "default");

    let lp = ListParams::default();
    println!("Current books:");
    for b in books.list(&lp).await? {
        println!("Book ({}): Title: {}, Author: {}", Meta::name(&b), b.spec.title, b.spec.author);
    }

    println!("Watch book's changes:");
    let mut w = watcher(books, lp).boxed();
    while let Some(event) = w.try_next().await? {
        use kube_runtime::watcher::Event;
        match event {
            Event::Applied(b) => println!("Created({}): Title: {}, Author: {}", Meta::name(&b), b.spec.title, b.spec.author),
            Event::Deleted(b) => println!("Deleted({}): Title: {}, Author: {}", Meta::name(&b), b.spec.title, b.spec.author),
            _ => (), // Ignore Restarted (We already use list)
        };
    }
 
    Ok(())
}

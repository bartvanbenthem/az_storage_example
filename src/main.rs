use azure_storage::prelude::*;
use azure_storage_blobs::{prelude::*, container::operations::BlobItem};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {

    // First we retrieve the account name and access key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");

    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
    let client = ClientBuilder::new(&account, storage_credentials.clone()).blob_service_client();
    
    let containers = list_containers(client);
    for container in containers.await {
        println!();

        let container_client = ClientBuilder::new(&account, 
            storage_credentials.clone()).container_client(&container);
    
        println!("{}", container_client.container_name());
        for _ in 0..container_client.container_name().len() {
            print!("-")
        }
        println!();
    
        let blobs = list_blobs(container_client);
        for blob in blobs.await {
            println!("{}", blob)
        }
    }

    Ok(())
}

async fn list_containers(client: BlobServiceClient) -> Vec<String>{
    let mut containers_list: Vec<String> = vec![];
    
    let mut stream = client.list_containers().into_stream();
    while let Some(list) = stream.next().await {
        match list {
            Ok(response) => {
                for container in response.containers {
                  containers_list.push(container.name)
                }
            }
            Err(error) => {
                eprintln!("{:?}", error)
            }
        }
    }

    return containers_list
}

async fn list_blobs(client: ContainerClient) -> Vec<String>{

    let mut blob_list: Vec<String> = vec![];

    let mut stream = client.list_blobs().into_stream();
    while let Some(list) = stream.next().await {
        match list {
            Ok(response) => {
                for item in response.blobs.items {
                    match item {
                        BlobItem::Blob(blob) => {
                            blob_list.push(blob.name);
                        }
                        _ => {}
                    }
                }
            }
            Err(error) => {
                eprintln!("{:?}", error)
            }
        }
    }

    return blob_list

}

/*
//use azure_core::error::{ErrorKind, ResultExt};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {

    // First we retrieve the account name and access key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
    let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");
    let container = std::env::var("STORAGE_CONTAINER").expect("missing STORAGE_CONTAINER");
    let blob_name = std::env::var("STORAGE_BLOB_NAME").expect("missing STORAGE_BLOB_NAME");

    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
    let blob_client = ClientBuilder::new(account, storage_credentials).blob_client(&container, blob_name);

    blob_client.put_block_blob("hello world").content_type("text/plain").await?;

    let mut result: Vec<u8> = vec![];

    // The stream is composed of individual calls to the get blob endpoint
    let mut stream = blob_client.get().into_stream();
    while let Some(value) = stream.next().await {
        let mut body = value?.data;
        // For each response, we stream the body instead of collecting it all
        // into one large allocation.
        while let Some(value) = body.next().await {
            let value = value?;
            result.extend(&value);
        }
    }

    println!("result: {:?}", result);

    Ok(())
}

 */
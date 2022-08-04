use async_trait::async_trait;
use mongodb::Collection;

#[async_trait]
pub trait Generator<T> {
    async fn generate(size: u32) -> Vec<T>;
}

#[async_trait]
pub trait COSICollection<T> {
    async fn get_collection() -> Collection<T>;
}

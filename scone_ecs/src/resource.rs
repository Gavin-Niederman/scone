pub struct Resouce<T: Send + Sync + ?Sized> {
    pub data: Box<T>,
}

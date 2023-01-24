pub struct Resouce<T: Send + ?Sized> {
    pub data: Box<T>,
}

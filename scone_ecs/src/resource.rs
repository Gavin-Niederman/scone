pub struct Resouce<T: ResourceType + ?Sized> {
    pub data: Box<T>,
}

pub trait ResourceType: Send {}
impl<T: ?Sized + Send> ResourceType for T {}
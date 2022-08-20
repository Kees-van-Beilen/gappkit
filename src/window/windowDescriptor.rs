pub struct WindowDescriptor{
    pub title:String
}
impl From<&str> for WindowDescriptor{
    fn from(name: &str) -> Self {
        return Self {title:name.to_string(), ..Default::default()};
    }
}
impl From<()> for WindowDescriptor{
    fn from(_: ()) -> Self {
        return Self::default();
    }
}
impl Default for WindowDescriptor{
    fn default()->Self{
        return WindowDescriptor{
            title:"window".to_string()
        }
    }
}
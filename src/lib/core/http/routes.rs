#[derive(Debug)]
pub struct RouteInfo<'a, 'b> {
    pub path: &'a str,
    pub to: &'b str,
}

impl<'a, 'b> RouteInfo<'a, 'b> {
    pub fn new(path: &'a str, to: &'b str) -> Self {
        Self { path, to }
    }
}

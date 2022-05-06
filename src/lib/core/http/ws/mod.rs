use ws::listen;
pub struct WsHandler {
    pub to: String
}


impl WsHandler {
    pub fn new(to: &str) -> Self {
        Self {
            to: to.to_string()
        }
    }

    pub fn out(&self)  {
        listen(&self.to, |s| {
            move |msg| {
                s.send(msg)
            }
        }).unwrap();
    }
}
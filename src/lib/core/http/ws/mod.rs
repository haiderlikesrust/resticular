use ws::{connect, CloseCode};
pub struct WsHandler {
    pub to: String,
}

impl WsHandler {
    pub fn new(to: &str) -> Self {
        Self { to: to.to_string() }
    }

    pub fn out(self) {
        connect("http://0.0.0.0:4200", |out| {
            out.send("Hello WebSocket").unwrap();

            move |msg| {
                println!("Got message: {}", msg);
                out.close(CloseCode::Normal)
            }
        })
        .unwrap()
    }
}

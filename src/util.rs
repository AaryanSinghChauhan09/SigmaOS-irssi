//! irssi model — terminal IRC client.

#[derive(Debug, Clone)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub nick: String,
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub topic: String,
    pub users: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub channel: String,
    pub nick: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub servers: Vec<Server>,
    pub channels: Vec<Channel>,
    pub messages: Vec<Message>,
}

impl Default for Model {
    fn default() -> Self { Self { servers: vec![], channels: vec![], messages: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_server(&mut self, server: Server) { self.servers.push(server); }

    pub fn join(&mut self, channel: &str) {
        if !self.channels.iter().any(|c| c.name == channel) {
            self.channels.push(Channel { name: channel.into(), topic: String::new(), users: vec![] });
        }
    }

    pub fn send(&mut self, channel: &str, nick: &str, body: &str) {
        self.messages.push(Message { channel: channel.into(), nick: nick.into(), body: body.into() });
    }

    pub fn list_channels(&self) -> &[Channel] { &self.channels }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn server_join_send() {
        let mut i = Model::new();
        i.add_server(Server { name: "libera".into(), host: "irc.libera.chat".into(), port: 6697, nick: "bob".into() });
        i.join("#rust");
        i.send("#rust", "bob", "hi");
        assert_eq!(i.list_channels().len(), 1);
    }
}

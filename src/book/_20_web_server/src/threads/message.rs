pub enum Message {
    Execute(Job),
    Terminate,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

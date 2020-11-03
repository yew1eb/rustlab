pub mod kafka;
pub mod stdin;

pub trait Source {
    fn start(&self);
}

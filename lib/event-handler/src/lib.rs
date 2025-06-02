use async_channel::{unbounded};
use futures::executor;

pub struct EventHandler<E> {
    sender: async_channel::Sender<E>,
    receiver: async_channel::Receiver<E>,
}

impl<E> Default for EventHandler<E> {
    fn default() -> Self {
        let (sender, receiver) = unbounded::<E>();

        Self {
            sender,
            receiver,
        }
    }
}

impl<E: Clone> EventHandler<E> {
    pub fn dispatch(&self, event: E) {
        let send = self.sender.send(event);

        executor::block_on(async move {
            send.await.expect("Can't dispatch event");
        });
    }

    pub fn receiver(&self) -> async_channel::Receiver<E> {
        self.receiver.clone()
    }
}

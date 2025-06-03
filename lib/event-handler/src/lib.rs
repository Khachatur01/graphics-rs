pub use async_channel::Receiver;
use async_channel::Sender;

use async_channel::unbounded;
use futures::executor;

pub struct EventChannel<E> {
    sender: Sender<E>,
    receiver: Receiver<E>,
}

impl<E> Default for EventChannel<E> {
    fn default() -> Self {
        let (sender, receiver) = unbounded::<E>();

        Self {
            sender,
            receiver,
        }
    }
}

impl<E: Clone> EventChannel<E> {
    pub fn dispatch(&self, event: E) {
        executor::block_on(self.sender.send(event)).expect("Can't dispatch event");
    }

    pub fn receiver(&self) -> Receiver<E> {
        self.receiver.clone()
    }
}

#[macro_export]
macro_rules! make_event_handler {
    ($($element: ident: $ty: ty),*) => {
        #[derive(Default)]
        pub struct EventHandler {
            $($element: EventChannel<$ty>),*
        }

        impl EventHandler {
            $(
                pub fn $element(&self) -> Receiver<$ty> {
                    self.$element.receiver()
                }
            )*
        }
    }
}

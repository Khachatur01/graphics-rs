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
    ($name:ident $(<$($generic_param:ident $(: $generic_bound:path)*),*>)*, $($element:ident: $ty:ty),*) => {
        pub struct $name $(< $($generic_param $(: $generic_bound)*),* >)* {
            $($element: EventChannel<$ty>),*,
        }

        impl $(< $($generic_param $(: $generic_bound)*),* >)* $name $(< $($generic_param),* >)* {
            $(
                pub fn $element(&self) -> Receiver<$ty> {
                    self.$element.receiver()
                }
            )*
        }

        impl $(< $($generic_param $(: $generic_bound)*),* >)* Default for $name $(< $($generic_param),* >)* {
            fn default() -> Self {
                Self {
                    $($element: Default::default()),*,
                }
            }
        }
    }
}

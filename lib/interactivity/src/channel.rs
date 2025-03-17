pub struct Channel<T: Clone> {
    callback: Vec<Box<dyn Fn(T)>>,
}

impl<T: Clone> Channel<T> {
    pub fn subscribe<Callback: Fn(T) + 'static>(&mut self, callback: Callback) {
        self.callback.push(Box::new(callback));
    }

    pub(crate) fn send(&self, item: T) {
        self.callback.iter().for_each(|callback| {
            callback(item.clone());
        });
    }
}

impl<T: Clone> Default for Channel<T> {
    fn default() -> Self {
        // let (sender, receiver) = unbounded();

        Self {
            callback: Vec::new()
        }

        // Self {
        //     tx: sender,
        //     rx: receiver
        // }
    }
}

use gtk::glib;

#[derive(Default, Debug)]
pub struct CancellableAsyncTasks<R: 'static> {
    tasks: Vec<glib::JoinHandle<R>>,
}

impl<R: 'static> CancellableAsyncTasks<R> {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn spawn_task<F: Future<Output = R> + 'static>(&mut self, task: F) {
        self.tasks.push(glib::spawn_future_local(task));
    }

    pub fn cancel_all(&mut self) {
        for task in self.tasks.iter() {
            task.abort();
        }

        self.tasks.clear();
    }
}
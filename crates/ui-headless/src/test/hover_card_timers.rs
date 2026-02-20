use std::cell::RefCell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TestTimeoutHandle {
    id: u64,
}

impl TestTimeoutHandle {
    pub fn clear(self) {
        TEST_SCHEDULER.with(|scheduler| scheduler.borrow_mut().cancel(self.id));
    }
}

struct Task {
    id: u64,
    due_ms: u64,
    callback: Option<Box<dyn FnOnce()>>,
}

struct Scheduler {
    now_ms: u64,
    next_id: u64,
    tasks: Vec<Task>,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            now_ms: 0,
            next_id: 1,
            tasks: Vec::new(),
        }
    }

    fn set_timeout(
        &mut self,
        delay_ms: u64,
        callback: impl FnOnce() + 'static,
    ) -> TestTimeoutHandle {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        let due_ms = self.now_ms.saturating_add(delay_ms);
        self.tasks.push(Task {
            id,
            due_ms,
            callback: Some(Box::new(callback)),
        });
        TestTimeoutHandle { id }
    }

    fn cancel(&mut self, id: u64) {
        self.tasks.retain(|task| task.id != id);
    }

    fn take_due(&mut self) -> Vec<Box<dyn FnOnce()>> {
        let now_ms = self.now_ms;
        let mut callbacks = Vec::new();
        self.tasks.retain_mut(|task| {
            if task.due_ms <= now_ms {
                if let Some(callback) = task.callback.take() {
                    callbacks.push(callback);
                }
                false
            } else {
                true
            }
        });
        callbacks
    }
}

thread_local! {
    static TEST_SCHEDULER: RefCell<Scheduler> = RefCell::new(Scheduler::new());
}

pub fn set_timeout(delay_ms: u64, callback: impl FnOnce() + 'static) -> TestTimeoutHandle {
    TEST_SCHEDULER.with(|scheduler| scheduler.borrow_mut().set_timeout(delay_ms, callback))
}

pub fn advance_by(delta_ms: u64) {
    TEST_SCHEDULER.with(|scheduler| {
        let mut scheduler = scheduler.borrow_mut();
        scheduler.now_ms = scheduler.now_ms.saturating_add(delta_ms);
    });

    loop {
        let callbacks = TEST_SCHEDULER.with(|scheduler| scheduler.borrow_mut().take_due());
        if callbacks.is_empty() {
            break;
        }
        for callback in callbacks {
            callback();
        }
    }
}

pub fn reset() {
    TEST_SCHEDULER.with(|scheduler| *scheduler.borrow_mut() = Scheduler::new());
}

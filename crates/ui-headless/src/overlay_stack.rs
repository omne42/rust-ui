use leptos::prelude::*;

#[derive(Clone)]
pub struct OverlayStack {
    stack: ReadSignal<Vec<u64>>,
    set_stack: WriteSignal<Vec<u64>>,
    next_id: ReadSignal<u64>,
    set_next_id: WriteSignal<u64>,
}

#[derive(Clone)]
pub struct OverlayRegistration {
    pub is_topmost: ReadSignal<bool>,
}

pub fn provide_overlay_stack() -> OverlayStack {
    let (stack, set_stack) = signal(Vec::<u64>::new());
    let (next_id, set_next_id) = signal(1_u64);

    let state = OverlayStack {
        stack,
        set_stack,
        next_id,
        set_next_id,
    };

    provide_context(state.clone());

    state
}

pub fn use_overlay_stack() -> Option<OverlayStack> {
    use_context::<OverlayStack>()
}

pub fn use_overlay_stack_registration() -> OverlayRegistration {
    if let Some(stack) = use_overlay_stack() {
        stack.register()
    } else {
        OverlayRegistration {
            is_topmost: signal(true).0,
        }
    }
}

impl OverlayStack {
    pub fn register(&self) -> OverlayRegistration {
        let id = self.next_id.get_untracked();
        self.set_next_id.update(|n| *n += 1);
        self.set_stack.update(|s| s.push(id));

        let (is_topmost, set_topmost) = signal(true);
        let stack = self.stack;

        Effect::new(move |_| {
            let top = stack.get().last().copied();
            set_topmost.set(top == Some(id));
        });

        let set_stack = self.set_stack;
        on_cleanup(move || {
            set_stack.update(|s| s.retain(|v| *v != id));
        });

        OverlayRegistration { is_topmost }
    }
}

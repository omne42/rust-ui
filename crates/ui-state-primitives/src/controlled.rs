use std::sync::Arc;

pub type ControlledOnChange<T> = Arc<dyn Fn(T) + Send + Sync>;

#[derive(Clone)]
pub struct ControlledState<T>
where
    T: Clone + PartialEq,
{
    value: T,
    default_value: T,
    is_controlled: bool,
    on_change: Option<ControlledOnChange<T>>,
}

#[derive(Clone, Default)]
pub struct ControlledStateOptions<T> {
    pub value: Option<T>,
    pub default_value: Option<T>,
    pub on_change: Option<ControlledOnChange<T>>,
}

pub fn use_controlled_state<T>(initial: T, options: ControlledStateOptions<T>) -> ControlledState<T>
where
    T: Clone + PartialEq,
{
    let ControlledStateOptions {
        value,
        default_value,
        on_change,
    } = options;

    let is_controlled = value.is_some();
    let value = value.clone().or(default_value.clone()).unwrap_or(initial);
    let default_value = default_value.unwrap_or_else(|| value.clone());

    ControlledState {
        value,
        default_value,
        is_controlled,
        on_change,
    }
}

impl<T> ControlledState<T>
where
    T: Clone + PartialEq,
{
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn default_value(&self) -> &T {
        &self.default_value
    }

    pub fn is_controlled(&self) -> bool {
        self.is_controlled
    }

    pub fn sync_controlled(&mut self, value: Option<T>) {
        self.is_controlled = value.is_some();
        if let Some(value) = value {
            self.value = value;
        }
    }

    pub fn set_value(&mut self, value: T) {
        if value == self.value {
            return;
        }

        if let Some(on_change) = &self.on_change {
            on_change(value.clone());
        }

        if !self.is_controlled {
            self.value = value;
        }
    }
}

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

use leptos::prelude::*;

mod common;

pub use common::CommonStrings;

#[derive(Clone, Debug, Default)]
pub struct UiI18n {
    bundles: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl UiI18n {
    pub fn with_strings<T>(mut self, strings: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        self.bundles.insert(TypeId::of::<T>(), Arc::new(strings));
        self
    }

    pub fn strings<T>(&self) -> Arc<T>
    where
        T: Default + Send + Sync + 'static,
    {
        self.bundles
            .get(&TypeId::of::<T>())
            .cloned()
            .and_then(|any| any.downcast::<T>().ok())
            .unwrap_or_else(|| Arc::new(T::default()))
    }
}

pub fn provide_ui_i18n(i18n: UiI18n) {
    provide_context(i18n);
}

pub fn use_ui_i18n() -> UiI18n {
    use_context::<UiI18n>().unwrap_or_default()
}

use std::{any::Any, sync::Arc};

use crate::{app::Senders, tree::ElementId, widget::ui_message::UIMessage};

use super::widget_tree::WidgetTree;

pub struct UIContext<'a> {
    id: ElementId,
    state: Option<&'a (dyn Any + Send)>,
    element_tree: &'a WidgetTree,
    senders: Senders,
    pub ui_messages: Vec<UIMessage>,
}

impl<'a> UIContext<'a> {
    pub fn new(
        id: ElementId,
        state: Option<&'a (dyn Any + Send)>,
        element_tree: &'a WidgetTree,
        senders: Senders,
    ) -> Self {
        Self {
            id,
            state,
            element_tree,
            senders,
            ui_messages: vec![],
        }
    }

    pub fn send_internal_message(&mut self, message: UIMessage) {
        self.ui_messages.push(message);
    }

    pub fn state<T>(&self) -> Option<&T>
    where
        T: 'static,
    {
        if let Some(state) = self.state {
            state.downcast_ref::<T>()
        } else {
            None
        }
    }

    pub fn id(&self) -> ElementId {
        self.id
    }

    pub fn child_id(&self, index: usize) -> ElementId {
        self.element_tree[self.id].children[index]
    }

    pub fn set_state<T>(&mut self, modify: impl Fn(&T) -> T + Send + 'static)
    where
        T: Any + Send + 'static,
    {
        self.senders
            .state_update_queue()
            .send((
                self.id(),
                Box::new(move |any| Arc::new(modify(any.downcast_ref::<T>().unwrap()))),
            ))
            .expect("Send state update to queue failed")
    }
}

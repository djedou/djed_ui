
use djed::{
    djed_dom::{VNode, Listener},
    callback::Callback,
};
use std::rc::Rc;
use web_sys::{MouseEvent};

pub fn set_attribute<'a>(attr: Option<&String>, target: &mut VNode, attr_name: &'a str) {

    if let Some(data) = attr {
        if let VNode::VTag(tag) = target {
            tag.add_attribute(attr_name, data);
        }
    }

}


pub fn set_event<'a, T: 'a>(event: Option<&Callback<T>>, target: &mut VNode, listener: Rc<dyn Listener>) {

    if let Some(_data) = event {
        if let VNode::VTag(tag) = target {
            tag.listeners.push(listener);
        }
    }

}
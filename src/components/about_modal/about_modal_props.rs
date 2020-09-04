use djed::{
    macros::Properties,
    callback::Callback,
    djed::{
        listener::{ChangeData, InputData},
        Children
    }
};

use web_sys::{MouseEvent, Event, DragEvent,
    FocusEvent, KeyboardEvent, ProgressEvent,
};


#[derive(Clone, PartialEq, Properties, Debug)]
pub struct AboutModalBoxProps {
    /** content rendered inside the AboutModelBox.  */
    #[prop_or_default]
    pub children: Children,

    // Global Attributes
    #[prop_or_default]
    pub accesskey: Option<String>,
    #[prop_or_default]
    pub autocapitalize: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub contenteditable: Option<String>,
    #[prop_or_default]
    pub dir: Option<String>,
    #[prop_or_default]
    pub draggable: Option<String>,
    #[prop_or_default]
    pub hidden: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub inputmode: Option<String>,
    #[prop_or_default]
    pub is: Option<String>,
    #[prop_or_default]
    pub itemid: Option<String>,
    #[prop_or_default]
    pub itemprop: Option<String>,
    #[prop_or_default]
    pub itemref: Option<String>,
    #[prop_or_default]
    pub itemscope: Option<String>,
    #[prop_or_default]
    pub itemtype: Option<String>,
    #[prop_or_default]
    pub lang: Option<String>,
    #[prop_or_default]
    pub part: Option<String>,
    #[prop_or_default]
    pub slot: Option<String>,
    #[prop_or_default]
    pub spellcheck: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub tabindex: Option<String>,
    #[prop_or_default]
    pub title: Option<String>,



    // ARIA Attributes
    #[prop_or_default]
    pub aria_activedescendant: Option<String>,
    #[prop_or_default]
    pub aria_atomic: Option<String>,
    #[prop_or_default]
    pub aria_autocomplete: Option<String>,
    #[prop_or_default]
    pub aria_busy: Option<String>,
    #[prop_or_default]
    pub aria_checked: Option<String>,
    #[prop_or_default]
    pub aria_colcount: Option<String>,
    #[prop_or_default]
    pub aria_colindex: Option<String>,
    #[prop_or_default]
    pub aria_colspan: Option<String>,
    #[prop_or_default]
    pub aria_controls: Option<String>,
    #[prop_or_default]
    pub aria_current: Option<String>,
    #[prop_or_default]
    pub aria_describedby: Option<String>,
    #[prop_or_default]
    pub aria_details: Option<String>,
    #[prop_or_default]
    pub aria_disabled: Option<String>,
    #[prop_or_default]
    pub aria_dropeffect: Option<String>,
    #[prop_or_default]
    pub aria_errormessage: Option<String>,
    #[prop_or_default]
    pub aria_expanded: Option<String>,
    #[prop_or_default]
    pub aria_flowto: Option<String>,
    #[prop_or_default]
    pub aria_grabbed: Option<String>,
    #[prop_or_default]
    pub aria_haspopup: Option<String>,
    #[prop_or_default]
    pub aria_hidden: Option<String>,
    #[prop_or_default]
    pub aria_invalid: Option<String>,
    #[prop_or_default]
    pub aria_keyshortcuts: Option<String>,
    #[prop_or_default]
    pub aria_label: Option<String>,
    #[prop_or_default]
    pub aria_labelledby: Option<String>,
    #[prop_or_default]
    pub aria_level: Option<String>,
    #[prop_or_default]
    pub aria_live: Option<String>,
    #[prop_or_default]
    pub aria_modal: Option<String>,
    #[prop_or_default]
    pub aria_multiline: Option<String>,
    #[prop_or_default]
    pub aria_multiselectable: Option<String>,
    #[prop_or_default]
    pub aria_orientation: Option<String>,
    #[prop_or_default]
    pub aria_owns: Option<String>,
    #[prop_or_default]
    pub aria_placeholder: Option<String>,
    #[prop_or_default]
    pub aria_posinset: Option<String>,
    #[prop_or_default]
    pub aria_pressed: Option<String>,
    #[prop_or_default]
    pub aria_readonly: Option<String>,
    #[prop_or_default]
    pub aria_relevant: Option<String>,
    #[prop_or_default]
    pub aria_required: Option<String>,
    #[prop_or_default]
    pub aria_roledescription: Option<String>,
    #[prop_or_default]
    pub aria_rowcount: Option<String>,
    #[prop_or_default]
    pub aria_rowindex: Option<String>,
    #[prop_or_default]
    pub aria_rowspan: Option<String>,
    #[prop_or_default]
    pub aria_selected: Option<String>,
    #[prop_or_default]
    pub aria_setsize: Option<String>,
    #[prop_or_default]
    pub aria_sort: Option<String>,
    #[prop_or_default]
    pub aria_valuemax: Option<String>,
    #[prop_or_default]
    pub aria_valuemin: Option<String>,
    #[prop_or_default]
    pub aria_valuenow: Option<String>,
    #[prop_or_default]
    pub aria_valuetext: Option<String>,
    #[prop_or_default]
    pub role: Option<String>,


    // Document Event
    #[prop_or_default]
    pub onabort: Option<Callback<Event>>,
    #[prop_or_default]
    pub onerror: Option<Callback<Event>>,
    #[prop_or_default]
    pub onresize: Option<Callback<Event>>,
    #[prop_or_default]
    pub onscroll: Option<Callback<Event>>,
    #[prop_or_default]
    pub onunload: Option<Callback<Event>>,

    // Document Element Event 
    #[prop_or_default]
    pub oncopy: Option<Callback<Event>>,
    #[prop_or_default] 
    pub oncut: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onpaste: Option<Callback<Event>>,

    // Global Event
    #[prop_or_default]
    pub oncancel: Option<Callback<Event>>,
    #[prop_or_default] 
    pub oncanplay: Option<Callback<Event>>,
    #[prop_or_default] 
    pub oncanplaythrough: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onchange: Option<Callback<ChangeData>>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onclose: Option<Callback<Event>>,
    #[prop_or_default] 
    pub oncuechange: Option<Callback<Event>>,
    #[prop_or_default] 
    pub ondblclick: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub ondrag: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragend: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragenter: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragexit: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragleave: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragover: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondragstart: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondrop: Option<Callback<DragEvent>>,
    #[prop_or_default] 
    pub ondurationchange: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onemptied: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onended: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onfocus: Option<Callback<FocusEvent>>,
    #[prop_or_default] 
    pub oninput: Option<Callback<InputData>>,
    #[prop_or_default] 
    pub oninvalid: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onkeydown: Option<Callback<KeyboardEvent>>,
    #[prop_or_default] 
    pub onkeypress: Option<Callback<KeyboardEvent>>,
    #[prop_or_default] 
    pub onkeyup: Option<Callback<KeyboardEvent>>,
    #[prop_or_default] 
    pub onload: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onloadeddata: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onloadedmetadata: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onloadstart: Option<Callback<ProgressEvent>>,
    #[prop_or_default] 
    pub onmousedown: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmouseenter: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmouseleave: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmousemove: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmouseout: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmouseover: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmouseup: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onmousewheel: Option<Callback<MouseEvent>>,
    #[prop_or_default] 
    pub onpause: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onplay: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onplaying: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onprogress: Option<Callback<ProgressEvent>>,
    #[prop_or_default] 
    pub onratechange: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onreset: Option<Callback<Event>>,
    #[prop_or_default]   
    pub onseeked: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onseeking: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onselect: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onshow: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onstalled: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onsubmit: Option<Callback<FocusEvent>>,
    #[prop_or_default] 
    pub onsuspend: Option<Callback<Event>>,
    #[prop_or_default] 
    pub ontimeupdate: Option<Callback<Event>>,
    #[prop_or_default] 
    pub ontoggle: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onvolumechange: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onwaiting: Option<Callback<Event>>, 

    #[prop_or_default] 
    pub onautocomplete: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onautocompleteerror: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onblur: Option<Callback<FocusEvent>>,

    // Graphical Event 
    #[prop_or_default]
    pub onactivate: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onfocusin: Option<Callback<Event>>,
    #[prop_or_default] 
    pub onfocusout: Option<Callback<Event>>, 
}
  

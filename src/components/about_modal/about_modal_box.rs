use crate::components::about_modal::about_modal_props;
 
/*import * as React from 'react';
import { css } from '@patternfly/react-styles';
import styles from '@patternfly/react-styles/css/components/AboutModalBox/about-modal-box';


export const AboutModalBox: React.FunctionComponent<AboutModalBoxProps> = ({
  children,
  className = '',
  ...props
}: AboutModalBoxProps) => (
  <div role="dialog" aria-modal="true" className={css(styles.aboutModalBox, className)} {...props}>
    {children}
  </div>
);
AboutModalBox.displayName = 'AboutModalBox';*/


use djed::{
    html, 
    djed::{Component, ComponentLink, Html, ShouldRender},
    callback::Callback,
    djed::{
        listener::{*},
        Children,
    }
};

use std::rc::Rc;
pub use about_modal_props::AboutModalBoxProps;
use crate::utils::{set_attribute, set_event};
use web_sys::{MouseEvent, Event, DragEvent,
    FocusEvent, KeyboardEvent, ProgressEvent
};

pub struct AboutModalBox {
    pub children: Children,

    // Global Attributes
    pub accesskey: Option<String>,
    pub autocapitalize: Option<String>,
    pub class: Option<String>,
    pub contenteditable: Option<String>,
    pub dir: Option<String>,
    pub draggable: Option<String>,
    pub hidden: Option<String>,
    pub id: Option<String>,
    pub inputmode: Option<String>,
    pub is: Option<String>,
    pub itemid: Option<String>,
    pub itemprop: Option<String>,
    pub itemref: Option<String>,
    pub itemscope: Option<String>,
    pub itemtype: Option<String>,
    pub lang: Option<String>,
    pub part: Option<String>,
    pub slot: Option<String>,
    pub spellcheck: Option<String>,
    pub style: Option<String>,
    pub tabindex: Option<String>,
    pub title: Option<String>,



    // ARIA Attributes
    pub aria_activedescendant: Option<String>,
    pub aria_atomic: Option<String>,
    pub aria_autocomplete: Option<String>,
    pub aria_busy: Option<String>,
    pub aria_checked: Option<String>,
    pub aria_colcount: Option<String>,
    pub aria_colindex: Option<String>,
    pub aria_colspan: Option<String>,
    pub aria_controls: Option<String>,
    pub aria_current: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_details: Option<String>,
    pub aria_disabled: Option<String>,
    pub aria_dropeffect: Option<String>,
    pub aria_errormessage: Option<String>,
    pub aria_expanded: Option<String>,
    pub aria_flowto: Option<String>,
    pub aria_grabbed: Option<String>,
    pub aria_haspopup: Option<String>,
    pub aria_hidden: Option<String>,
    pub aria_invalid: Option<String>,
    pub aria_keyshortcuts: Option<String>,
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
    pub aria_level: Option<String>,
    pub aria_live: Option<String>,
    pub aria_modal: Option<String>,
    pub aria_multiline: Option<String>,
    pub aria_multiselectable: Option<String>,
    pub aria_orientation: Option<String>,
    pub aria_owns: Option<String>,
    pub aria_placeholder: Option<String>,
    pub aria_posinset: Option<String>,
    pub aria_pressed: Option<String>,
    pub aria_readonly: Option<String>,
    pub aria_relevant: Option<String>,
    pub aria_required: Option<String>,
    pub aria_roledescription: Option<String>,
    pub aria_rowcount: Option<String>,
    pub aria_rowindex: Option<String>,
    pub aria_rowspan: Option<String>,
    pub aria_selected: Option<String>,
    pub aria_setsize: Option<String>,
    pub aria_sort: Option<String>,
    pub aria_valuemax: Option<String>,
    pub aria_valuemin: Option<String>,
    pub aria_valuenow: Option<String>,
    pub aria_valuetext: Option<String>,
    pub role: Option<String>,


    // Document Event
    pub onabort: Option<Callback<Event>>,
    pub onerror: Option<Callback<Event>>,
    pub onresize: Option<Callback<Event>>,
    pub onscroll: Option<Callback<Event>>,
    pub onunload: Option<Callback<Event>>,

    // Document Element Event 
    pub oncopy: Option<Callback<Event>>, 
    pub oncut: Option<Callback<Event>>, 
    pub onpaste: Option<Callback<Event>>,

    // Global Event
    pub oncancel: Option<Callback<Event>>, 
    pub oncanplay: Option<Callback<Event>>, 
    pub oncanplaythrough: Option<Callback<Event>>, 
    pub onchange: Option<Callback<ChangeData>>,
    pub onclick: Option<Callback<MouseEvent>>, 
    pub onclose: Option<Callback<Event>>, 
    pub oncuechange: Option<Callback<Event>>, 
    pub ondblclick: Option<Callback<MouseEvent>>, 
    pub ondrag: Option<Callback<DragEvent>>, 
    pub ondragend: Option<Callback<DragEvent>>, 
    pub ondragenter: Option<Callback<DragEvent>>, 
    pub ondragexit: Option<Callback<DragEvent>>, 
    pub ondragleave: Option<Callback<DragEvent>>, 
    pub ondragover: Option<Callback<DragEvent>>, 
    pub ondragstart: Option<Callback<DragEvent>>, 
    pub ondrop: Option<Callback<DragEvent>>, 
    pub ondurationchange: Option<Callback<Event>>, 
    pub onemptied: Option<Callback<Event>>, 
    pub onended: Option<Callback<Event>>, 
    pub onfocus: Option<Callback<FocusEvent>>, 
    pub oninput: Option<Callback<InputData>>, 
    pub oninvalid: Option<Callback<Event>>, 
    pub onkeydown: Option<Callback<KeyboardEvent>>, 
    pub onkeypress: Option<Callback<KeyboardEvent>>, 
    pub onkeyup: Option<Callback<KeyboardEvent>>, 
    pub onload: Option<Callback<Event>>, 
    pub onloadeddata: Option<Callback<Event>>, 
    pub onloadedmetadata: Option<Callback<Event>>, 
    pub onloadstart: Option<Callback<ProgressEvent>>, 
    pub onmousedown: Option<Callback<MouseEvent>>, 
    pub onmouseenter: Option<Callback<MouseEvent>>, 
    pub onmouseleave: Option<Callback<MouseEvent>>, 
    pub onmousemove: Option<Callback<MouseEvent>>, 
    pub onmouseout: Option<Callback<MouseEvent>>, 
    pub onmouseover: Option<Callback<MouseEvent>>, 
    pub onmouseup: Option<Callback<MouseEvent>>, 
    pub onmousewheel: Option<Callback<MouseEvent>>, 
    pub onpause: Option<Callback<Event>>, 
    pub onplay: Option<Callback<Event>>, 
    pub onplaying: Option<Callback<Event>>, 
    pub onprogress: Option<Callback<ProgressEvent>>, 
    pub onratechange: Option<Callback<Event>>, 
    pub onreset: Option<Callback<Event>>,   
    pub onseeked: Option<Callback<Event>>, 
    pub onseeking: Option<Callback<Event>>, 
    pub onselect: Option<Callback<Event>>, 
    pub onshow: Option<Callback<Event>>, 
    pub onstalled: Option<Callback<Event>>, 
    pub onsubmit: Option<Callback<FocusEvent>>, 
    pub onsuspend: Option<Callback<Event>>, 
    pub ontimeupdate: Option<Callback<Event>>, 
    pub ontoggle: Option<Callback<Event>>, 
    pub onvolumechange: Option<Callback<Event>>, 
    pub onwaiting: Option<Callback<Event>>, 
 
    pub onautocomplete: Option<Callback<Event>>, 
    pub onautocompleteerror: Option<Callback<Event>>, 
    pub onblur: Option<Callback<FocusEvent>>,

    // Graphical Event 
    pub onactivate: Option<Callback<Event>>, 
    pub onfocusin: Option<Callback<Event>>, 
    pub onfocusout: Option<Callback<Event>>, 
}

pub enum AboutModalBoxState {

}


impl Component for AboutModalBox {
    type State = AboutModalBoxState;
    type Props = AboutModalBoxProps;

    fn create(props: Self::Props, _link: ComponentLink<Self>) -> Self {
        AboutModalBox {
            children: props.children,
            // Global Attributes
            accesskey: props.accesskey,
            autocapitalize: props.autocapitalize,
            class: props.class,
            contenteditable: props.contenteditable,
            dir: props.dir,
            draggable: props.draggable,
            hidden: props.hidden,
            id: props.id,
            inputmode: props.inputmode,
            is: props.is,
            itemid: props.itemid,
            itemprop: props.itemprop,
            itemref: props.itemref,
            itemscope: props.itemscope,
            itemtype: props.itemtype,
            lang: props.lang,
            part: props.part,
            slot: props.slot,
            spellcheck: props.spellcheck,
            style: props.style,
            tabindex: props.tabindex,
            title: props.title,

            // ARIA Attributes
            aria_activedescendant: props.aria_activedescendant,
            aria_atomic: props.aria_atomic,
            aria_autocomplete: props.aria_autocomplete,
            aria_busy: props.aria_busy,
            aria_checked: props.aria_checked,
            aria_colcount: props.aria_colcount,
            aria_colindex: props.aria_colindex,
            aria_colspan: props.aria_colspan,
            aria_controls: props.aria_controls,
            aria_current: props.aria_current,
            aria_describedby: props.aria_describedby,
            aria_details: props.aria_details,
            aria_disabled: props.aria_disabled,
            aria_dropeffect: props.aria_dropeffect,
            aria_errormessage: props.aria_errormessage,
            aria_expanded: props.aria_expanded,
            aria_flowto: props.aria_flowto,
            aria_grabbed: props.aria_grabbed,
            aria_haspopup: props.aria_haspopup,
            aria_hidden: props.aria_hidden,
            aria_invalid: props.aria_invalid,
            aria_keyshortcuts: props.aria_keyshortcuts,
            aria_label: props.aria_label,
            aria_labelledby: props.aria_labelledby,
            aria_level: props.aria_level,
            aria_live: props.aria_live,
            aria_modal: props.aria_modal,
            aria_multiline: props.aria_multiline,
            aria_multiselectable: props.aria_multiselectable,
            aria_orientation: props.aria_orientation,
            aria_owns: props.aria_owns,
            aria_placeholder: props.aria_placeholder,
            aria_posinset: props.aria_posinset,
            aria_pressed: props.aria_pressed,
            aria_readonly: props.aria_readonly,
            aria_relevant: props.aria_relevant,
            aria_required: props.aria_required,
            aria_roledescription: props.aria_roledescription,
            aria_rowcount: props.aria_rowcount,
            aria_rowindex: props.aria_rowindex,
            aria_rowspan: props.aria_rowspan,
            aria_selected: props.aria_selected,
            aria_setsize: props.aria_setsize,
            aria_sort: props.aria_sort,
            aria_valuemax: props.aria_valuemax,
            aria_valuemin: props.aria_valuemin,
            aria_valuenow: props.aria_valuenow,
            aria_valuetext: props.aria_valuetext,
            role: props.role,



            // Document Event
            onabort: props.onabort,
            onerror: props.onerror,
            onresize: props.onresize,
            onscroll: props.onscroll,
            onunload: props.onunload,

            // Document Element Event 
            oncopy: props.oncopy, 
            oncut: props.oncut, 
            onpaste: props.onpaste,

            // Global Event
            oncancel: props.oncancel, 
            oncanplay: props.oncanplay, 
            oncanplaythrough: props.oncanplaythrough, 
            onchange: props.onchange,
            onclick: props.onclick, 
            onclose: props.onclose, 
            oncuechange: props.oncuechange, 
            ondblclick: props.ondblclick, 
            ondrag: props.ondrag, 
            ondragend: props.ondragend, 
            ondragenter: props.ondragenter, 
            ondragexit: props.ondragexit, 
            ondragleave: props.ondragleave, 
            ondragover: props.ondragover, 
            ondragstart: props.ondragstart, 
            ondrop: props.ondrop, 
            ondurationchange: props.ondurationchange,
            onemptied: props.onemptied, 
            onended: props.onended, 
            onfocus: props.onfocus, 
            oninput: props.oninput, 
            oninvalid: props.oninvalid, 
            onkeydown: props.onkeydown, 
            onkeypress: props.onkeypress, 
            onkeyup: props.onkeyup, 
            onload: props.onload, 
            onloadeddata: props.onloadeddata, 
            onloadedmetadata: props.onloadedmetadata, 
            onloadstart: props.onloadstart, 
            onmousedown: props.onmousedown, 
            onmouseenter: props.onmouseenter, 
            onmouseleave: props.onmouseleave, 
            onmousemove: props.onmousemove, 
            onmouseout: props.onmouseout, 
            onmouseover: props.onmouseover, 
            onmouseup: props.onmouseup, 
            onmousewheel: props.onmousewheel, 
            onpause: props.onpause, 
            onplay: props.onplay, 
            onplaying: props.onplaying, 
            onprogress: props.onprogress, 
            onratechange: props.onratechange, 
            onreset: props.onreset,   
            onseeked: props.onseeked, 
            onseeking: props.onseeking, 
            onselect: props.onselect, 
            onshow: props.onshow, 
            onstalled: props.onstalled, 
            onsubmit: props.onsubmit, 
            onsuspend: props.onsuspend, 
            ontimeupdate: props.ontimeupdate, 
            ontoggle: props.ontoggle, 
            onvolumechange: props.onvolumechange, 
            onwaiting: props.onwaiting, 
        
            onautocomplete: props.onautocomplete, 
            onautocompleteerror: props.onautocompleteerror, 
            onblur: props.onblur,

            // Graphical Event 
            onactivate: props.onactivate, 
            onfocusin: props.onfocusin, 
            onfocusout: props.onfocusout, 
        }
    }

    fn update(&mut self, _state: Self::State) -> ShouldRender {
        
        false
    }

    fn change(&mut self, props: Self::Props) -> ShouldRender {
        self.children = props.children;
        // Global Attributes
        self.accesskey = props.accesskey;
        self.autocapitalize = props.autocapitalize;
        self.class = props.class;
        self.contenteditable = props.contenteditable;
        self.dir = props.dir;
        self.draggable = props.draggable;
        self.hidden = props.hidden;
        self.id = props.id;
        self.inputmode = props.inputmode;
        self.is = props.is;
        self.itemid = props.itemid;
        self.itemprop = props.itemprop;
        self.itemref = props.itemref;
        self.itemscope = props.itemscope;
        self.itemtype = props.itemtype;
        self.lang = props.lang;
        self.part = props.part;
        self.slot = props.slot;
        self.spellcheck = props.spellcheck;
        self.style = props.style;
        self.tabindex = props.tabindex;
        self.title = props.title;

        // ARIA Attributes
        self.aria_activedescendant = props.aria_activedescendant;
        self.aria_atomic = props.aria_atomic;
        self.aria_autocomplete = props.aria_autocomplete;
        self.aria_busy = props.aria_busy;
        self.aria_checked = props.aria_checked;
        self.aria_colcount = props.aria_colcount;
        self.aria_colindex = props.aria_colindex;
        self.aria_colspan = props.aria_colspan;
        self.aria_controls = props.aria_controls;
        self.aria_current = props.aria_current;
        self.aria_describedby = props.aria_describedby;
        self.aria_details = props.aria_details;
        self.aria_disabled = props.aria_disabled;
        self.aria_dropeffect = props.aria_dropeffect;
        self.aria_errormessage = props.aria_errormessage;
        self.aria_expanded = props.aria_expanded;
        self.aria_flowto = props.aria_flowto;
        self.aria_grabbed = props.aria_grabbed;
        self.aria_haspopup = props.aria_haspopup;
        self.aria_hidden = props.aria_hidden;
        self.aria_invalid = props.aria_invalid;
        self.aria_keyshortcuts = props.aria_keyshortcuts;
        self.aria_label = props.aria_label;
        self.aria_labelledby = props.aria_labelledby;
        self.aria_level = props.aria_level;
        self.aria_live = props.aria_live;
        self.aria_modal = props.aria_modal;
        self.aria_multiline = props.aria_multiline;
        self.aria_multiselectable = props.aria_multiselectable;
        self.aria_orientation = props.aria_orientation;
        self.aria_owns = props.aria_owns;
        self.aria_placeholder = props.aria_placeholder;
        self.aria_posinset = props.aria_posinset;
        self.aria_pressed = props.aria_pressed;
        self.aria_readonly = props.aria_readonly;
        self.aria_relevant = props.aria_relevant;
        self.aria_required = props.aria_required;
        self.aria_roledescription = props.aria_roledescription;
        self.aria_rowcount = props.aria_rowcount;
        self.aria_rowindex = props.aria_rowindex;
        self.aria_rowspan = props.aria_rowspan;
        self.aria_selected = props.aria_selected;
        self.aria_setsize = props.aria_setsize;
        self.aria_sort = props.aria_sort;
        self.aria_valuemax = props.aria_valuemax;
        self.aria_valuemin = props.aria_valuemin;
        self.aria_valuenow = props.aria_valuenow;
        self.aria_valuetext = props.aria_valuetext;
        self.role = props.role;



        // Document Event
        self.onabort = props.onabort;
        self.onerror = props.onerror;
        self.onresize = props.onresize;
        self.onscroll = props.onscroll;
        self.onunload = props.onunload;

        // Document Element Event 
        self.oncopy = props.oncopy; 
        self.oncut = props.oncut; 
        self.onpaste = props.onpaste;

        // Global Event
        self.oncancel = props.oncancel; 
        self.oncanplay = props.oncanplay; 
        self.oncanplaythrough = props.oncanplaythrough; 
        self.onchange = props.onchange;
        self.onclick = props.onclick; 
        self.onclose = props.onclose; 
        self.oncuechange = props.oncuechange; 
        self.ondblclick = props.ondblclick; 
        self.ondrag = props.ondrag; 
        self.ondragend = props.ondragend; 
        self.ondragenter = props.ondragenter; 
        self.ondragexit = props.ondragexit; 
        self.ondragleave = props.ondragleave; 
        self.ondragover = props.ondragover; 
        self.ondragstart = props.ondragstart; 
        self.ondrop = props.ondrop; 
        self.ondurationchange = props.ondurationchange;
        self.onemptied = props.onemptied; 
        self.onended = props.onended; 
        self.onfocus = props.onfocus; 
        self.oninput = props.oninput; 
        self.oninvalid = props.oninvalid; 
        self.onkeydown = props.onkeydown; 
        self.onkeypress = props.onkeypress; 
        self.onkeyup = props.onkeyup; 
        self.onload = props.onload; 
        self.onloadeddata = props.onloadeddata; 
        self.onloadedmetadata = props.onloadedmetadata; 
        self.onloadstart = props.onloadstart; 
        self.onmousedown = props.onmousedown; 
        self.onmouseenter = props.onmouseenter; 
        self.onmouseleave = props.onmouseleave; 
        self.onmousemove = props.onmousemove; 
        self.onmouseout = props.onmouseout; 
        self.onmouseover = props.onmouseover; 
        self.onmouseup = props.onmouseup; 
        self.onmousewheel = props.onmousewheel; 
        self.onpause = props.onpause; 
        self.onplay = props.onplay; 
        self.onplaying = props.onplaying; 
        self.onprogress = props.onprogress; 
        self.onratechange = props.onratechange; 
        self.onreset = props.onreset;   
        self.onseeked = props.onseeked; 
        self.onseeking = props.onseeking; 
        self.onselect = props.onselect; 
        self.onshow = props.onshow; 
        self.onstalled = props.onstalled; 
        self.onsubmit = props.onsubmit; 
        self.onsuspend = props.onsuspend; 
        self.ontimeupdate = props.ontimeupdate; 
        self.ontoggle = props.ontoggle; 
        self.onvolumechange = props.onvolumechange; 
        self.onwaiting = props.onwaiting; 
    
        self.onautocomplete = props.onautocomplete; 
        self.onautocompleteerror = props.onautocompleteerror; 
        self.onblur = props.onblur;

        // Graphical Event 
        self.onactivate = props.onactivate; 
        self.onfocusin = props.onfocusin; 
        self.onfocusout = props.onfocusout; 
        
        true
    }

    fn view(&self) -> Html {

        let mut div_tag = if self.children.is_empty() {
            html! {<div role="dialog" aria-modal="true"></div>}
        }
        else{
            html! {<div role="dialog" aria-modal="true">{ self.children.clone() }</div>}
        };

        set_attribute(self.accesskey.as_ref(), &mut div_tag, "accesskey");
        set_attribute(self.autocapitalize.as_ref(), &mut div_tag, "autocapitalize");
        set_attribute(self.contenteditable.as_ref(), &mut div_tag, "contenteditable");
        set_attribute(self.dir.as_ref(), &mut div_tag, "dir");

        set_attribute(self.id.as_ref(), &mut div_tag, "id");
        set_attribute(self.tabindex.as_ref(), &mut div_tag, "tabindex");

        set_attribute(self.style.as_ref(), &mut div_tag, "style");
        set_attribute(self.class.as_ref(), &mut div_tag, "class");

        set_attribute(self.draggable.as_ref(), &mut div_tag, "draggable");
        set_attribute(self.hidden.as_ref(), &mut div_tag, "hidden");

        set_attribute(self.inputmode.as_ref(), &mut div_tag, "inputmode");
        set_attribute(self.is.as_ref(), &mut div_tag, "is");
        set_attribute(self.itemid.as_ref(), &mut div_tag, "itemid");

        set_attribute(self.itemprop.as_ref(), &mut div_tag, "itemprop");
        set_attribute(self.itemref.as_ref(), &mut div_tag, "itemref");

        set_attribute(self.itemscope.as_ref(), &mut div_tag, "itemscope");
        set_attribute(self.itemtype.as_ref(), &mut div_tag, "itemtype");
        set_attribute(self.lang.as_ref(), &mut div_tag, "lang");


       
        set_attribute(self.part.as_ref(), &mut div_tag, "part");
        set_attribute(self.slot.as_ref(), &mut div_tag, "slot");
        set_attribute(self.spellcheck.as_ref(), &mut div_tag, "spellcheck");
        set_attribute(self.style.as_ref(), &mut div_tag, "style");
        set_attribute(self.tabindex.as_ref(), &mut div_tag, "tabindex");
        
        set_attribute(self.title.as_ref(), &mut div_tag, "title");

        set_attribute(self.aria_activedescendant.as_ref(), &mut div_tag, "aria-activedescendant");
        set_attribute(self.aria_atomic.as_ref(), &mut div_tag, "aria-atomic");
        set_attribute(self.aria_autocomplete.as_ref(), &mut div_tag, "aria-autocomplete");
        set_attribute(self.aria_busy.as_ref(), &mut div_tag,"aria-busy");
        set_attribute(self.aria_checked .as_ref(), &mut div_tag, "aria-checked");
        set_attribute(self.aria_colcount.as_ref(), &mut div_tag, "aria-colcount");
        set_attribute(self.aria_colindex.as_ref(), &mut div_tag, "aria-colindex");
        set_attribute(self.aria_colspan .as_ref(), &mut div_tag, "aria-colspan");
        set_attribute(self.aria_controls .as_ref(), &mut div_tag, "aria-controls");
        set_attribute(self.aria_current .as_ref(), &mut div_tag, "aria-current");
        set_attribute(self.aria_describedby.as_ref(), &mut div_tag, "aria_-escribedby");
        set_attribute(self.aria_details.as_ref(), &mut div_tag, "aria-details");
        set_attribute(self.aria_disabled.as_ref(), &mut div_tag, "aria-disabled");
        set_attribute(self.aria_dropeffect.as_ref(), &mut div_tag, "aria-dropeffect");
        set_attribute(self.aria_errormessage.as_ref(), &mut div_tag, "aria-errormessage");
        set_attribute(self.aria_expanded.as_ref(), &mut div_tag, "aria-expanded");
        set_attribute(self.aria_flowto.as_ref(), &mut div_tag, "aria-flowto");
        set_attribute(self.aria_grabbed.as_ref(), &mut div_tag, "aria-grabbed");
        set_attribute(self.aria_haspopup.as_ref(), &mut div_tag, "aria-haspopup");
        set_attribute(self.aria_hidden.as_ref(), &mut div_tag, "aria-hidden");
        set_attribute(self.aria_invalid.as_ref(), &mut div_tag, "aria-invalid");
        set_attribute(self.aria_keyshortcuts.as_ref(), &mut div_tag, "aria-keyshortcuts");
        set_attribute(self.aria_label.as_ref(), &mut div_tag, "aria-label");
        set_attribute(self.aria_labelledby.as_ref(), &mut div_tag, "aria-labelledby");
        set_attribute(self.aria_level.as_ref(), &mut div_tag, "aria-level");
        set_attribute(self.aria_live.as_ref(), &mut div_tag, "aria-live");
        set_attribute(self.aria_modal.as_ref(), &mut div_tag, "aria-modal");
        set_attribute(self.aria_multiline.as_ref(), &mut div_tag, "aria-multiline");
        set_attribute(self.aria_multiselectable.as_ref(), &mut div_tag, "aria-multiselectable");
        set_attribute(self.aria_orientation.as_ref(), &mut div_tag, "aria-orientation");
        set_attribute(self.aria_owns.as_ref(), &mut div_tag,"aria-owns");
        set_attribute(self.aria_placeholder.as_ref(), &mut div_tag, "aria-placeholder");
        set_attribute(self.aria_posinset.as_ref(), &mut div_tag, "aria-posinset");
        set_attribute(self.aria_pressed.as_ref(), &mut div_tag, "aria-pressed");
        set_attribute(self.aria_readonly.as_ref(), &mut div_tag, "aria-readonly");
        set_attribute(self.aria_relevant.as_ref(), &mut div_tag, "aria-relevant");
        set_attribute(self.aria_required.as_ref(), &mut div_tag, "aria-required");
        set_attribute(self.aria_roledescription.as_ref(), &mut div_tag, "aria-roledescription");
        set_attribute(self.aria_rowcount.as_ref(), &mut div_tag, "aria-rowcount");
        set_attribute(self.aria_rowindex.as_ref(), &mut div_tag, "aria-rowindex");
        set_attribute(self.aria_rowspan.as_ref(), &mut div_tag, "aria-rowspan");
        set_attribute(self.aria_selected.as_ref(), &mut div_tag, "aria-selected");
        set_attribute(self.aria_setsize.as_ref(), &mut div_tag, "aria-setsize");
        set_attribute(self.aria_sort.as_ref(), &mut div_tag, "aria-sort");
        set_attribute(self.aria_valuemax.as_ref(), &mut div_tag, "aria-valuemax");
        set_attribute(self.aria_valuemin.as_ref(), &mut div_tag, "aria-valuemin");
        set_attribute(self.aria_valuenow.as_ref(), &mut div_tag, "aria-valuenow");
        set_attribute(self.aria_valuetext.as_ref(), &mut div_tag, "aria-valuetext");
        set_attribute(self.role.as_ref(), &mut div_tag, "role");

        
        // Events

        // Document Event
        if let Some(event) = self.onabort.as_ref() {
            let onabort_listener = onabort::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onabort_listener));
        }
        if let Some(event) = self.onerror.as_ref() {
            let onerror_listener = onerror::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onerror_listener));
        }
        if let Some(event) = self.onresize.as_ref() {
            let onresize_listener = onresize::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onresize_listener));
        }
        if let Some(event) = self.onscroll.as_ref() {
            let onscroll_listener = onscroll::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onscroll_listener));
        }
        if let Some(event) = self.onunload.as_ref() {
            let onunload_listener = onunload::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onunload_listener));
        }

        // Document Element Event 
        if let Some(event) = self.oncopy.as_ref() {
            let oncopy_listener = oncopy::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(oncopy_listener));
        } 
        if let Some(event) = self.oncut.as_ref() {
            let oncut_listener = oncut::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(oncut_listener));
        } 
        if let Some(event) = self.onpaste.as_ref() {
            let onpaste_listener = onpaste::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onpaste_listener));
        }

        // Global Event
        if let Some(event) = self.oncancel.as_ref() {
            let oncancel_listener = oncancel::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(oncancel_listener));
        } 
        if let Some(event) = self.oncanplay.as_ref() {
            let oncanplay_listener = oncanplay::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(oncanplay_listener));
        } 
        if let Some(event) = self.oncanplaythrough.as_ref() {
            let oncanplaythrough_listener = oncanplaythrough::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(oncanplaythrough_listener));
        } 
        if let Some(event) = self.onchange.as_ref() {
            let onchange_listener = onchange::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onchange_listener));
        }
        if let Some(event) = self.onclick.as_ref() {
            let onclick_listener = onclick::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onclick_listener));
        }
        if let Some(event) = self.onclose.as_ref() {
            let onclose_listener = onclose::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onclose_listener));
        } 
        if let Some(event) = self.oncuechange.as_ref() {
            let oncuechange_listener = oncuechange::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(oncuechange_listener));
        } 
        if let Some(event) = self.ondblclick.as_ref() {
            let ondblclick_listener = ondblclick::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ondblclick_listener));
        } 
        if let Some(event) = self.ondrag.as_ref() {
            let ondrag_listener = ondrag::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ondrag_listener));
        } 
        if let Some(event) = self.ondragend.as_ref() {
            let ondragend_listener = ondragend::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ondragend_listener));
        } 
        if let Some(event) = self.ondragenter.as_ref() {
            let ondragenter_listener = ondragenter::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ondragenter_listener));
        } 
        if let Some(event) = self.ondragexit.as_ref() {
            let ondragexit_listener = ondragexit::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ondragexit_listener));
        } 
        if let Some(event) = self.ondragleave.as_ref() {
            let ondragleave_listener = ondragleave::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ondragleave_listener));
        } 
        if let Some(event) = self.ondragover.as_ref() {
            let ondragover_listener = ondragover::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ondragover_listener));
        } 
        if let Some(event) = self.ondragstart.as_ref() {
            let ondragstart_listener = ondragstart::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ondragstart_listener));
        } 
        if let Some(event) = self.ondrop.as_ref() {
            let ondrop_listener = ondrop::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ondrop_listener));
        } 
        if let Some(event) = self.ondurationchange.as_ref() {
            let ondurationchange_listener = ondurationchange::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ondurationchange_listener));
        } 
        if let Some(event) = self.onemptied.as_ref() {
            let onemptied_listener = onemptied::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onemptied_listener));
        } 
        if let Some(event) = self.onended.as_ref() {
            let onended_listener = onended::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onended_listener));
        } 
        if let Some(event) = self.onfocus.as_ref() {
            let onfocus_listener = onfocus::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onfocus_listener));
        } 
        if let Some(event) = self.oninput.as_ref() {
            let oninput_listener = oninput::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(oninput_listener));
        } 
        if let Some(event) = self.oninvalid.as_ref() {
            let oninvalid_listener = oninvalid::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(oninvalid_listener));
        } 
        if let Some(event) = self.onkeydown.as_ref() {
            let onkeydown_listener = onkeydown::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onkeydown_listener));
        } 
        if let Some(event) = self.onkeypress.as_ref() {
            let onkeypress_listener = onkeypress::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onkeypress_listener));
        } 
        if let Some(event) = self.onkeyup.as_ref() {
            let onkeyup_listener = onkeyup::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onkeyup_listener));
        } 
        if let Some(event) = self.onload.as_ref() {
            let onload_listener = onload::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onload_listener));
        } 
        if let Some(event) = self.onloadeddata.as_ref() {
            let onloadeddata_listener = onloadeddata::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onloadeddata_listener));
        } 
        if let Some(event) = self.onloadedmetadata.as_ref() {
            let onloadedmetadata_listener = onloadedmetadata::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onloadedmetadata_listener));
        } 
        if let Some(event) = self.onloadstart.as_ref() {
            let onloadstart_listener = onloadstart::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onloadstart_listener));
        } 
        if let Some(event) = self.onmousedown.as_ref() {
            let onmousedown_listener = onmousedown::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onmousedown_listener));
        } 
        if let Some(event) = self.onmouseenter.as_ref() {
            let onmouseenter_listener = onmouseenter::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onmouseenter_listener));
        } 
        if let Some(event) = self.onmouseleave.as_ref() {
            let onmouseleave_listener = onmouseleave::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onmouseleave_listener));
        } 
        if let Some(event) = self.onmousemove.as_ref() {
            let onmousemove_listener = onmousemove::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onmousemove_listener));
        } 
        if let Some(event) = self.onmouseout.as_ref() {
            let onmouseout_listener = onmouseout::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onmouseout_listener));
        } 
        if let Some(event) = self.onmouseover.as_ref() {
            let onmouseover_listener = onmouseover::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onmouseover_listener));
        } 
        if let Some(event) = self.onmouseup.as_ref() {
            let onmouseup_listener = onmouseup::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onmouseup_listener));
        } 
        if let Some(event) = self.onmousewheel.as_ref() {
            let onmousewheel_listener = onmousewheel::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onmousewheel_listener));
        } 
        if let Some(event) = self.onpause.as_ref() {
            let onpause_listener = onpause::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onpause_listener));
        } 
        if let Some(event) = self.onplay.as_ref() {
            let onplay_listener = onplay::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onplay_listener));
        } 
        if let Some(event) = self.onplaying.as_ref() {
            let onplaying_listener = onplaying::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onplaying_listener));
        } 
        if let Some(event) = self.onprogress.as_ref() {
            let onprogress_listener = onprogress::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onprogress_listener));
        } 
        if let Some(event) = self.onratechange.as_ref() {
            let onratechange_listener = onratechange::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onratechange_listener));
        } 
        if let Some(event) = self.onreset.as_ref() {
            let onreset_listener = onreset::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onreset_listener));
        }   
        if let Some(event) = self.onseeked.as_ref() {
            let onseeked_listener = onseeked::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onseeked_listener));
        } 
        if let Some(event) = self.onseeking.as_ref() {
            let onseeking_listener = onseeking::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onseeking_listener));
        } 
        if let Some(event) = self.onselect.as_ref() {
            let onselect_listener = onselect::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onselect_listener));
        } 
        if let Some(event) = self.onshow.as_ref() {
            let onshow_listener = onshow::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onshow_listener));
        } 
        if let Some(event) = self.onstalled.as_ref() {
            let onstalled_listener = onstalled::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onstalled_listener));
        } 
        if let Some(event) = self.onsubmit.as_ref() {
            let onsubmit_listener = onsubmit::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onsubmit_listener));
        } 
        if let Some(event) = self.onsuspend.as_ref() {
            let onsuspend_listener = onsuspend::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onsuspend_listener));
        } 
        if let Some(event) = self.ontimeupdate.as_ref() {
            let ontimeupdate_listener = ontimeupdate::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ontimeupdate_listener));
        } 
        if let Some(event) = self.ontoggle.as_ref() {
            let ontoggle_listener = ontoggle::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(ontoggle_listener));
        } 
        if let Some(event) = self.onvolumechange.as_ref() {
            let onvolumechange_listener = onvolumechange::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onvolumechange_listener));
        } 
        if let Some(event) = self.onwaiting.as_ref() {
            let onwaiting_listener = onwaiting::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onwaiting_listener));
        } 
        if let Some(event) = self.onautocomplete.as_ref() {
            let onautocomplete_listener = onautocomplete::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onautocomplete_listener));
        } 
        if let Some(event) = self.onautocompleteerror.as_ref() {
            let onautocompleteerror_listener = onautocompleteerror::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onautocompleteerror_listener));
        } 
        if let Some(event) = self.onblur.as_ref() {
            let onblur_listener = onblur::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onblur_listener));
        } 
        
        // Graphical Event 
        if let Some(event) = self.onactivate.as_ref() {
            let onactivate_listener = onactivate::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onactivate_listener));
        } 
        if let Some(event) = self.onfocusin.as_ref() {
            let onfocusin_listener = onfocusin::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onfocusin_listener));
        } 
        if let Some(event) = self.onfocusout.as_ref() {
            let onfocusout_listener = onfocusout::Wrapper::new(event.clone());
            set_event(Some(event),&mut div_tag, Rc::new(onfocusout_listener));
        } 

        div_tag

    }
}

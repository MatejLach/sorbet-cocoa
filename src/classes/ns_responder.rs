use std::ptr;
use objc;
use {into_bool, ShareId, WeakId, Object,
     AnyObject, NSObject, IsNSObject, NSEvent};

#[repr(C)]
pub struct NSResponder {
    super_: NSObject
}

unsafe impl objc::Message for NSResponder { }

impl Object for NSResponder {
    type Super = NSObject;

    fn super_ref(&self) -> &Self::Super {
        &self.super_
    }

    fn super_mut(&mut self) -> &mut Self::Super {
        &mut self.super_
    }
}

pub trait IsNSResponder: IsNSObject {
    fn accepts_first_responder(&self) -> bool;
    fn become_first_responder(&self) -> bool;
    fn resign_first_responder(&self) -> bool;
    fn validates_proposed_first_responder_for_event(&self, responder: ShareId<NSResponder>, event: Option<ShareId<NSEvent>>) -> bool;

    unsafe fn next_responder(&self) -> Option<WeakId<NSResponder>>;
    unsafe fn set_next_responder(&self, next_responder: Option<WeakId<NSResponder>>);

    fn mouse_down(&self, event: ShareId<NSEvent>);
    fn mouse_dragged(&self, event: ShareId<NSEvent>);
    fn mouse_up(&self, event: ShareId<NSEvent>);
    fn mouse_moved(&self, event: ShareId<NSEvent>);
    fn mouse_entered(&self, event: ShareId<NSEvent>);
    fn mouse_exited(&self, event: ShareId<NSEvent>);
    fn right_mouse_down(&self, event: ShareId<NSEvent>);
    fn right_mouse_dragged(&self, event: ShareId<NSEvent>);
    fn right_mouse_up(&self, event: ShareId<NSEvent>);
    fn other_mouse_down(&self, event: ShareId<NSEvent>);
    fn other_mouse_dragged(&self, event: ShareId<NSEvent>);
    fn other_mouse_up(&self, event: ShareId<NSEvent>);

    // fn key_down(&self, event: ShareId<NSEvent>);
    // fn key_up(&self, event: ShareId<NSEvent>);
    // fn interpret_key_events(&self, event: Vec<ShareId<NSEvent>>);
    // fn perform_key_equivalent(&self, event: ShareId<NSEvent>) -> bool;
    // fn flush_buffered_key_events(&self);
    //
    // fn pressure_change_with_event(&self, event: ShareId<NSEvent>);
    // fn cursor_update(&self, event: ShareId<NSEvent>);
    // fn flags_changed(&self, event: ShareId<NSEvent>);
    // fn tablet_point(&self, event: ShareId<NSEvent>);
    // fn tablet_proximity(&self, event: ShareId<NSEvent>);
    // fn help_requested(&self, event: ShareId<NSEvent>);
    // fn scroll_wheel(&self, event: ShareId<NSEvent>);
    // fn quick_look_with_event(&self, event: ShareId<NSEvent>);
    //
    // fn cancel_operation(&self, sender: Option<ShareId<NSObject>>);
    // fn capitalize_word(&self, sender: Option<ShareId<NSObject>>);
    // fn center_selection_in_visible_area(&self, sender: Option<ShareId<NSObject>>);
    // fn change_case_of_letter(&self, sender: Option<ShareId<NSObject>>);
    // fn complete(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_backward(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_backward_by_decomposing_previous_character(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_forward(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_to_beginning_of_line(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_to_beginning_of_paragraph(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_to_end_of_line(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_to_end_of_paragraph(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_to_mark(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_word_backward(&self, sender: Option<ShareId<NSObject>>);
    // fn delete_word_forward(&self, sender: Option<ShareId<NSObject>>);
}

pub trait SubNSResponder {
    type SuperNSResponder: IsNSResponder;

    fn super_ns_responder_ref(&self) -> &Self::SuperNSResponder;
    fn super_ns_responder_mut(&mut self) -> &mut Self::SuperNSResponder;
}

impl<T> SubNSResponder for T
    where T: Object, T::Super: IsNSResponder
{
    type SuperNSResponder = T::Super;

    fn super_ns_responder_ref(&self) -> &Self::SuperNSResponder {
        self.super_ref()
    }

    fn super_ns_responder_mut(&mut self) -> &mut Self::SuperNSResponder {
        self.super_mut()
    }
}

impl IsNSResponder for NSResponder {
    fn accepts_first_responder(&self) -> bool {
        unsafe {
            into_bool(msg_send![self, acceptsFirstResponder])
        }
    }

    fn become_first_responder(&self) -> bool {
        unsafe {
            into_bool(msg_send![self, becomeFirstResponder])
        }
    }

    fn resign_first_responder(&self) -> bool {
        unsafe {
            into_bool(msg_send![self, resignFirstResponder])
        }
    }

    fn validates_proposed_first_responder_for_event(&self, responder: ShareId<NSResponder>, event: Option<ShareId<NSEvent>>) -> bool
    {
        let event_ptr: *const NSEvent = match event {
            Some(event) => &*event,
            None => ptr::null()
        };
        unsafe {
            into_bool(msg_send![self, validatesProposedResponder:responder forEvent:event_ptr])
        }
    }



    unsafe fn next_responder(&self) -> Option<WeakId<NSResponder>> {
        let next_responder: *mut AnyObject = msg_send![self, nextResponder];
        let next_responder = next_responder as *mut NSResponder;
        if next_responder.is_null() {
            None
        }
        else {
            let share_id = ShareId::from_retained_ptr(next_responder);
            Some(WeakId::new(&share_id))
        }
    }

    unsafe fn set_next_responder(&self, next_responder: Option<WeakId<NSResponder>>) {
        let next_responder = next_responder.and_then(|weak| weak.load());
        let next_responder_ptr: *const NSResponder = match next_responder {
            Some(next_responder) => &*next_responder,
            None => ptr::null()
        };
        let next_responder_ptr = next_responder_ptr as *const AnyObject;
        msg_send![self, setNextResponder:next_responder_ptr];
    }



    fn mouse_down(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, mouseDown:&*event]; }
    }

    fn mouse_dragged(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, mouseDragged:&*event]; }
    }

    fn mouse_up(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, mouseUp:&*event]; }
    }

    fn mouse_moved(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, mouseMoved:&*event]; }
    }

    fn mouse_entered(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, mouseEntered:&*event]; }
    }

    fn mouse_exited(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, mouseExited:&*event]; }
    }

    fn right_mouse_down(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, rightMouseDown:&*event]; }
    }

    fn right_mouse_dragged(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, rightMouseDragged:&*event]; }
    }

    fn right_mouse_up(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, rightMouseUp:&*event]; }
    }

    fn other_mouse_down(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, otherMouseUp:&*event]; }
    }

    fn other_mouse_dragged(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, otherMouseDragged:&*event]; }
    }

    fn other_mouse_up(&self, event: ShareId<NSEvent>) {
        unsafe { msg_send![self, otherMouseUp:&*event]; }
    }
}

impl<T> IsNSResponder for T
    where T: SubNSResponder + IsNSObject
{
    fn accepts_first_responder(&self) -> bool {
        self.super_ns_responder_ref().accepts_first_responder()
    }

    fn become_first_responder(&self) -> bool {
        self.super_ns_responder_ref().become_first_responder()
    }

    fn resign_first_responder(&self) -> bool {
        self.super_ns_responder_ref().resign_first_responder()
    }

    fn validates_proposed_first_responder_for_event(&self, responder: ShareId<NSResponder>, event: Option<ShareId<NSEvent>>) -> bool {
        self.super_ns_responder_ref().validates_proposed_first_responder_for_event(responder, event)
    }



    unsafe fn next_responder(&self) -> Option<WeakId<NSResponder>> {
        self.super_ns_responder_ref().next_responder()
    }

    unsafe fn set_next_responder(&self, next_responder: Option<WeakId<NSResponder>>) {
        self.super_ns_responder_ref().set_next_responder(next_responder);
    }



    fn mouse_down(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().mouse_down(event);
    }

    fn mouse_dragged(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().mouse_dragged(event);
    }

    fn mouse_up(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().mouse_up(event);
    }

    fn mouse_moved(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().mouse_moved(event)
    }

    fn mouse_entered(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().mouse_entered(event);
    }

    fn mouse_exited(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().mouse_exited(event);
    }

    fn right_mouse_down(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().right_mouse_down(event);
    }

    fn right_mouse_dragged(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().right_mouse_dragged(event);
    }

    fn right_mouse_up(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().right_mouse_up(event);
    }

    fn other_mouse_down(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().other_mouse_down(event);
    }

    fn other_mouse_dragged(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().other_mouse_dragged(event);
    }

    fn other_mouse_up(&self, event: ShareId<NSEvent>) {
        self.super_ns_responder_ref().other_mouse_up(event);
    }
}

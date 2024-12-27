use std::any::Any;

// So that we can use const and static with Any
#[repr(C)]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct AnyObject {
    pub data: *mut (),
    pub vtable: *mut (),
}

impl AnyObject {
    /// "Set" value
    pub fn anonymise<T: 'static>(value: T) -> AnyObject {
        let any: Box<dyn Any> = Box::new(value) as Box<dyn Any>;
        let any: *mut dyn Any = Box::into_raw(any);
        unsafe { std::mem::transmute(any) }
    }

    /// "Get" Value
    /// SAFETY: Make sure that the object exists.
    pub fn deanonymise<T: 'static>(&self) -> Option<&T> {
        unsafe {
            let any: *const *const dyn Any = self as *const AnyObject as *const *const dyn Any;
            let any: &dyn Any = &*(*any as *const dyn Any);
            any.downcast_ref()
        }
    }
}

impl Drop for AnyObject {
    fn drop(&mut self) {
        unsafe {
            let any: *mut *mut dyn Any = self as *mut AnyObject as *mut *mut dyn Any;
            let any: *mut dyn Any = *any;
            let any: Box<dyn Any> = Box::from_raw(any);
            drop(any);
        }
    }
}

#[macro_export]
/// ## unwarp_option_rc
/// It will try to convert `Option<Rc<RefCell<T>>>` to `&T`
macro_rules! unwarp_option_rc {
    ($option:expr) => {
        match $option {
            Some(ref rc_refcell) => rc_refcell.borrow(),
            None => panic!("Called `unwarp_rc!` on a `None` value"),
        }
    };
}
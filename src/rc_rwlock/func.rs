use crate::*;

pub fn rc_rwlock<T>(data: T) -> RcRwLock<T> {
    Rc::new(RwLock::new(data))
}

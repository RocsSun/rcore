use core::cell::{RefCell, RefMut};

pub struct RcUpSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for RcUpSafeCell<T> {}

impl<T> RcUpSafeCell<T> {
    /// 用户必须保证这个内部结构体只在单核环境下使用，这里只是针对裸机编写操作系统，因此能保证单核运行，不存在多核运行情况的出现。
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    /// 使用权限检查，如果数据正在被借用，直接panic
    pub fn execute_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}

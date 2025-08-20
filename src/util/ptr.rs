use derive_where::derive_where;

/// Wrapper for a const raw pointer with unsafe Send.
#[derive_where(Debug, Clone, Copy)]
pub struct SendPtr<T>(*const T);

/// Wrapper for a mutable raw pointer with unsafe Send.
#[derive_where(Debug, Clone, Copy)]
pub struct SendPtrMut<T>(*mut T);

impl<T> SendPtr<T> {
    pub fn new(ptr: *const T) -> Self {
        Self(ptr)
    }
    pub fn null() -> Self {
        Self(std::ptr::null())
    }
    pub fn get(&self) -> *const T {
        self.0
    }
    pub fn cast<U>(&self) -> SendPtr<U> {
        SendPtr::new(self.get().cast::<U>())
    }
    pub fn cast_mut(&self) -> SendPtrMut<T> {
        SendPtrMut::new(self.get().cast_mut())
    }
    pub unsafe fn offset(&self, count: isize) -> Self {
        Self::new(unsafe { self.get().offset(count) })
    }
}
impl<T> SendPtrMut<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self(ptr)
    }
    pub fn null() -> Self {
        Self(std::ptr::null_mut())
    }
    pub fn get(&self) -> *mut T {
        self.0
    }
    pub fn cast<U>(&self) -> SendPtrMut<U> {
        SendPtrMut::new(self.get().cast::<U>())
    }
    pub fn cast_const(&self) -> SendPtr<T> {
        SendPtr::new(self.get())
    }
    pub unsafe fn offset(&self, count: isize) -> Self {
        Self::new(unsafe { self.get().offset(count) })
    }
}

unsafe impl<T> Send for SendPtr<T> {}
unsafe impl<T> Sync for SendPtr<T> {}
unsafe impl<T> Send for SendPtrMut<T> {}
unsafe impl<T> Sync for SendPtrMut<T> {}

impl<T> Default for SendPtr<T> {
    fn default() -> Self {
        Self::null()
    }
}
impl<T> Default for SendPtrMut<T> {
    fn default() -> Self {
        Self::null()
    }
}

/// Wrapper that owns a mutable raw pointer but cannot be dropped when it is non-null.
#[derive_where(Debug)]
pub struct UndropPtr<T>(*mut T);

unsafe impl<T> Send for UndropPtr<T> {}
unsafe impl<T> Sync for UndropPtr<T> {}

impl<T> UndropPtr<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self(ptr)
    }
    pub fn null() -> Self {
        Self(std::ptr::null_mut())
    }
    pub fn get(&self) -> *mut T {
        self.0
    }
    pub fn cast<U>(&self) -> UndropPtr<U> {
        UndropPtr::new(self.get().cast::<U>())
    }
    pub unsafe fn reset(&mut self) {
        self.0 = std::ptr::null_mut();
    }
}

impl<T> Default for UndropPtr<T> {
    fn default() -> Self {
        Self::null()
    }
}

impl<T> Drop for UndropPtr<T> {
    fn drop(&mut self) {
        if !self.get().is_null() {
            panic!("UndropPtr must be null");
        }
    }
}

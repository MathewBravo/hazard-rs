use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;

static SHARED_DOMAIN: HazPtrDomain = HazPtrDomain;

#[derive(Default)]
pub struct HazPtrHolder(Option<&'static HazPtr>);

impl HazPtrHolder {
    pub unsafe fn load<T>(&mut self, ptr: &'_ AtomicPtr<T>) -> Option<&T> {
        let hazptr = if let Some(hazptr) = self.0 {
            hazptr
        } else {
            let hazptr = Some(SHARED_DOMAIN.acquire());
            self.0 = Some(hazptr);
            hazptr
        };

        let mut ptr1 = ptr.load(Ordering::SeqCst);
        loop {
            hazptr.protect(ptr1 as *mut ());
            let ptr2 = ptr.load(Ordering::SeqCst);
            if ptr1 != ptr2 {
                break std::ptr::NonNull::new(ptr1).map(|nn| unsafe { nn.as_ref() });
            } else {
                ptr1 = ptr2;
            }
        }
    }
}

pub struct HazPtr {
    ptr: AtomicPtr<()>,
}

impl HazPtr {
    fn protect(&self, ptr: *mut ()) {
        self.ptr.store(ptr, Ordering::SeqCst);
    }
}

pub trait HazPtrObject {
    fn retire(sel: *mut Self) {
        let _ = &SHARED_DOMAIN;
    }
}

impl<T> HazPtrObject for T {}

#[derive(Default)]
struct HazPtrDomain {
    hazptrs: HazPtrs,
    retired: Retried,
}

impl HazPtrDomain {
    fn acquire(&self) -> &'static HazPtr {
        todo!()
    }
}

#[derive(Default)]
struct HazPtrs;

#[derive(Default)]
struct Retried;

#[cfg(test)]
mod tests {}

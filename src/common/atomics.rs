use std::{
    ffi::c_void,
    sync::atomic::{AtomicUsize, Ordering},
};

#[repr(C)]
pub struct AwsCAtomicVar {
    pub value: *mut c_void,
}

impl AwsCAtomicVar {
    pub fn from_int(n: usize) -> Self {
        Self {
            value: n as *mut c_void,
        }
    }
}

impl AwsCAtomicVar {
    // Interpret this atomic variable as an integer (usize).
    pub fn as_atomic_usize(&self) -> &AtomicUsize {
        let self_ptr = self as *const Self;
        let atomic_ptr = self_ptr as *const AtomicUsize;
        let atomic_ref = unsafe { atomic_ptr.as_ref().unwrap() };
        atomic_ref
    }

    // Interpret this atomic variable as a mutable integer (usize).
    pub fn as_mut_atomic_usize(&mut self) -> &mut AtomicUsize {
        let self_ptr = self as *mut Self;
        let atomic_ptr = self_ptr as *mut AtomicUsize;
        let atomic_ref = unsafe { atomic_ptr.as_mut().unwrap() };
        atomic_ref
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum AwsCMemoryOrder {
    Relaxed = 0,
    Acquire = 2,
    Release = 3,
    AcqRel = 4,
    SeqCst = 5,
}

impl From<AwsCMemoryOrder> for Ordering {
    fn from(order: AwsCMemoryOrder) -> Self {
        match order {
            AwsCMemoryOrder::Relaxed => Ordering::Relaxed,
            AwsCMemoryOrder::Acquire => Ordering::Acquire,
            AwsCMemoryOrder::Release => Ordering::Release,
            AwsCMemoryOrder::AcqRel => Ordering::AcqRel,
            AwsCMemoryOrder::SeqCst => Ordering::SeqCst,
        }
    }
}

/// Initializes an atomic variable with an integer value. This operation should be done before any
/// other operations on this atomic variable, and must be done before attempting any parallel operations.
///
/// This operation does not imply a barrier. Ensure that you use an acquire-release barrier (or stronger)
/// when communicating the fact that initialization is complete to the other thread. Launching the thread
/// implies a sufficiently strong barrier.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_init_int(var: *mut AwsCAtomicVar, n: usize) {
    let usize_ptr = var as *mut usize;
    *usize_ptr = n;
}

 /// Initializes an atomic variable with a pointer value. This operation should be done before any
/// other operations on this atomic variable, and must be done before attempting any parallel operations.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_init_ptr(var: *mut AwsCAtomicVar, p: *mut c_void) {
    let cvoid_ptr = var as *mut *mut c_void;
    *cvoid_ptr = p;
}


/// Reads an atomic var as an integer, using sequentially consistent ordering, and returns the result.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_load_int(var: *const AwsCAtomicVar) -> usize {
    aws_atomic_load_int_explicit(var, AwsCMemoryOrder::SeqCst)
}

/// Stores an integer into an atomic var, using sequentially consistent ordering.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_store_int(var: *mut AwsCAtomicVar, val: usize) {
    aws_atomic_store_int_explicit(var, val, AwsCMemoryOrder::SeqCst)
}

/// Exchanges an integer with the value in an atomic_var, using sequentially consistent ordering.
/// Returns the value that was previously in the atomic_var.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_exchange_int(var: *mut AwsCAtomicVar, n: usize) -> usize {
    aws_atomic_exchange_int_explicit(var, n, AwsCMemoryOrder::SeqCst)
}

/// Atomically compares `*var` to `*expected`; if they are equal, atomically sets `*var = desired`. Otherwise, `*expected` is set
/// to the value in `*var`. Uses sequentially consistent memory ordering, regardless of success or failure.
/// Returns `true` if the compare was successful and the variable updated to desired.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_compare_exchange_int(var: *mut AwsCAtomicVar, expected: *mut usize, desired: usize) -> bool {
    aws_atomic_compare_exchange_int_explicit(var, expected, desired, AwsCMemoryOrder::SeqCst, AwsCMemoryOrder::SeqCst)
}

/// Atomically adds `n` to `*var`, and returns the previous value of `*var`.
/// Uses sequentially consistent ordering.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_fetch_add(var: *mut AwsCAtomicVar, n: usize) -> usize {
    aws_atomic_fetch_add_explicit(var, n, AwsCMemoryOrder::SeqCst)
}

/// Atomically subtracts `n` from `*var`, and returns the previous value of `*var`.
/// Uses sequentially consistent ordering.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_fetch_sub(var: *mut AwsCAtomicVar, n: usize) -> usize {
    aws_atomic_fetch_sub_explicit(var, n, AwsCMemoryOrder::SeqCst)
}

/// Atomically ANDs `n` into `*var`, and returns the previous value of `*var`.
/// Uses sequentially consistent ordering.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_fetch_and(var: *mut AwsCAtomicVar, n: usize) -> usize {
    aws_atomic_fetch_and_explicit(var, n, AwsCMemoryOrder::SeqCst)
}

/// Atomically ORs `n` into *var, and returns the previous value of `*var`.
/// Uses sequentially consistent ordering.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_fetch_or(var: *mut AwsCAtomicVar, n: usize) -> usize {
    aws_atomic_fetch_or_explicit(var, n, AwsCMemoryOrder::SeqCst)
}

/// Atomically XORs `n` into `*var`, and returns the previous value of `*var`.
/// Uses sequentially consistent ordering.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_fetch_xor(var: *mut AwsCAtomicVar, n: usize) -> usize {
    aws_atomic_fetch_xor_explicit(var, n, AwsCMemoryOrder::SeqCst)
}

/// Reads an atomic var as an integer, using the specified ordering, and returns the result.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_load_int_explicit(var: *const AwsCAtomicVar, order: AwsCMemoryOrder) -> usize {
    var.as_ref().unwrap().as_atomic_usize().load(order.into())
}

/// Stores an integer into an atomic var, using the specified ordering.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_store_int_explicit(var: *mut AwsCAtomicVar, val: usize, order: AwsCMemoryOrder) {
    var.as_mut().unwrap().as_mut_atomic_usize().store(val, order.into())
}

/// Exchanges an integer with the value in an atomic_var, using the specified ordering.
/// Returns the value that was previously in the atomic_var.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_exchange_int_explicit(var: *mut AwsCAtomicVar, n: usize, memory_order: AwsCMemoryOrder) -> usize {
    var.as_mut().unwrap().as_mut_atomic_usize().swap(n, memory_order.into())
}

/// Atomically compares `*var` to `*expected`; if they are equal, atomically sets `*var = desired`. Otherwise, `*expected` is set
/// to the value in `*var`. On success, the memory ordering used was `order_success`; otherwise, it was `order_failure`.
/// `order_failure` must be no stronger than `order_success`, and must not be `release` or `acq_rel`.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_compare_exchange_int_explicit(
    var: *mut AwsCAtomicVar,
    expected: *mut usize,
    desired: usize,
    success: AwsCMemoryOrder,
    failure: AwsCMemoryOrder,
) -> bool {
    let result = var.as_mut()
        .unwrap()
        .as_mut_atomic_usize()
        .compare_exchange(*expected, desired, success.into(), failure.into());

    match result {
        Ok(_) => true,
        Err(actual) => {
            *expected = actual;
            false
        }
    }
}

/// Atomically adds `n` to `*var`, and returns the previous value of `*var`.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_fetch_add_explicit(var: *mut AwsCAtomicVar, n: usize, order: AwsCMemoryOrder) -> usize {
    var.as_mut().unwrap().as_mut_atomic_usize().fetch_add(n, order.into())
}

/// Atomically subtracts `n` from `*var`, and returns the previous value of `*var`.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_fetch_sub_explicit(var: *mut AwsCAtomicVar, n: usize, order: AwsCMemoryOrder) -> usize {
    var.as_mut().unwrap().as_mut_atomic_usize().fetch_sub(n, order.into())
}

/// Atomically ORs `n` with `*var`, and returns the previous value of `*var`.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_fetch_or_explicit(var: *mut AwsCAtomicVar, n: usize, order: AwsCMemoryOrder) -> usize {
    var.as_mut().unwrap().as_mut_atomic_usize().fetch_or(n, order.into())
}

/// Atomically ANDs `n` with `*var`, and returns the previous value of `*var`.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_fetch_and_explicit(var: *mut AwsCAtomicVar, n: usize, order: AwsCMemoryOrder) -> usize {
    var.as_mut().unwrap().as_mut_atomic_usize().fetch_and(n, order.into())
}

/// Atomically XORs `n` with `*var`, and returns the previous value of `*var`.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_fetch_xor_explicit(var: *mut AwsCAtomicVar, n: usize, order: AwsCMemoryOrder) -> usize {
    var.as_mut().unwrap().as_mut_atomic_usize().fetch_xor(n, order.into())
}

// Pointer operations

/// Reads an atomic var as a pointer, using sequentially consistent ordering, and returns the result.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_load_ptr(var: *const AwsCAtomicVar) -> *mut c_void {
    aws_atomic_load_ptr_explicit(var, AwsCMemoryOrder::SeqCst)
}

/// Stores a pointer into an atomic var, using sequentially consistent ordering.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_store_ptr(var: *mut AwsCAtomicVar, val: *mut c_void) {
    aws_atomic_store_ptr_explicit(var, val, AwsCMemoryOrder::SeqCst)
}

/// Exchanges an integer with the value in an atomic_var, using sequentially consistent ordering.
/// Returns the value that was previously in the atomic_var.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_exchange_ptr(var: *mut AwsCAtomicVar, n: *mut c_void) -> *mut c_void {
    aws_atomic_exchange_ptr_explicit(var, n, AwsCMemoryOrder::SeqCst)
}

/// Atomically compares `*var` to `*expected`; if they are equal, atomically sets `*var = desired`. Otherwise, `*expected` is set
/// to the value in `*var`. Uses sequentially consistent memory ordering, regardless of success or failure.
/// Returns `true` if the compare was successful and the variable updated to desired.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid.
pub unsafe fn aws_atomic_compare_exchange_ptr(
    var: *mut AwsCAtomicVar,
    expected: *mut *mut c_void,
    desired: *mut c_void,
) -> bool {
    aws_atomic_compare_exchange_ptr_explicit(var, expected, desired, AwsCMemoryOrder::SeqCst, AwsCMemoryOrder::SeqCst)
}

/// Reads an atomic var as a pointer, using the specified ordering, and returns the result.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_load_ptr_explicit(var: *const AwsCAtomicVar, order: AwsCMemoryOrder) -> *mut c_void {
    var.as_ref().unwrap().as_atomic_usize().load(order.into()) as *mut c_void
}

/// Stores an pointer into an atomic var, using the specified ordering.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_store_ptr_explicit(var: *mut AwsCAtomicVar, val: *mut c_void, order: AwsCMemoryOrder) {
    var.as_mut().unwrap().as_mut_atomic_usize().store(val as usize, order.into())
}

/// Exchanges a pointer with the value in an atomic_var, using the specified ordering.
/// Returns the value that was previously in the atomic_var.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_exchange_ptr_explicit(var: *mut AwsCAtomicVar, n: *mut c_void, memory_order: AwsCMemoryOrder) -> *mut c_void {
    var.as_mut().unwrap().as_mut_atomic_usize().swap(n as usize, memory_order.into()) as *mut c_void
}

/// Atomically compares `*var` to `*expected`; if they are equal, atomically sets `*var = desired`. Otherwise, `*expected` is set
/// to the value in `*var`. On success, the memory ordering used was `order_success`; otherwise, it was `order_failure`.
/// `order_failure` must be no stronger than `order_success`, and must not be `release` or `acq_rel`.
///
/// # Safety
/// The caller must ensure that the `AwsCAtomicVar` pointer is valid. 
pub unsafe fn aws_atomic_compare_exchange_ptr_explicit(
    var: *mut AwsCAtomicVar,
    expected: *mut *mut c_void,
    desired: *mut c_void,
    success: AwsCMemoryOrder,
    failure: AwsCMemoryOrder,
) -> bool {
    let result = var.as_mut()
        .unwrap()
        .as_mut_atomic_usize()
        .compare_exchange(*expected as usize, desired as usize, success.into(), failure.into());

    match result {
        Ok(_) => true,
        Err(actual) => {
            *expected = actual as *mut c_void;
            false
        }
    }
}

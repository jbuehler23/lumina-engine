use std::sync::atomic::{AtomicUsize, Ordering};
use std::alloc::{GlobalAlloc, Layout, System};

pub struct TrackedAllocator {
    inner: System,
    allocated: AtomicUsize,
    peak_allocated: AtomicUsize,
    allocation_count: AtomicUsize,
}

impl TrackedAllocator {
    pub const fn new() -> Self {
        Self {
            inner: System,
            allocated: AtomicUsize::new(0),
            peak_allocated: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
        }
    }

    pub fn allocated_bytes(&self) -> usize {
        self.allocated.load(Ordering::Relaxed)
    }

    pub fn peak_allocated_bytes(&self) -> usize {
        self.peak_allocated.load(Ordering::Relaxed)
    }

    pub fn allocation_count(&self) -> usize {
        self.allocation_count.load(Ordering::Relaxed)
    }

    pub fn reset_stats(&self) {
        self.peak_allocated.store(self.allocated.load(Ordering::Relaxed), Ordering::Relaxed);
        self.allocation_count.store(0, Ordering::Relaxed);
    }
}

unsafe impl GlobalAlloc for TrackedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = self.inner.alloc(layout);
        if !ptr.is_null() {
            let size = layout.size();
            let old_allocated = self.allocated.fetch_add(size, Ordering::Relaxed);
            let new_allocated = old_allocated + size;
            
            self.allocation_count.fetch_add(1, Ordering::Relaxed);
            
            let current_peak = self.peak_allocated.load(Ordering::Relaxed);
            if new_allocated > current_peak {
                self.peak_allocated.store(new_allocated, Ordering::Relaxed);
            }
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner.dealloc(ptr, layout);
        self.allocated.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

#[derive(Debug)]
pub struct MemoryStats {
    pub allocated_bytes: usize,
    pub peak_allocated_bytes: usize,
    pub allocation_count: usize,
}

pub fn memory_stats() -> MemoryStats {
    #[cfg(feature = "track-allocations")]
    {
        MemoryStats {
            allocated_bytes: TRACKED_ALLOCATOR.allocated_bytes(),
            peak_allocated_bytes: TRACKED_ALLOCATOR.peak_allocated_bytes(),
            allocation_count: TRACKED_ALLOCATOR.allocation_count(),
        }
    }
    #[cfg(not(feature = "track-allocations"))]
    {
        MemoryStats {
            allocated_bytes: 0,
            peak_allocated_bytes: 0,
            allocation_count: 0,
        }
    }
}

pub fn format_bytes(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

pub struct Pool<T> {
    items: Vec<T>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> Pool<T> {
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            items: Vec::new(),
            factory: Box::new(factory),
        }
    }

    pub fn with_capacity<F>(capacity: usize, factory: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            items: Vec::with_capacity(capacity),
            factory: Box::new(factory),
        }
    }

    pub fn acquire(&mut self) -> T {
        self.items.pop().unwrap_or_else(|| (self.factory)())
    }

    pub fn release(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(feature = "track-allocations")]
#[global_allocator]
static TRACKED_ALLOCATOR: TrackedAllocator = TrackedAllocator::new();
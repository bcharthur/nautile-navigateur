use crate::trace::{Trace, Tracer};

pub struct GcHandle(u32);

/// Heap mark-and-sweep simple pour la phase 1.
/// La nursery / GC générationnel vient en phase 6.
pub struct Heap {
    slots: Vec<Option<Box<dyn Trace>>>,
    marks: Vec<bool>,
    free: Vec<u32>,
}

impl Heap {
    pub fn new() -> Self {
        Self { slots: Vec::new(), marks: Vec::new(), free: Vec::new() }
    }

    pub fn alloc<T: Trace + 'static>(&mut self, obj: T) -> GcHandle {
        if let Some(id) = self.free.pop() {
            self.slots[id as usize] = Some(Box::new(obj));
            self.marks[id as usize] = false;
            GcHandle(id)
        } else {
            let id = self.slots.len() as u32;
            self.slots.push(Some(Box::new(obj)));
            self.marks.push(false);
            GcHandle(id)
        }
    }

    pub fn collect(&mut self, roots: &[u32]) {
        // Mark
        let mut tracer = MarkTracer { marks: &mut self.marks, worklist: roots.to_vec() };
        while let Some(id) = tracer.worklist.pop() {
            if tracer.marks[id as usize] { continue; }
            tracer.marks[id as usize] = true;
            if let Some(Some(obj)) = self.slots.get(id as usize) {
                // Safety: obj lives in self.slots, tracer borrows marks + worklist only
                let obj_ptr = obj.as_ref() as *const dyn Trace;
                unsafe { (*obj_ptr).trace(&mut tracer); }
            }
        }
        // Sweep
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_some() && !self.marks[i] {
                *slot = None;
                self.free.push(i as u32);
            }
            self.marks[i] = false;
        }
    }
}

impl Default for Heap { fn default() -> Self { Self::new() } }

struct MarkTracer<'a> {
    marks: &'a mut Vec<bool>,
    worklist: Vec<u32>,
}

impl<'a> Tracer for MarkTracer<'a> {
    fn mark(&mut self, id: u32) {
        if (id as usize) < self.marks.len() && !self.marks[id as usize] {
            self.worklist.push(id);
        }
    }
}

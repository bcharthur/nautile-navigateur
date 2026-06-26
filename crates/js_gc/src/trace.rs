/// Tout objet qui contient des références GC doit implémenter Trace.
pub trait Trace {
    fn trace(&self, tracer: &mut dyn Tracer);
}

pub trait Tracer {
    fn mark(&mut self, id: u32);
}

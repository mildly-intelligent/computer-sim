/// Contains objects for handling memory
pub mod memory;
/// Contains objects for the CPU and processing
pub mod cpu;
/// Code for processing and running machine code
pub mod proc;

pub mod prelude {
    pub use crate::memory;
}
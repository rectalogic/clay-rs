#[repr(C)]
pub struct String {
    length: isize,
    chars: *const u8,
}

#[repr(C)]
pub struct Arena {
    label: String,
    next_allocation: u64,
    capacity: u64,
    memory: *const u8,
}

#[link(name = "clay")]
extern "C" {
    fn Clay_CreateArenaWithCapacityAndMemory(capacity: u32, offset: *const u8) -> Arena;
}

// XXX make associated function
pub fn create_arena(memory: &[u8]) -> Arena {
    unsafe { Clay_CreateArenaWithCapacityAndMemory(memory.len() as u32, memory.as_ptr()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena() {
        let memory = [0u8; 1024];
        let arena = create_arena(&memory);
        assert_eq!(arena.capacity, memory.len() as u64);
    }
}

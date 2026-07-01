static STACK_SIZE: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ThreadStatus {
    ThreadRunning,
}

#[derive(Debug)]
pub struct Thread {
    pub id: usize,
    pub sp: usize,
    pub status: ThreadStatus,
}

#[derive(Debug)]
pub struct Cv {}

unsafe extern "C" {
    unsafe fn ctx_start(old_sp: *mut *mut u8, new_sp: *mut u8);
    unsafe fn ctx_switch(old_sp: *mut *mut u8, new_sp: *mut u8);
    unsafe fn _end();
}
#[unsafe(no_mangle)]
unsafe extern "C" fn ctx_entry() {
    unimplemented!()
}

fn thread_init() {
    unimplemented!()
}

fn thread_create(entry: extern "C" fn(*mut u8), arg: *mut u8) {
    unimplemented!()
}

fn thread_yield() {
    unimplemented!()
}

fn thread_exit() {
    unimplemented!()
}

fn cv_init(condition: &mut Cv) {
    unimplemented!()
}

fn cv_wait(condition: &mut Cv) {
    unimplemented!()
}

fn cv_signal(condition: &mut Cv) {
    unimplemented!()
}

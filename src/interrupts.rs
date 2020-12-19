use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let idt = InterruptDescriptorTable::new();
        // TODO: setup handler
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}


use crate::sbi::shutdown;
use core::panic::PanicInfo;
// use crate::console;


#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    // loop{}
    if let Some(location) = _info.location(){
        println!("panicked at{}:{}{}",
            location.file(),
            location.line(),
            _info.message().unwrap());
    }else{
        println!("panicked :{}",_info.message().unwrap());
    }
    shutdown()
}

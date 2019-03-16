#![no_std]



extern crate mbox;
use mbox::*;

//extern crate alloc;
//use alloc::boxed::Box;



#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod uorb {
    //TODO temp: load pre-built bindings
    include!("uorb.rs");

    extern {
        //TODO generate a list of static ORB_IDs from msg files

        // #define ORB_DECLARE(_name)		extern "C" const struct orb_metadata __orb_##_name __EXPORT
        // #define ORB_ID(_name)		&__orb_##_name
        // ORB_DECLARE(sensor_combined);
        // declare: const struct orb_metadata __orb_sensor_combined
        // orbid: &__orb_sensor_combined
        // pub type orb_id_t = *const orb_metadata;
        pub static __orb_sensor_combined: orb_metadata;
    }

    //TODO switch to this once we're generating bindings from .h files with bindgen
    //include!(concat!(env!("OUT_DIR"), "/uorb_bindings.rs")
}


#[no_mangle]
pub extern fn ephemeral_happy() -> u32 {
    let inst = Happy::new();
    inst.check_subscriptions()
}

#[no_mangle]
pub extern fn get_sub_metadata() -> uorb::orb_id_t {
    Happy::get_sub_address()
}

#[no_mangle]
pub extern fn new_happy_instance() -> *mut Happy  {
   MBox::into_raw(MBox::new(Happy::new()))
//    Box::into_raw(Box::new(Happy::new()))
}

#[no_mangle]
pub extern fn check_inst_subs(ptr: *mut Happy) -> u32 {
    let mut _inst = unsafe { &mut *ptr };
    _inst.check_subscriptions()
}

pub struct Happy {
    /// Subscription to uORB topic
    sensor_combined_sub: i32
}

impl Happy {
    fn new() -> Happy {
        let mut inst = Happy {
            sensor_combined_sub: -1
        };
        inst.setup_subscriptions();
        inst
    }

    pub  fn check_subscriptions(&self) -> u32 {
        unsafe {
            let mut checko: bool = false;
            uorb::orb_check(self.sensor_combined_sub,  &mut checko as *mut bool );
            return checko as u32;
        }
    }

    fn get_sub_address() -> uorb::orb_id_t {
        unsafe {
            &uorb::__orb_sensor_combined
        }
    }

    /// Subscribe to desired uorb topics
    fn setup_subscriptions(&mut self) {
        unsafe {
            self.sensor_combined_sub = uorb::orb_subscribe(&uorb::__orb_sensor_combined);
        }
    }
}

impl Drop for Happy {

    fn drop(&mut self) {
        unsafe {
            uorb::orb_unsubscribe(self.sensor_combined_sub);
        }
    }
}






use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

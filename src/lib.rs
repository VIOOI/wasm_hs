use wasm_bindgen::prelude::*;
use js_sys::{Object, Promise};
use web_sys::Window;
use std::{thread, time::Duration};

#[wasm_bindgen]
pub fn add( a: i32, b: i32 ) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn unic_vec( arr: Vec<i32> ) -> Vec<i32> {
    let mut item_arr = vec![];
    let mut del_item = vec![];
    let mut new_arr = arr.clone();
    arr.iter().enumerate().for_each(|(index, item)| {
        match item_arr.iter().find(|&x| x == &item ) {
            Some(_) => { del_item.push(index) },
            None => { item_arr.push(item) },
        }
    });
    del_item.reverse();
    del_item.iter().for_each(|x| { new_arr.remove(*x); });
    new_arr
}


#[wasm_bindgen]
pub fn concat_object( a: Object, b: Object ) -> Object {
    println!("{:?}", a);
    Object::assign(&a, &b)
}

#[wasm_bindgen]
pub fn sleep(time: i32) -> Promise {
    Promise::new(&mut | res, reg | {
        let window = web_sys::window().expect("should have a window in this context");
        Window
        ::set_timeout_with_callback_and_timeout_and_arguments_0(
            &window,
            &res,
            time
            ).unwrap();
    })
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

use userlib::*;

task_slot!(UART, usart_driver);

#[export_name = "main"]
fn main() -> ! {
    loop {
        uart_send(b"We're in!!!\r\n");
        hl::sleep_for(2000);
        uart_send(b"looping\r\n");
    }
}


fn uart_send(text: &[u8]) {
    let peer = UART.get_task_id();

    const OP_WRITE: u16 = 1;
    let (code, _) =
        sys_send(peer, OP_WRITE, &[], &mut [], &[Lease::from(text)]);
    assert_eq!(0, code);
}
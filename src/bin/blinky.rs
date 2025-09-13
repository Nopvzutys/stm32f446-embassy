#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let led = Output::new(p.PA5, Level::High, Speed::Low);
    spawner.spawn(my_task(led)).unwrap();

    let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);
    spawner.spawn(my_check_task(button)).unwrap();
}

#[embassy_executor::task]
async fn my_task(mut led: Output<'static>) {
    loop {
        info!("Task: LED high");
        led.set_high();
        Timer::after_millis(500).await;
        info!("Task: LED low");
        led.set_low();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn my_check_task(mut button: ExtiInput<'static>) {
    info!("Press the USER button...");

    loop {
        button.wait_for_rising_edge().await;
        info!("Released!");
        button.wait_for_falling_edge().await;
        info!("Pressed!");
    }
}

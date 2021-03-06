use core::time::Duration;

pub fn timer_now() -> Duration {
    // TODO: get actual tsc
    const TSC_FREQUENCY: u16 = 2600;
    let tsc = unsafe { core::arch::x86_64::_rdtsc() };
    Duration::from_nanos(tsc * 1000 / TSC_FREQUENCY as u64)
}

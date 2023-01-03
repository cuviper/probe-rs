use probe::probe;

fn main() {
    let mut iter = 0;
    loop {
        iter += 1;
        probe!(foo, iter, {
            // This delay is an exaggeration of the overhead of a probe argument, but it's only
            // incurred while something is attached to the probe, thanks to the semaphore.
            std::thread::sleep(std::time::Duration::from_secs(1));
            iter
        });
    }
}

// bcc/tools/trace.py -p $(pidof semaphore) 'u::foo:iter "iter = %d", arg1'

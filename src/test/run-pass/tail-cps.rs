


// -*- rust -*-
fn checktrue(rs: bool) -> bool { assert (rs); ret true; }

fn main() { let k = checktrue; evenk(42, k); oddk(45, k); }

fn evenk(n: int, k: fn(bool) -> bool) -> bool {
    log "evenk";
    log n;
    if n == 0 { be k(true); } else { be oddk(n - 1, k); }
}

fn oddk(n: int, k: fn(bool) -> bool) -> bool {
    log "oddk";
    log n;
    if n == 0 { be k(false); } else { be evenk(n - 1, k); }
}

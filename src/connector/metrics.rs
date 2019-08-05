use std::time::Instant;

pub(crate) fn query<F, T>(tag: &'static str, f: F) -> T
where
    F: FnOnce() -> T,
{
    counter!(format!("{}.query.count", tag), 1);
    time(format!("{}.query.time", tag), f)
}

pub(crate) fn connect<F, T>(tag: &'static str, f: F) -> T
where
    F: FnOnce() -> T,
{
    counter!(format!("{}.connect.count", tag), 1);
    time(format!("{}.connect.time", tag), f)
}

fn time<F, T>(tag: String, f: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let res = f();
    let end = Instant::now();

    timing!(tag, start, end);

    res
}
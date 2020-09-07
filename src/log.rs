//! # Logging macros

#[macro_export]
macro_rules! info {
    ($ctx:expr,  $msg:expr) => {
        info!($ctx, $msg,)
    };
    ($ctx:expr, $msg:expr, $($args:expr),* $(,)?) => {{
        let formatted = format!($msg, $($args),*);
        let thread = ::std::thread::current();
        // cs add standard width for all items
        let full = format!("{thid:12} {file:21}:{line:4}: {msg}",
                           thid = format!("{:12}", format!("{:?}", thread.id())),
                           file = file!(),
                           line = line!(),
                           msg = &formatted);
        emit_event!($ctx, $crate::Event::Info(full));
    }};
}

#[macro_export]
macro_rules! warn {
    ($ctx:expr, $msg:expr) => {
        warn!($ctx, $msg,)
    };
    ($ctx:expr, $msg:expr, $($args:expr),* $(,)?) => {{
        let formatted = format!($msg, $($args),*);
        let thread = ::std::thread::current();
        // cs add standard width for all items
        let full = format!("{thid:12} {file:21}:{line:4}: {msg}",
                           thid = format!("{:12}", format!("{:?}", thread.id())),
                           file = file!(),
                           line = line!(),
                           msg = &formatted);
        emit_event!($ctx, $crate::Event::Warning(full));
    }};
}

#[macro_export]
macro_rules! error {
    ($ctx:expr, $msg:expr) => {
        error!($ctx, $msg,)
    };
    ($ctx:expr, $msg:expr, $($args:expr),* $(,)?) => {{
        let formatted = format!($msg, $($args),*);
        emit_event!($ctx, $crate::Event::Error(formatted));
    }};
}

#[macro_export]
macro_rules! emit_event {
    ($ctx:expr, $event:expr) => {
        $ctx.call_cb($event);
    };
}

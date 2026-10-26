cfg_select! {
    feature = "MessagePort-post-ext" => {
        mod message_port;
        pub use message_port::*;
    }
    _ => {}
}

cfg_select! {
    feature = "Window-post-ext" => {
        mod window_port;
        pub use window_port::*;
    }
    _ => {}
}

cfg_select! {
    feature = "Worker-post-ext" => {
        mod worker_port;
        pub use worker_port::*;
    }
    _ => {}
}

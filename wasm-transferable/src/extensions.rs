cfg_select! {
    feature = "std-ext" => {
        mod std;
    }
    _ => {}
}

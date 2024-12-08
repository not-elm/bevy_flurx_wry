pub(crate) trait WryResultLog {
    fn output_log_if_failed(self);
}

impl<T> WryResultLog for wry::Result<T> {
    fn output_log_if_failed(self) {
        if let Err(e) = self {
            bevy_log::error!("{e}");
        }
    }
}

use anyhow::anyhow;

trait WrapAnyhow<T, E> {
    fn wrap_anyhow(self) -> anyhow::Result<T>;
}

impl<T, E: Into<anyhow::Error>> WrapAnyhow<T, E> for Result<T, E> {
    fn wrap_anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| anyhow!(e))
    }
}

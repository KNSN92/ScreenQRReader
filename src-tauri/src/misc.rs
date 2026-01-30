use anyhow::anyhow;

pub fn wrap_anyhow<T, E: Into<anyhow::Error>>(result: Result<T, E>) -> anyhow::Result<T> {
    result.map_err(|err| anyhow!(err))
} 
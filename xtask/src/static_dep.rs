use anyhow::Context;

pub struct StaticDep {
    url: String,
    not_run: std::path::PathBuf,
}

impl StaticDep {
    pub fn new(url: String, not_run: std::path::PathBuf) -> Self {
        Self { url, not_run }
    }

    /// Download and run unpack_cmd on download result download result is add at end of unpack_cmd
    pub fn run(&self, unpack_cmd: &mut std::process::Command) -> anyhow::Result<()> {
        if self.not_run.exists() {
            return Ok(());
        }

        let tmp_file = tempfile::NamedTempFile::new().context("Can't get temp file")?;

        let tmp_path = tmp_file.into_temp_path();
        let tmp_path_str = tmp_path.to_str().context("Failled to get temp path")?;

        match std::process::Command::new("curl")
            .args([&self.url, "-o", tmp_path_str])
            .spawn()
            .context("create curl command failled")?
            .wait()
            .context("curl command failled")?
            .code()
        {
            Some(0) => (),
            Some(a) => anyhow::bail!("curl command finish with exit code {}", a),
            None => anyhow::bail!("curl command finish without exit code"),
        }

        log::info!("prev {:?}", unpack_cmd);
        unpack_cmd.arg(tmp_path_str);

        log::info!("unpack_cmd {:?}", unpack_cmd);

        match unpack_cmd
            .spawn()
            .context("create unpack command failled")?
            .wait()
            .context("unpack command run failled")?
            .code()
        {
            Some(0) => (),
            Some(a) => anyhow::bail!("unpack command finish with exit code {}", a),
            None => anyhow::bail!("unpack command finish without exit code"),
        }

        Ok(())
    }
}

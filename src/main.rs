mod cli;

use fs::File;
use fs_err as fs;

use std::{ops::Not, path::Path, process::Command};

use anyhow::{bail, ensure, Context, Ok};
use cli::{New, SubcommandEnum, TopLevel};

type Result<T = ()> = anyhow::Result<T>;

struct PgTemp {
    config_dir: String,
}

impl PgTemp {
    pub fn init() -> Result<Self> {
        let home_dir = std::env::var("HOME").with_context(|| "Missing HOME variable")?;
        let config_dir = format!("{home_dir}/.pgtemp");

        if exists(&config_dir).not() {
            fs::create_dir(&config_dir)?;
        }

        Ok(Self { config_dir })
    }

    fn read_port(&self) -> Result<u32> {
        let port_path = format!("{}/port", self.config_dir);

        fs::read_to_string(port_path)?
            .trim_end()
            .parse()
            .map_err(Into::into)
    }

    fn write_port(&self, port: u32) -> Result<()> {
        use std::io::Write;

        let port_path = format!("{}/port", self.config_dir);
        let mut file = File::create(port_path)?;
        write!(file, "{port}")?;

        Ok(())
    }

    fn clean_up(&self) -> Result<()> {
        if !exists(&self.config_dir) {
            return Ok(());
        }

        fs::remove_dir_all(&self.config_dir)
            .with_context(|| format!("Failed to remove {}", self.config_dir))
    }

    fn create_folders<P: AsRef<Path>>(&self, path: &P) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            if parent.exists().not() {
                fs::create_dir(parent)?;
            }
        }

        Ok(())
    }

    pub fn new_db(&self, port: u32) -> Result<()> {
        let version = get_postgres_version()?;
        println!("Detected PostgreSQL {version}.");
        let db_path = format!("{}/db", self.config_dir);

        ensure!(
            exists(&db_path).not(),
            "Cannot create a new DB, one still exists."
        );

        self.write_port(port)?;

        self.create_folders(&db_path)?;

        let setup = || {
            run("initdb", &["-D", &db_path])?;
            println!("initdb successful.");
            run(
                "pg_ctl",
                &["-D", &db_path, "-o", &format!("-p {port} -k /tmp"), "start"],
            )?;
            println!("pg_ctl successful.");

            Ok(()) as Result<()>
        };

        setup().map_err(|err| {
            let _ = self.clean_up();
            err
        })?;

        println!("New instance is up!");

        Ok(())
    }

    fn delete(&self) -> Result<()> {
        let version = get_postgres_version()?;
        let db_path = format!("{}/db", self.config_dir);
        run(
            &format!("pg_ctl"),
            &["-D", &db_path, "stop"],
        )?;

        self.clean_up()?;

        Ok(())
    }

    pub fn conn_string(&self) -> Result<String> {
        self.read_port()
            .map(|port| format!("postgresql://localhost:{port}/postgres"))
    }

    fn connect(&self) -> Result<()> {
        let conn_string = self.conn_string()?;

        run("psql", &[&conn_string])
    }
}

fn run(program: &str, args: &[&str]) -> Result<()> {
    let child = Command::new(program).args(args).spawn()?;
    let output = child.wait_with_output()?;

    match output.status.success() {
        true => {
            let output = String::from_utf8(output.stdout)?;
            println!("{output}");
        }
        false => {
            let output = String::from_utf8(output.stderr)?;
            bail!("Failed to run {}: '{}'", program, output);
        }
    }

    Ok(())
}

fn get_postgres_version() -> Result<u8> {
    let output = Command::new("pg_config").arg("--version").output()?;
    ensure!(output.status.success(), "failed to run pg_config");

    let stdout = String::from_utf8(output.stdout)?;

    let version = if stdout.starts_with("PostgreSQL 14") {
        14
    } else if stdout.starts_with("PostgreSQL 15") {
        15
    } else if stdout.starts_with("PostgreSQL 16") {
        16
    } else {
        bail!("Unsupported Postgres version")
    };

    Ok(version)
}

fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

fn main() -> Result<()> {
    let pgtemp = PgTemp::init()?;

    let args: TopLevel = argh::from_env();

    match args.subcommand {
        SubcommandEnum::New(New { port }) => pgtemp.new_db(port),
        SubcommandEnum::Delete(_) => pgtemp.delete(),
        SubcommandEnum::Connect(_) => pgtemp.connect(),
        SubcommandEnum::Connstring(_) => {
            let conn_string = pgtemp.conn_string()?;
            println!("{conn_string}");
            Ok(())
        }
    }
}

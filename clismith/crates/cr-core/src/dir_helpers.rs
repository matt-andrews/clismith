use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    sync::{LazyLock, RwLock},
};

use crate::helpers::env_or;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DirType {
    Project,
    Target,
    ExecAppData,
    Exec,
}

static DIR_STORE: LazyLock<RwLock<HashMap<DirType, PathBuf>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn set_dir(t: DirType, path: &Path) {
    fs::create_dir_all(path).unwrap_or_default(); //ensure dir is created
    DIR_STORE.write().unwrap().insert(t, path.to_path_buf());
}

pub fn get_dir(t: DirType) -> PathBuf {
    if let Some(p) = DIR_STORE.read().unwrap().get(&t) {
        return p.clone();
    }

    let def = get_dir_as_env(t);
    fs::create_dir_all(def.clone()).unwrap_or_default(); //ensure dir is created
    DIR_STORE.write().unwrap().entry(t).or_insert(def).clone()
}

pub fn get_dir_with(t: DirType, str: &str) -> PathBuf {
    let mut dir = get_dir(t);
    dir.push(str);
    dir
}

pub fn get_project_name() -> String {
    env_or("PROJECT_NAME", "system")
}

pub fn project_is_system() -> bool {
    get_project_name() == "system"
}

fn get_dir_as_env(t: DirType) -> PathBuf {
    match t {
        DirType::ExecAppData => get_exec_loc(),
        DirType::Exec => {
            let mut top = get_dir(DirType::ExecAppData);
            top.push(get_project_name());
            top
        }
        DirType::Target => get_env_as_path("BUILD_TARGET", PathBuf::from("./target")),
        DirType::Project => get_env_as_path("BUILD_PROJECT", PathBuf::from("./")),
    }
}

fn get_env_as_path(key: &str, default: PathBuf) -> PathBuf {
    if let Ok(env) = env::var(key) {
        return PathBuf::from(env);
    }
    default
}

fn get_exec_loc() -> PathBuf {
    let mut result = get_env_as_path(
        "PROJECT_LOCATION",
        match dirs::config_dir() {
            Some(config_dir) => config_dir,
            None => PathBuf::from("./"),
        },
    );

    result.push("clismith");
    result
}

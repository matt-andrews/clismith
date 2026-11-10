use std::collections::BTreeMap;

use cr_core::v1spec::{
    core::CoreSpec,
    index::{Index, IndexModule},
    module::{CommandSpec, ModuleSpec},
    param::ParamSpec,
};

use crate::runtime::sys::{
    SYS_CMD_COMPILE_APP, SYS_CMD_COMPILE_VERSION, SYS_CMD_INSTALL_APP, SYS_CMD_PROJECT_INIT,
};

pub fn get_sys_manifest() -> Index {
    //
    Index {
        name: "system".to_string(),
        core: "system".to_string(),
        modules: vec![
            IndexModule {
                name: "compiler".to_string(),
                version: "1.0.0".to_string(),
                ..Default::default()
            },
            IndexModule {
                name: "app-manager".to_string(),
                version: "1.0.0".to_string(),
                ..Default::default()
            },
        ],
    }
}

pub fn get_sys_module(module: &str) -> Option<ModuleSpec> {
    //
    let map: BTreeMap<String, ModuleSpec> = BTreeMap::from([
        (
            "compiler".to_string(),
            ModuleSpec {
                name: "compiler".to_string(),
                about: Some("Tools to compile apps".to_string()),
                commands: vec![
                    CommandSpec {
                        target: Some(SYS_CMD_COMPILE_APP.to_string()),
                        about: Some("Compiles the app at the given location".to_string()),
                        bindings: BTreeMap::from([
                            ("verb".to_string(), "build".to_string()),
                            ("module".to_string(), "compiler".to_string()),
                        ]),
                        flags: vec![
                            ParamSpec {
                                target: Some("src".to_string()),
                                long: "src".to_string(),
                                about: Some("The project source".to_string()),
                                ..Default::default()
                            },
                            ParamSpec {
                                target: Some("target".to_string()),
                                long: "target".to_string(),
                                about: Some("The project target".to_string()),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                    CommandSpec {
                        target: Some(SYS_CMD_PROJECT_INIT.to_string()),
                        about: Some(
                            "Initialize a new project using opinionated defaults".to_string(),
                        ),
                        bindings: BTreeMap::from([
                            ("verb".to_string(), "init".to_string()),
                            ("resource".to_string(), "project".to_string()),
                            ("module".to_string(), "compiler".to_string()),
                        ]),
                        args: vec![ParamSpec {
                            target: Some("name".to_string()),
                            long: "name".to_string(),
                            about: Some("The name of the project".to_string()),
                            required: true,
                            ..Default::default()
                        }],
                        flags: vec![
                            ParamSpec {
                                target: Some("about".to_string()),
                                long: "about".to_string(),
                                about: Some("Pre-set the about section of the project".to_string()),
                                ..Default::default()
                            },
                            ParamSpec {
                                target: Some("src".to_string()),
                                long: "src".to_string(),
                                about: Some("The project location".to_string()),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                    CommandSpec {
                        target: Some(SYS_CMD_COMPILE_VERSION.to_string()),
                        about: Some("Gets the CLIsmith executable version".to_string()),
                        bindings: BTreeMap::from([
                            ("verb".to_string(), "version".to_string()),
                            ("module".to_string(), "compiler".to_string()),
                        ]),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ),
        (
            "app-manager".to_string(),
            ModuleSpec {
                name: "app-manager".to_string(),
                about: Some("Tools to manage apps".to_string()),
                commands: vec![CommandSpec {
                    target: Some(SYS_CMD_INSTALL_APP.to_string()),
                    about: Some("Installs a specified app".to_string()),
                    bindings: BTreeMap::from([
                        ("verb".to_string(), "install".to_string()),
                        ("module".to_string(), "app-manager".to_string()),
                    ]),
                    args: vec![
                        ParamSpec {
                            target: Some("name".to_string()),
                            long: "name".to_string(),
                            about: Some("The name to save this app as".to_string()),
                            required: true,
                            ..Default::default()
                        },
                        ParamSpec {
                            target: Some("index".to_string()),
                            long: "index".to_string(),
                            about: Some(
                                "The remote index location for the app to be downloaded"
                                    .to_string(),
                            ),
                            required: true,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                ..Default::default()
            },
        ),
    ]);

    map.iter().find(|f| f.0 == module).map(|v| v.1.to_owned())
}

pub fn get_sys_core() -> CoreSpec {
    //
    CoreSpec::init_default_template("system", Some("Baked-in system commands".to_string()))
}

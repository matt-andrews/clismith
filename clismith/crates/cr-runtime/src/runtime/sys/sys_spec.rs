use std::collections::BTreeMap;

use cr_core::v1spec::{
    core::{CoreSpec, Grammar, Naming},
    index::{Index, IndexModule},
    module::{CommandSpec, ModuleSpec},
    param::ParamSpec,
    value_kind::ValueKind,
};

use crate::runtime::sys::{SYS_CMD_COMPILE_APP, SYS_CMD_COMPILE_VERSION, SYS_CMD_INSTALL_APP};

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
    CoreSpec {
        name: "system".to_string(),
        about: Some("Baked-in system commands".to_string()),
        grammar: Grammar {
            naming: Naming {
                case: "kebab".to_string(),
            },
            pattern: "{ module } { resource? } { verb }".to_string(),
            verbs: Vec::new(),
        },
        global_flags: vec![
            ParamSpec {
                long: "verbose".to_string(),
                short: Some('v'),
                about: Some("Enable verbose logging".to_string()),
                kind: ValueKind::Bool,
                target: Some("SYS_FLAG_VERBOSE".to_string()),
                ..Default::default()
            },
            ParamSpec {
                long: "head".to_string(),
                short: Some('I'),
                about: Some(
                    "Force the output to be request headers instead of content".to_string(),
                ),
                target: Some("SYS_FLAG_HEAD".to_string()),
                ..Default::default()
            },
            ParamSpec {
                long: "output".to_string(),
                short: Some('o'),
                about: Some("Set the printed structure".to_string()),
                target: Some("SYS_FLAG_OUTPUT".to_string()),
                default: Some("json".to_string()),
                choices: Some(vec![
                    "json".to_string(),
                    "table".to_string(),
                    "none".to_string(),
                ]),
                ..Default::default()
            },
            ParamSpec {
                long: "output-prop".to_string(),
                about: Some("Select which properties are output".to_string()),
                target: Some("SYS_FLAG_OUTPUT_PROP".to_string()),
                multiple: true,
                ..Default::default()
            },
            ParamSpec {
                long: "output-sort".to_string(),
                about: Some(
                    "Sort output by given property. Prefix the property with `-` for descending"
                        .to_string(),
                ),
                target: Some("SYS_FLAG_OUTPUT_SORT".to_string()),
                multiple: true,
                allow_hyphen_values: Some(true),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}

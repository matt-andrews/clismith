pub mod module_installer;
pub mod sys_dispatcher;
pub mod sys_spec;

//flags
pub const SYS_FLAG_VERBOSE: &str = "SYS_FLAG_VERBOSE";
pub const SYS_FLAG_OUTPUT: &str = "SYS_FLAG_OUTPUT";
pub const SYS_FLAG_HEAD: &str = "SYS_FLAG_HEAD";
pub const SYS_FLAG_OUTPUT_PROP: &str = "SYS_FLAG_OUTPUT_PROP";
pub const SYS_FLAG_OUTPUT_SORT: &str = "SYS_FLAG_OUTPUT_SORT";

//commands
pub const SYS_CMD_MANIFEST_LIST: &str = "SYS_CMD_MANIFEST_LIST";
pub const SYS_CMD_INSTALL_APP: &str = "SYS_CMD_INSTALL_APP";
pub const SYS_CMD_COMPILE_APP: &str = "SYS_CMD_COMPILE_APP";
pub const SYS_CMD_COMPILE_VERSION: &str = "SYS_CMD_COMPILE_VERSION";
pub const SYS_CMD_PROJECT_INIT: &str = "SYS_CMD_PROJECT_INIT";

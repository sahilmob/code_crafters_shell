use std::{collections::HashSet, sync::LazyLock};

use crate::{cd, cmd_types, echo, exec_bin, exit, pwd, r#type};

pub static CMD_TYPES: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    cmd_types!(
        exit::TYPE,
        echo::TYPE,
        r#type::TYPE,
        exec_bin::TYPE,
        pwd::TYPE,
        cd::TYPE
    )
});

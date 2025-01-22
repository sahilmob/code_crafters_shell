use std::{collections::HashSet, sync::LazyLock};

use crate::{cmd_types, echo, exit, r#type};

pub static CMD_TYPES: LazyLock<HashSet<&str>> =
    std::sync::LazyLock::new(|| cmd_types!(exit::TYPE, echo::TYPE, r#type::TYPE));

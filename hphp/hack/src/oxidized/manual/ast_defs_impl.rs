// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use crate::ast_defs::*;
use crate::pos::Pos;

impl ShapeFieldName {
    pub fn get_name(&self) -> &str {
        use ShapeFieldName::*;
        match self {
            SFlitInt((_, name)) | SFlitStr((_, name)) | SFclassConst(_, (_, name)) => name.as_str(),
        }
    }

    pub fn get_pos(&self) -> &Pos {
        use ShapeFieldName::*;
        match self {
            SFlitInt((p, _)) | SFlitStr((p, _)) | SFclassConst(_, (p, _)) => &p,
        }
    }
}

impl FunKind {
    pub fn is_async(self) -> bool {
        self == FunKind::FAsync || self == FunKind::FAsyncGenerator
    }
}

impl Id {
    pub fn name(&self) -> &str {
        &self.1
    }
}

impl ClassKind {
    pub fn is_cabstract(&self) -> bool {
        match self {
            ClassKind::Cabstract => true,
            _ => false,
        }
    }

    pub fn is_cinterface(&self) -> bool {
        match self {
            ClassKind::Cinterface => true,
            _ => false,
        }
    }
}

impl FunKind {
    pub fn is_fasync(&self) -> bool {
        match self {
            FunKind::FAsync => true,
            _ => false,
        }
    }

    pub fn is_fasync_generator(&self) -> bool {
        match self {
            FunKind::FAsyncGenerator => true,
            _ => false,
        }
    }
}

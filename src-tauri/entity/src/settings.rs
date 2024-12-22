//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "settings"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub language: String,
    pub startup: bool,
    pub synchronize: bool,
    pub dark_mode: bool,
    pub tooltip: bool,
    pub display_scale: f32,
    pub position: String,
    pub max_file_size: i32,
    pub max_image_size: i32,
    pub max_text_size: i32,
    pub max_rtf_size: i32,
    pub max_html_size: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Language,
    Startup,
    Synchronize,
    DarkMode,
    Tooltip,
    DisplayScale,
    Position,
    MaxFileSize,
    MaxImageSize,
    MaxTextSize,
    MaxRtfSize,
    MaxHtmlSize,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Language => ColumnType::String(StringLen::N(2u32)).def(),
            Self::Startup => ColumnType::Boolean.def(),
            Self::Synchronize => ColumnType::Boolean.def(),
            Self::DarkMode => ColumnType::Boolean.def(),
            Self::Tooltip => ColumnType::Boolean.def(),
            Self::DisplayScale => ColumnType::Float.def(),
            Self::Position => ColumnType::String(StringLen::None).def(),
            Self::MaxFileSize => ColumnType::Integer.def(),
            Self::MaxImageSize => ColumnType::Integer.def(),
            Self::MaxTextSize => ColumnType::Integer.def(),
            Self::MaxRtfSize => ColumnType::Integer.def(),
            Self::MaxHtmlSize => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

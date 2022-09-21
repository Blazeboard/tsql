use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "performance_pc")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub ip: String,
    pub disk: i32,
    pub data_time: Option<DateTime>,
    pub network_frequency: Option<f64>,
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<f64>,
    pub disk_usage: Option<f64>,
    pub create_time: DateTime,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

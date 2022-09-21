mod entities;

use clap::Parser;
use futures::executor::block_on;
use sea_orm::*;
use sea_orm::prelude::*;
use entities::{prelude::*, *};
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow;

#[derive(Serialize, Deserialize)]
struct Performance {
    id: i64,
    ip: String,
    disk: i32,
    data_time: Option<DateTime>,
    network_frequency: Option<f64>,
    cpu_usage: Option<f64>,
    memory_usage: Option<f64>,
    disk_usage: Option<f64>,
    create_time: DateTime,
    update_time: Option<DateTime>,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Database URL
    #[clap(
        short,
        long,
        value_parser,
        default_value = "root:password@localhost:3306"
    )]
    db_url: String,

    /// The JSON file name
    #[clap(short, long, value_parser)]
    file: String,

    #[clap(short, long, value_parser)]
    name_db: String,
}

async fn run(db_url: &str, p: Performance, name_db: &str) -> Result<(), DbErr> {
    let db = Database::connect(format!("mysql://{}", db_url)).await?;

    let db = {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", name_db),
            ))
            .await?;

            let url = format!("mysql://{}/{}", db_url, name_db);
            let db = Database::connect(&url).await?;

            db.execute(Statement::from_string(db.get_database_backend(), String::from(
                "create table if not exists performance_pc
            (
                id                bigint auto_increment
                    primary key,
                ip                varchar(30)                        not null comment 'ip',
                disk              int                                not null comment '磁盘大小',
                data_time         datetime                           null comment '时间',
                network_frequency double                             null comment '网络频率',
                cpu_usage         double                             null comment 'cpu使用率',
                memory_usage      double                             null comment '内存使用率',
                disk_usage        double                             null comment '磁盘使用率',
                create_time       datetime default CURRENT_TIMESTAMP not null comment '创建时间',
                update_time       datetime default CURRENT_TIMESTAMP null on update CURRENT_TIMESTAMP
            )
                comment '电脑性能';")))
            .await?;

            db
    };

    let item = performance_pc::ActiveModel {
        id: ActiveValue::Set(p.id),
        ip: ActiveValue::Set(p.ip),
        disk: ActiveValue::Set(p.disk),
        data_time: ActiveValue::Set(p.data_time),
        network_frequency: ActiveValue::Set(p.network_frequency),
        cpu_usage: ActiveValue::Set(p.cpu_usage),
        memory_usage: ActiveValue::Set(p.memory_usage),
        disk_usage: ActiveValue::Set(p.disk_usage),
        create_time: ActiveValue::Set(p.create_time),
        update_time: ActiveValue::Set(p.update_time),
    };

    PerformancePc::insert(item).exec(&db).await?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let json_str = fs::read_to_string(args.file)?;
    let p: Performance = serde_json::from_str(&json_str)?;

    block_on(run(&args.db_url, p, &args.name_db))?;

    print!("推送成功！");
    Ok(())
}

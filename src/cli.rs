use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claude-switch")]
#[command(about = "切换 Claude Code 项目配置")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 列出所有配置，标记当前活跃
    #[command(visible_aliases = ["ls"])]
    List,

    /// 切换到指定配置
    Use {
        /// 配置名称
        name: String,
    },

    /// 添加一个新的配置
    Add {
        /// 配置名称
        name: String,
        /// 覆盖已存在的配置
        #[arg(long, short)]
        force: bool,
    },

    /// 显示当前活跃配置名称
    #[command(visible_aliases = ["show"])]
    Current,

    /// 删除指定配置
    #[command(visible_aliases = ["rm"])]
    Delete {
        /// 配置名称
        name: String,
        /// 跳过删除活跃配置的确认
        #[arg(long, short)]
        force: bool,
    },

    /// 查看当前配置与指定配置的差异
    Diff {
        /// 配置名称
        name: String,
    },
}
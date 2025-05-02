use chai::config::SolverConfig;
use chai::encoders::编码器;
use chai::objectives::{default::默认目标函数, 目标函数};
use chai::operators::default::默认操作;
use chai::optimizers::{优化方法, 优化问题};
use chai::{命令, 命令行, 命令行参数, 错误};
use clap::Parser;
use libchai_smdc::四码定长编码器;
use std::thread::spawn;

fn main() -> Result<(), 错误> {
    let 命令行参数 = 命令行参数::parse();
    let 命令行 = 命令行::新建(命令行参数, None);
    let 数据 = 命令行.准备数据();
    match 命令行.参数.command {
        命令::Encode => {
            let mut 编码器 = 四码定长编码器::新建(&数据)?;
            let mut 目标函数 = 默认目标函数::新建(&数据)?;
            let mut 编码结果 = 编码器.编码(&数据.初始映射, &None).clone();
            let 码表 = 数据.生成码表(&编码结果);
            let (指标, _) = 目标函数.计算(&mut 编码结果, &数据.初始映射);
            命令行.输出编码结果(码表);
            命令行.输出评测指标(指标);
        }
        命令::Optimize => {
            let 线程数 = 命令行.参数.threads.unwrap_or(1);
            let 配置 = 数据.配置.clone();
            let SolverConfig::SimulatedAnnealing(退火) =
                配置.optimization.unwrap().metaheuristic.unwrap();
            let mut 线程池 = vec![];
            for 线程编号 in 0..线程数 {
                let 编码器 = 四码定长编码器::新建(&数据)?;
                let 目标函数 = 默认目标函数::新建(&数据)?;
                let 操作 = 默认操作::新建(&数据)?;
                let mut 问题 = 优化问题::新建(数据.clone(), 编码器, 目标函数, 操作);
                let 退火 = 退火.clone();
                let 子界面 = 命令行.生成子命令行(线程编号);
                let 线程 = spawn(move || 退火.优化(&mut 问题, &子界面));
                线程池.push(线程);
            }
            let mut 计算结果列表 = vec![];
            for 线程 in 线程池 {
                计算结果列表.push(线程.join().unwrap());
            }
            计算结果列表.sort_by(|a, b| a.分数.partial_cmp(&b.分数).unwrap());
            for 计算结果 in 计算结果列表 {
                print!("{}", 计算结果.指标);
            }
        }
    }
    Ok(())
}

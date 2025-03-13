use chai::config::SolverConfig;
use chai::encoders::Encoder;
use chai::metaheuristics::Metaheuristic;
use chai::objectives::{Objective, default::DefaultObjective};
use chai::problems::default::DefaultProblem;
use chai::representation::{Assets, Representation};
use chai::{Args, Command, CommandLine, Error};
use clap::Parser;
use libchai_smdc::四码定长编码器;
use std::thread::spawn;

fn main() -> Result<(), Error> {
    let 命令行参数 = Args::parse();
    let 命令行界面 = CommandLine::new(命令行参数, None);
    let (配置, 资源) = 命令行界面.prepare_file();
    let Assets {
        key_distribution: 按键分布,
        pair_equivalence: 双键当量,
        encodables: 词信息,
    } = 资源;
    let _配置 = 配置.clone();
    let length = 词信息.len();
    let 环境 = Representation::new(配置)?;
    match 命令行界面.args.command {
        Command::Encode => {
            let mut 编码器 = 四码定长编码器::new(&环境, 词信息)?;
            let mut 目标函数 = DefaultObjective::new(&环境, 按键分布, 双键当量, length)?;
            let 编码输出 = 编码器.encode(&环境.initial, &None).clone();
            let 码表 = 环境.export_code(&编码输出, &编码器.词信息);
            let (metric, _) = 目标函数.evaluate(&mut 编码器, &环境.initial, &None);
            命令行界面.write_encode_results(码表);
            命令行界面.report_metric(metric);
        }
        Command::Optimize => {
            let 线程数 = 命令行界面.args.threads.unwrap_or(1);
            let SolverConfig::SimulatedAnnealing(退火) =
                _配置.optimization.unwrap().metaheuristic.unwrap();
            let mut 线程池 = vec![];
            for 线程编号 in 0..线程数 {
                let 编码器 = 四码定长编码器::new(&环境, 词信息.clone())?;
                let 目标函数 =
                    DefaultObjective::new(&环境, 按键分布.clone(), 双键当量.clone(), length)?;
                let mut 问题 = DefaultProblem::new(环境.clone(), 目标函数, 编码器)?;
                let 退火 = 退火.clone();
                let 子界面 = 命令行界面.make_child(线程编号);
                let 线程 = spawn(move || 退火.solve(&mut 问题, &子界面));
                线程池.push(线程);
            }
            let mut 计算结果列表 = vec![];
            for 线程 in 线程池 {
                计算结果列表.push(线程.join().unwrap());
            }
            计算结果列表.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
            for 计算结果 in 计算结果列表 {
                print!("{}", 计算结果.metric);
            }
        }
    }
    Ok(())
}

use chai::config::Config;
use chai::objectives::default::DefaultObjective;
use chai::problems::Problem;
use chai::problems::default::DefaultProblem;
use chai::representation::Assets;
use chai::{Args, CommandLine};
use chai::{Error, representation::Representation};
use criterion::{Criterion, criterion_group, criterion_main};
use libchai_smdc::四码定长编码器;

fn 测试(配置: Config, 资源: Assets, b: &mut Criterion) -> Result<(), Error> {
    let representation = Representation::new(配置)?;
    let Assets {
        encodables,
        key_distribution,
        pair_equivalence,
    } = 资源;
    let 词数 = encodables.len();
    let 编码器 = 四码定长编码器::new(&representation, encodables)?;
    let 目标函数 =
        DefaultObjective::new(&representation, key_distribution, pair_equivalence, 词数)?;
    let mut 问题 = DefaultProblem::new(representation, 目标函数, 编码器)?;
    let 映射 = 问题.initialize();
    b.bench_function("Evaluation", |b| {
        b.iter(|| {
            let 移动的元素 = 问题.constrained_random_move(&mut 映射.clone());
            问题.rank(&映射, &Some(移动的元素));
        })
    });
    Ok(())
}

fn 四码定长字词(b: &mut Criterion) {
    let 命令行参数 = Args::生成("米十五笔");
    let (配置, 资源) = CommandLine::new(命令行参数, None).prepare_file();
    测试(配置, 资源, b).unwrap();
}

fn 四码定长单字(b: &mut Criterion) {
    let 命令行参数 = Args::生成("米十五笔");
    let (mut 配置, mut 资源) = CommandLine::new(命令行参数, None).prepare_file();
    资源.encodables = 资源
        .encodables
        .into_iter()
        .filter(|x| x.name.chars().count() == 1)
        .collect();
    配置.optimization.as_mut().unwrap().objective.words_short = None;
    测试(配置, 资源, b).unwrap();
}

criterion_group!(benches, 四码定长字词, 四码定长单字);
criterion_main!(benches);

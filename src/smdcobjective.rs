use std::fmt::Display;

use chai::{data::{元素映射, 数据, 编码信息}, objectives::{default::默认目标函数, metric::默认指标, 目标函数}, 错误};
use serde::Serialize;

pub struct 四码定长目标函数 {
	默认目标函数: 默认目标函数,
	进制: u64,
}

#[derive(Clone, Serialize)]
pub struct 四码定长指标 {
	默认指标: 默认指标,
	四码率: f64,
}

impl Display for 四码定长指标 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\n", self.默认指标)?;
		write!(f, "四码率: {}", self.四码率)
	}
}

impl 四码定长目标函数 {
	pub fn 新建(数据: &数据) -> Result<Self, 错误> {
		let 默认目标函数 = 默认目标函数::新建(数据)?;
		let 进制 = 数据.进制;
		Ok(Self {
			默认目标函数,
			进制
		})
	}
}

impl 目标函数 for 四码定长目标函数 {
	type 目标值 = 四码定长指标;
	
	fn 计算(
		&mut self, 编码结果: &mut [编码信息], 映射: &元素映射
	) -> (Self::目标值, f64) {
		let (默认指标, 损失函数) = self.默认目标函数.计算(编码结果, 映射);
		let mut 一字词总频率 = 0.0;
		let mut 四码一字词总频率 = 0.0;
		for 编码结果 in 编码结果.iter() {
			if 编码结果.词长 != 1 { continue; }
			一字词总频率 += 编码结果.频率 as f64;
			if 编码结果.简码.原始编码 > self.进制 * self.进制 * self.进制 {
				四码一字词总频率 += 编码结果.频率 as f64;
			}
		}
		let 四码率 = 四码一字词总频率 / 一字词总频率;
		let 损失函数 = 损失函数 + 四码率 * 25.0;
		let 指标 = 四码定长指标 {
			默认指标,
			四码率,
		};
		(指标, 损失函数)
	}
}

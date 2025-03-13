use chai::Error;
use chai::encoders::{Encodable, Encoder};
use chai::representation::{CodeInfo, Codes, Element, KeyMap, RawEncodable, Representation};
use std::iter::zip;

pub struct 四码定长编码器 {
    pub 进制: u64,
    pub 编码输出: Codes,
    pub 词信息: Vec<Encodable>,
    pub 全码空间: Vec<u8>,
    pub 简码空间: Vec<u8>,
    pub 包含元素的词映射: Vec<Vec<usize>>,
}

impl 四码定长编码器 {
    pub fn new(环境: &Representation, 原始词信息: Vec<RawEncodable>) -> Result<Self, Error> {
        let 最大码长 = 4;
        let 词信息 = 环境.transform_encodables(原始词信息)?;
        let 编码输出 = 词信息.iter().map(CodeInfo::new).collect();
        let 编码空间大小 = 环境.radix.pow(最大码长 as u32) as usize;
        let 全码空间 = vec![u8::default(); 编码空间大小];
        let 简码空间 = 全码空间.clone();
        let mut 包含元素的词映射 = vec![];
        for _ in 0..=环境.element_repr.len() {
            包含元素的词映射.push(vec![]);
        }
        for (索引, 词) in 词信息.iter().enumerate() {
            for 元素 in &词.sequence {
                包含元素的词映射[*元素].push(索引);
            }
        }
        let encoder = Self {
            进制: 环境.radix,
            编码输出,
            词信息,
            全码空间,
            简码空间,
            包含元素的词映射,
        };
        Ok(encoder)
    }

    pub fn 重置空间(&mut self) {
        self.全码空间.iter_mut().for_each(|x| {
            *x = 0;
        });
        self.简码空间.iter_mut().for_each(|x| {
            *x = 0;
        });
    }

    #[inline(always)]
    fn 全码规则(词: &Encodable, 映射: &KeyMap, 进制: u64) -> u64 {
        let 元素序列 = &词.sequence;
        let mut 全码 =
            (映射[元素序列[0]] * 进制 + 映射[元素序列[1]]) * 进制 + 映射[元素序列[2]];
        if 元素序列.len() >= 4 {
            全码 = 全码 * 进制 + 映射[元素序列[3]];
        }
        全码
    }

    fn 输出全码(&mut self, 映射: &KeyMap, 移动的元素: &Option<Vec<Element>>) {
        let 编码输出 = &mut self.编码输出;
        let 进制 = self.进制;
        if let Some(移动的元素) = 移动的元素 {
            for 元素 in 移动的元素 {
                for 索引 in &self.包含元素的词映射[*元素] {
                    let 词 = &self.词信息[*索引];
                    let 全码 = 四码定长编码器::全码规则(词, 映射, 进制);
                    编码输出[*索引].full.set_code(全码);
                }
            }
        } else {
            for (词, 输出指针) in zip(&self.词信息, 编码输出.iter_mut()) {
                let 全码 = 四码定长编码器::全码规则(词, 映射, 进制);
                输出指针.full.set_code(全码);
            }
        }

        for 输出指针 in 编码输出.iter_mut() {
            let 全码信息 = &mut 输出指针.full;
            let 是否重码 = self.全码空间[全码信息.code as usize] > 0;
            全码信息.set_duplicate(是否重码);
            self.全码空间[全码信息.code as usize] += 1;
        }
    }

    fn 输出简码(&mut self) {
        let 编码输出 = &mut self.编码输出;
        let 进制 = self.进制;
        for (输出指针, 词) in zip(编码输出.iter_mut(), &self.词信息) {
            let 全码 = &输出指针.full.code;
            let 简码信息 = &mut 输出指针.short;
            if 词.length == 1 {
                let 一简 = 全码 % 进制;
                let 重数 = self.全码空间[一简 as usize] + self.简码空间[一简 as usize];
                if 重数 == 0 {
                    简码信息.set(一简, false);
                    self.简码空间[一简 as usize] += 1;
                    continue;
                }
                let 二简 = 全码 % (进制 * 进制);
                let 重数 = self.全码空间[二简 as usize] + self.简码空间[二简 as usize];
                if 重数 == 0 {
                    简码信息.set(二简, false);
                    self.简码空间[二简 as usize] += 1;
                    continue;
                }
                let 三简 = 全码 % (进制 * 进制 * 进制);
                let 重数 = self.全码空间[三简 as usize] + self.简码空间[三简 as usize];
                if 重数 == 0 {
                    简码信息.set(三简, false);
                    self.简码空间[三简 as usize] += 1;
                    continue;
                }
            }
            let 全码是否重码 = self.简码空间[*全码 as usize] > 0;
            简码信息.set(*全码, 全码是否重码);
            self.简码空间[*全码 as usize] += 1;
        }
    }
}

impl Encoder for 四码定长编码器 {
    fn encode(&mut self, keymap: &KeyMap, moved_elements: &Option<Vec<Element>>) -> &mut Codes {
        self.重置空间();
        self.输出全码(keymap, moved_elements);
        self.输出简码();
        &mut self.编码输出
    }
}

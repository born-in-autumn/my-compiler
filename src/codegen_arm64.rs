use std::io::{BufWriter, Write};
use std::{collections::HashMap };
use std::fs::File;
use std::path::Path;
use crate::lower::{IrInst , IrValue};

pub struct GenArm64 {
    pub offset_map: HashMap<String, i64>
}

impl GenArm64 {
    pub fn gen_arm64(&mut self, ir: Vec<IrInst>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut idx = 0; // 记录下当前偏移了几个变量的字节
        let mut has_print = false;
        // 先生成prologue
        let count = self.gen_prologue(&mut result , &ir, &mut has_print);
        // let has_print = false;
        for code in ir {
            match code {
                IrInst::Load { dst, var } => {
                    // 先通过hashmap把a的位置找到
                    let i = self.offset_map.get(&var).unwrap();
                    // 拿到的idx，从开始读出这个东西，放到x1
                    result.push(format!("ldr x1, [sp, #{}]", i));
                    // 用新的x1存到新的栈空间上,start是下一个可以开始存的位置
                    let start = idx*16;
                    result.push(format!("str x1, [sp, #{}]", start));
                    let new_key = format!("t{}", dst.idx);
                    // 把t0存进去
                    self.offset_map.insert(new_key, idx*16); 
                    idx+=1;

                }
                //把一个数放到内存里，标准写法是两步
                // mov x1, #20
                // str x1, [sp]
                IrInst::Store { var, src } => {
                    match src {
                        IrValue::Const(i) => {
                            let s = format!("mov x1, #{}", i);
                            result.push(s);
                            result.push("str x1, [sp]".to_string());
                            // 记录下，存了a，并且a的偏移量，例如idx等于0，就代表现在没有变量存进来，于是第一个变量就分配idx*16-（idx+1)*16,也就是0-16字节，以此类推
                            self.offset_map.insert(var, idx*16); 
                            idx +=1;
                        }
                        // 这种情况先不处理
                        IrValue::Temp(t) => {

                        }
                    }

                }
                IrInst::Print { src } => match src {
                    IrValue::Const(i) => {
                        // 先不处理这种情况
                    }
                    IrValue::Temp(t) => {
                        // 找到位置
                        let key = format!("t{}", t.idx);
                        let i = self.offset_map.get(&key).unwrap(); // 理论上应该是16
                        // 扔进x1
                        result.push(format!("ldr x1, [sp, #{}]", i));
                        // 读取
                        result.push("adrp   x0, Lfmt@PAGE".to_string());
                        result.push("add    x0, x0, Lfmt@PAGEOFF".to_string());
                        result.push("bl     _printf".to_string());
                    }
                },
                _ => {}
            }
        }

        // 流程走完之后，生成epilogue
        self.gen_epilogue(&mut result, count, &has_print);

        result
    }
    pub fn gen_prologue(&self, result: &mut Vec<String>, ir: &Vec<IrInst>, has_print:&mut bool ) -> i64 {
        // 计数, 每当有一个能产生变量的操作，计数器加1，在这里是Load/Store操作
        let mut count = 0;
        result.push(".globl _main".to_string());
        result.push(".p2align 2".to_string());
        result.push("_main:".to_string());
        // 先全扫一遍
        for code in ir {
            match code {
                IrInst::Load { dst, var } => {
                    count+=1;
                }

                IrInst::Store { var, src } => {
                    count+=1;
                }
                IrInst::Print { src } => match src {
                    IrValue::Const(i) => {
                        *has_print = true;
                    }
                    IrValue::Temp(t) => {
                        *has_print = true;
                    }
                },
                _ => {}
            }
        }
        // 扫完之后，看下所有的状态，如果有print, 改一下x30
        if *has_print {
            result.push("sub  sp, sp,#16".to_string());
            result.push("str  x30, [sp]".to_string());
        }
        if count > 0 {
            result.push(format!("sub  sp, sp, #{}", count*16));
        }
        count
    }

    fn gen_epilogue(&self, result: &mut Vec<String> , count: i64, has_print:&bool) {
        // 在这里要开始复原那些东西了
        // 先还第一部分，count欠的那部分
        result.push(format!("add sp, sp, #{}", count*16));
        // x0归0
        result.push("mov x0, #0".to_string());

        // 这个时候要看下有没有print，如果有print，说明sp在原来x30的地方，如果没有说明sp已经回到原位了
        if *has_print {
            result.push("ldr x30 ,[sp]".to_string());
            result.push("add sp, sp, #16".to_string());
        }
        // return
        result.push("ret".to_string());
        // 底部
        result.push(".section __TEXT,__cstring".to_string());
        result.push("Lfmt:".to_string());
        let s = ".asciz \"%lld\\n\"";
        result.push(s.to_string());

    }
    pub fn write_arm64_code(&self, path: &Path, lines: &[String]) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        for line in lines {
            writeln!(writer, "{}" , line)?;
        }
        writer.flush()?;
        Ok(())
    }
}

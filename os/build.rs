use std::fs::{File, read_dir};
use std::io::{Result, Write};

static TARGET_PATH: &str = "../rclib/target/riscv64gc-unknown-none-elf/release/";
static LINK_APP: &str = "./src/link_app.S";
static APP_DIR: &str = "../rclib/src/bin/";

fn main() {
    println!("cargo:return-if-changed=../rclib/src/");
    println!("cargo:return-if-changed={}", TARGET_PATH);

    insert_app_data().unwrap();
}

fn insert_app_data() -> Result<()> {
    let mut f =File::create(LINK_APP)?;
    let mut apps = read_dir(APP_DIR)?.into_iter().map(|x| {
        let mut app_name_with_ext = x.unwrap().file_name().into_string().unwrap();
        app_name_with_ext.drain(app_name_with_ext.find(".").unwrap()..app_name_with_ext.len());
        app_name_with_ext
    }).collect::<Vec<_>>();
    apps.sort();

    // .align 3按照8字节对齐
    // .quad 是 GNU 汇编器（GAS） 的一个伪指令，用于定义 8 字节（64 位）的数据。
    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .globl _name_app
_name_app:
    .quad {}"#,
    apps.len()
    )?;

    for i in 0..apps.len() {
        writeln!(f, r#".   .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#".   .quad app_{}_end"#, apps.len()-1)?;

    apps.into_iter().enumerate().for_each(|(i, v)| {
        writeln!(
            f, r#"
    .section .data
    .globl app_{0}_start
    .globl app_{0}_end
app_{0}_start:
    .incbin {2}/{1}.bin
app_{0}_end"#,
    i, v, TARGET_PATH).unwrap();
    });
    Ok(())
}

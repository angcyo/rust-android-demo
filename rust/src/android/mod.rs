use std::io::{Read, Write};

use jni::objects::JIntArray;
use jni::sys::{jint, jintArray};
use jni::JNIEnv;

pub mod path;

// 2023-05-21
// 通过rust代码, 获取bitmap的像素数据

/// 将[jintArray]转换成[u32]
pub unsafe fn convert_jint_array_to_u32_array(
    env: &mut JNIEnv,
    jint_array: &JIntArray,
) -> Vec<u32> {
    //let env = &mut *env;
    let mut result: Vec<u32> = Vec::new();
    let jint_array_length = env.get_array_length(jint_array).unwrap();
    let mut jint_array_buffer: Vec<jint> = vec![0; jint_array_length as usize];
    env.get_int_array_region(jint_array, 0, jint_array_buffer.as_mut_slice())
        .unwrap();
    for i in 0..jint_array_length {
        result.push(jint_array_buffer[i as usize] as u32);
    }
    result
}

/// 将[JString]转换成[String]
pub unsafe fn convert_jstring_to_string(
    env: &mut JNIEnv,
    jstring: &jni::objects::JString,
) -> String {
    //let env = &mut *env;
    let jstring = env.get_string(jstring).unwrap();
    let result = jstring.to_str().unwrap().to_string();
    result
}

/// 将[String]转换成[jstring]
pub unsafe fn convert_string_to_jstring<'local>(
    env: &mut JNIEnv<'local>,
    string: &str,
) -> jni::objects::JString<'local> {
    //let env = &mut *env;
    let jstring = env.new_string(string).unwrap();
    jstring
}

/// 将[Vec<String>]转换成[jstring]
pub unsafe fn convert_vec_string_to_jstring<'local>(
    env: &mut JNIEnv<'local>,
    vec_string: &Vec<String>,
) -> jni::objects::JString<'local> {
    let string = vec_string.join("");
    convert_string_to_jstring(env, &string)
}

/// 使用Rust创建文件
pub fn create_file(path: &str) -> std::fs::File {
    std::fs::File::create(path).unwrap()
}

/// 使用Rust创建并写入文件, 如果文件不存在会创建, 如果文件存在会覆盖
pub fn write_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// 使用Rust打开文件
pub fn open_file(path: &str) -> std::fs::File {
    //std::io::Result<std::fs::File>
    std::fs::File::open(path).unwrap()
}

///使用Rust判断文件是否存在
pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

///读取文件所有内容
pub fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// 追加写入文件, [write_file]
pub fn append_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file;
    if file_exists(path) {
        file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)?;
    } else {
        file = std::fs::File::create(path)?;
    }
    file.write_all(content.as_bytes())?;
    Ok(())
}

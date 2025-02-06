use crate::android::path::Path::{line_to, move_to, new_path};
use crate::android::{
    append_file, convert_jstring_to_string, convert_string_to_jstring, create_file, read_file,
    write_file,
};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jfloat, jint, jobject, jstring};
use jni::{sys, JNIEnv, JavaVM};
use rand::Rng;
use std::io::Write;

mod android;

/// so加载时, 就会触发的初始化方法.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *mut std::os::raw::c_void) -> jint {
    //panic!("初始化失败");
    sys::JNI_VERSION_1_6
}

/// 测试jni连通性
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_angcyo_rust_1android_1demo_Rust_test(
    mut env: JNIEnv,
    _class: JClass,
) -> jfloat {
    let mut rng = rand::rng();

    // 生成一个范围在 1 到 100 之间的随机整数
    //let random_number: u32 = rng.random_range(1..=100);
    //println!("生成的随机数是: {}", random_number);

    // 生成一个随机浮点数
    let random_float: f64 = rng.random();
    //println!("生成的随机浮点数是: {}", random_float);
    random_float as jfloat
}

/// 测试文件的创建,读写
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_angcyo_rust_1android_1demo_Rust_testFile(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jstring {
    let path = convert_jstring_to_string(&mut env, &path);
    let mut file = create_file(&path);
    file.write_all(b"Hello, Rust 1!\n").unwrap();
    file.write_all("Hello, Rust 2!\n".as_bytes()).unwrap();

    append_file(&path, "Hello, Rust 3!\n").unwrap();

    let mut rng = rand::rng();
    let random_float: f64 = rng.random();
    append_file(&path, &format!("random->{}\n", random_float)).unwrap();

    let content = read_file(&path).unwrap();
    let path = convert_string_to_jstring(&mut env, &content);
    path.into_raw()
}

/// 测试通过反射创建对象并返回
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_angcyo_rust_1android_1demo_Rust_testPath(
    mut env: JNIEnv,
    _class: JClass,
    path: JObject,
) -> jobject {
    //let path = path.into_raw();

    //let new_path = env.new_object("android/graphics/Path", "()V", &[]).unwrap();

    let new_path = new_path(&mut env);
    move_to(&mut env, &new_path, 100.0, 100.0);
    line_to(&mut env, &new_path, 200.0, 200.0);

    //move_to(env, &new_path, 100.0, 100.0);
    //line_to(&env, &new_path, 200.0, 200.0);
    new_path.into_raw()
}

/// 测试写入内容到指定的文件路径
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_angcyo_rust_1android_1demo_Rust_writeFile(
    mut env: JNIEnv,
    _class: JClass,
    content: JString,
    append: jboolean,
    path: JString,
) -> jboolean {
    let content = convert_jstring_to_string(&mut env, &content);
    let path = convert_jstring_to_string(&mut env, &path);
    let append = append == 1;
    if append {
        append_file(&path, &content).unwrap();
    } else {
        write_file(&path, &content).unwrap();
    }
    jboolean::from(true)
}

/// 测试读取指定的文件路径内容
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_com_angcyo_rust_1android_1demo_Rust_readFile(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jstring {
    let path = convert_jstring_to_string(&mut env, &path);
    let content: String;
    match read_file(&path) {
        Ok(str) => content = str,
        Err(_) => content = String::from(""),
    }
    convert_string_to_jstring(&mut env, &content).into_raw()
}

//--

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

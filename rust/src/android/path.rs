//! 2023-5-22
//! Rust中操作Android Path的一些方法
//!

#[allow(non_snake_case)]
pub mod Path {
    use jni::objects::{JObject, JValue};
    use jni::JNIEnv;

    /// 创建一个Path对象
    pub unsafe fn new_path<'local>(env: &mut JNIEnv<'local>) -> JObject<'local> {
        let path = env.new_object("android/graphics/Path", "()V", &[]).unwrap();
        path
    }

    /// 移动到指定位置
    pub unsafe fn move_to(env: &mut JNIEnv, path: &JObject, x: f32, y: f32) {
        let args = [JValue::from(x), JValue::from(y)];
        env.call_method(path, "moveTo", "(FF)V", &args).unwrap();
    }

    /// 画线到指定位置
    pub unsafe fn line_to(env: &mut JNIEnv, path: &JObject, x: f32, y: f32) {
        let args = [JValue::from(x), JValue::from(y)];
        env.call_method(path, "lineTo", "(FF)V", &args).unwrap();
    }
}

#include <jni.h>
#include <string>

extern "C" JNIEXPORT jstring JNICALL
Java_com_angcyo_rust_1android_1demo_NativeLib_stringFromJNI(
        JNIEnv* env,
        jobject /* this */) {
    std::string hello = "Hello from C++";
    return env->NewStringUTF(hello.c_str());
}

extern "C"
JNIEXPORT jstring JNICALL
Java_com_angcyo_rust_1android_1demo_Rust_testFile(JNIEnv *env, jobject thiz, jstring path) {
    // TODO: implement testFile()
}
extern "C"
JNIEXPORT jobject JNICALL
Java_com_angcyo_rust_1android_1demo_Rust_testPath(JNIEnv *env, jobject thiz, jobject path) {
    // TODO: implement testPath()
}
extern "C"
JNIEXPORT jboolean JNICALL
Java_com_angcyo_rust_1android_1demo_Rust_writeFile(JNIEnv *env, jobject thiz, jstring content,
                                                   jboolean append, jstring path) {
    // TODO: implement writeFile()
}
extern "C"
JNIEXPORT jstring JNICALL
Java_com_angcyo_rust_1android_1demo_Rust_readFile(JNIEnv *env, jobject thiz, jstring path) {
    // TODO: implement readFile()
}
extern "C"
JNIEXPORT jfloat JNICALL
Java_com_angcyo_rust_1android_1demo_Rust_test(JNIEnv *env, jobject thiz) {
    // TODO: implement test()
}
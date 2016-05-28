# kotlin-rust-android

## Building libhello.so

 - install [rustup](https://www.rustup.rs/)
 - follow along this [blog post](http://blog.rust-lang.org/2016/05/13/rustup.html):
	 - add android arm target
     - install NDK r10e
     - make api 18 standalone toolchain
     - create your cargo config file
     - run `cargo build`, you can add `--release` flag
     - copy the `.so` to `app/jniLibs/armeabi-v7a`
 - run the app

If all goes well, you should see this:
![img](https://dl2.pushbulletusercontent.com/JWStqIWNX16pGtqqYdVUPEPTaMBt38KX/Screenshot_2016-05-28-23-32-03.png)

If python is not found (but python3 is installed), I got the simple app to build by simply:

```sh
sudo cp /usr/bin/python3 /usr/bin/python
```

App compiles, but the emulator doesn't actually show the app. Launch apparently
succeeds but logcat shows the following error: 

`java.lang.UnsatisfiedLinkError: dlopen failed: library "libsimple.so" not found`.

Looking this up I find [this](https://stackoverflow.com/questions/52076641/java-lang-unsatisfiedlinkerror-dlopen-failed-library-not-found)
article. 

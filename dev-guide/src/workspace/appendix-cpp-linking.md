# Appendix: Library Linking in C/C++

Let's see an example how C/C++ links to the external OpenCV library.

```bash
g++ \
    -I ~/my_opencv/include \
    -L ~/my_opencv/lib \
    -l opencv_core \
    main.cpp
```

Let's explain the compiler options.

The option  `-I` tells the directory to search for header files. For example, if
 we write the include code in C++, we have to make sure the
 file`~/my_opencv/include/opencv4/opencv2/core.hpp` exists.

```c++
#include <opencv4/opencv2/core.hpp>
```

The option `-L` tells the directory to search for library files, named
 `libname.a` and `libname.so` on Linux. It affects the later `-l`
 option.

The option `-l` asks the output binary to link to `opencv_core`. The
compiler will search for `libopencv_core.a` or `libopencv_core.so`
files in system default directories and those provided by `-L`. Here
we assume the file `~/my_opencv/lib/libopencv_core.a` exists.

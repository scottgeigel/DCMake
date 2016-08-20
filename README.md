# Dynamic C Make
A project management tool for the Digi International's Rabbit micro processor language Dynamic C.

The intent of this program is to get around Dynamic C's single file limitation in a portable manner.
Instead of creating lib files, you can create header files and source files like a normal C project.
This program, at the core, concatanates your files. So you will still have to work with Dynamic C
in the sense that some lib files may need to be included and standard lib files -- from C -- should
not be included.

You can this by placing code like this in your files
```
#ifdef DYNAMICC_MODE
#require "mylib.h"
#else
#include "mylib.h"
#include <stdio.h>
#endif
```

This can allow you, for example, make a desktop version of your project to test before loading
on to the microcontroller.

# How to use (Simple)

simply run:
```
dynamic_c_make < dc_make_file > output_file.c
```
Where output\_file is the source you want to compile with Dynamic C and
dc\_make\_file is a source listing in a format like so:
```
main.c
mylib.c
```

The program will handle the rest.


# Alpha Version

This project currently only does the bare minimum. It will only do the basic substitution.

It will not follow preprocessor directives, so everytime it sees the magic phrase \#require, it 
will begin substituting.

None of the command line arguments really work yet.

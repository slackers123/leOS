# The idea for how this project will be structured

leo (the workspace/root project directory)
 - leo-shared (shared implementations making sure to be available cross platform)
 - leo-libs (libraries used by applications and services)
 - leos-kernel (the kernel of the leos operating system)
 - leos-serivces (services of the leos operating system)
 - leo-applications (applications mainly for the leos operating system but also as standalone applications)
    - leo-demos (functionality demos)
     - hello world program with leo main etc
   - leo-browser (the browser of the leo project)
 - leo-tests (test for the project )

## Note:
To add functionality we are allowed to use:
- Rust builtins (core, alloc, etc.)
- self written code
- code from others (as little as possible) with proper citation

Using crates/libraries from outside of this repo and not written for
this project goes against the spirit of the project

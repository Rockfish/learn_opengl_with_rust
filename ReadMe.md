# Learn OpenGL

A Rust learning project based on [Learn OpenGL](https://learnopengl.com).

Working on porting Joey DeVries' LearnOpenGL project, https://github.com/JoeyDeVries/learnopengl, to Rust.

See the examples directory for the examples that follow the book.

## Dependencies

* glfw - For window and OpenGL context. https://docs.rs/glfw/0.52.0/glfw/


* glad - OpenGL API bindings generated from https://gen.glad.sh/ 
  See https://github.com/Dav1dde/glad/tree/glad2


* image - Image library. https://docs.rs/image/0.24.7/image/


* glam - Math library. https://docs.rs/glam/latest/glam/


* assimp - For modeling loading using the Assimp library, https://github.com/assimp/assimp


* russimp - For assimp rust bindings, https://github.com/jkvargas/russimp

## Notes

* Renamed folders to use dashes and underscores because cargo complains about periods in the package name. 
  Also changed the files names to have a consistent naming schema. 
* Kept variable name casing the same as the original code to make porting easier and easier to compare to the original cpp code. 
* Overrode Clippy warnings for zero_ptr and assign_op_pattern to stay closer to the original cpp code.
* External textures of Obj files don't appear in the russimp Scene object because of the way it loads materials. 
  Decided to work around it by calling the assimp functions directly through russimp binding instead of relying on russimp scene loading. 

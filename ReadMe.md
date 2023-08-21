# Learn OpenGL

A Rust learning project based on [Learn OpenGL](https://learnopengl.com).

Working on porting Joey DeVries' LearnOpenGL project, https://github.com/JoeyDeVries/learnopengl, to Rust.

See the examples directory for the examples that follow the book.

## Dependencies

* glfw - https://docs.rs/glfw/0.52.0/glfw/
* glad generated from https://gen.glad.sh/ 

  See https://github.com/Dav1dde/glad/tree/glad2
* image - https://docs.rs/image/0.24.7/image/
 

## Notes

* Renamed folders to use dashes and underscores because cargo complains about periods in the package name. 
  Also changed the files names to have a consistent naming schema. 
* Kept variable name casing the same as the original code to make porting easier and easier to compare to the original cpp code. 
* Overrode Clippy warnings for zero_ptr and assign_op_pattern to stay closer to the original cpp code.

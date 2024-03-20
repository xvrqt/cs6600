use crate::shader::error::ShaderError;
use gl::types::*;
// Error for GLProgram
#[derive(Debug)]
pub enum ProgramError {
    ShaderCompilation(ShaderError),
    Linking(String),
    SettingUniformValue(String),
    GetUniformLocation(String),
}

impl std::error::Error for ProgramError {}

impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProgramError::ShaderCompilation(shader_error) => {
                write!(f, "Error in processing a shader.\n{}", shader_error)
            }
            ProgramError::Linking(error_log) => {
                write!(f, "Could not link shaders to the program.\n{}", error_log)
            }
            ProgramError::SettingUniformValue(error) => {
                write!(f, "Failed to set Uniform Value.\n{}", error)
            }
            ProgramError::GetUniformLocation(name) => {
                write!(f, "Failed to find the location for '{}'.\n", name)
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<ProgramError> for crate::GLError {
    fn from(error: ProgramError) -> Self {
        crate::GLError::Program(error)
    }
}

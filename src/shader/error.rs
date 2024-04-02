use crate::error::GLUtilityError;

// Error type for Shaders
#[derive(Debug)]
pub enum ShaderError {
    FailedToParseSource(GLUtilityError),
    FailedToCompileShader(GLUtilityError),
}

impl std::error::Error for ShaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ShaderError::FailedToParseSource(error) => Some(error),
            ShaderError::FailedToCompileShader(error) => Some(error),
        }
    }
}

impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ShaderError::FailedToParseSource(_) => {
                write!(f, "Could not parse the shader's source.")
            }
            ShaderError::FailedToCompileShader(_) => {
                write!(f, "Failed to compile the shader from source.",)
            }
        }
    }
}

// Allows for painless casting into our crate's rollup error
impl From<ShaderError> for crate::GLError {
    fn from(error: ShaderError) -> Self {
        crate::GLError::Shader(error)
    }
}

// Allows for painless casting into our crate's rollup error
impl From<ShaderError> for crate::program::error::ProgramError {
    fn from(error: ShaderError) -> Self {
        crate::program::error::ProgramError::ShaderCompilation(error)
    }
}

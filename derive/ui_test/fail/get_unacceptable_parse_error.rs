use utils_lib_derive::Getter;

#[derive(Getter)]
struct S {
    #[get_mut(this is not valid)] // syn error
    f1: usize,
    #[get(not::an::ident = 1)] // left value error
    f2: usize,
    #[get(not::an::ident(1))] // this pass
    f3: usize,
    #[get(visibility(not::an::ident))] // error right hand not an ident
    f4: usize,
    #[get(visibility = not::a::string)] // error right hand not a string
    f5: usize,
    #[get_mut(visibility = "misformed")] // error right hand misformed
    f6: usize,
}

fn main() {}

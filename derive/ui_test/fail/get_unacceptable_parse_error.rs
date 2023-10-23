// test the reporting of
// OptionParseError::GetterParseError(GetterParseError::<ImmutableOptionList>::AddConfigError)
use utils_lib_derive::Getter;

#[derive(Getter)]
struct S {
    #[get_mut(this is not valid)] // syn error
    f1: usize,
    #[get(not::an::ident = 1)] // left value error
    f2: usize,
    #[get(not::an::ident(1))] // this pass
    f3: usize,
    #[get_mut(visibility(not::an::ident))] // error right hand not an ident
    f4: usize,
    #[get(visibility = not::a::string)] // error right hand not a string
    f5: usize,
    #[get_mut(visibility = "misformed")] // error right hand misformed
    f6: usize,
    #[get(Const = "misformed")] // error right hand misformed
    f7: usize,
    #[get(self_ty=not::an::string)] // error right hand not a string
    f8: usize,
    #[get(getter_ty = not::a::string)] // error right hand not a string
    f9: usize,
    #[get(name = 1)] // error right hand not a string
    f10: usize,
}

fn main() {}

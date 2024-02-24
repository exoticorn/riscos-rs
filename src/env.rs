use crate::sys;

type SyntaxString = heapless::Vec<u8, 256>;

pub unsafe trait Arg {
    type Result;

    fn add_to_syntax(&self, syntax: &mut SyntaxString) -> Result<usize, ()>;
    fn read_result(&self, value: u32) -> Self::Result;
}

unsafe impl Arg for () {
    type Result = ();
    fn add_to_syntax(&self, _syntax: &mut SyntaxString) -> Result<usize, ()> {
        Ok(1)
    }
    fn read_result(&self, _value: u32) -> () {
        ()
    }
}

pub mod arg {
    use super::{Arg, SyntaxString};

    pub struct Switch(pub &'static [u8]);

    unsafe impl Arg for Switch {
        type Result = bool;

        fn add_to_syntax(&self, syntax: &mut SyntaxString) -> Result<usize, ()> {
            add_identifier(syntax, self.0)?;
            syntax.extend_from_slice(b"/S")?;
            Ok(1)
        }

        fn read_result(&self, value: u32) -> bool {
            value != 0
        }
    }

    pub struct String;

    unsafe impl Arg for String {
        type Result = alloc::string::String;

        fn add_to_syntax(&self, _syntax: &mut SyntaxString) -> Result<usize, ()> {
            Ok(1)
        }
        fn read_result(&self, value: u32) -> alloc::string::String {
            let mut result = alloc::string::String::new();
            unsafe {
                let mut ptr = value as *const u8;
                while *ptr != 0 {
                    result.push(*ptr as char);
                    ptr = ptr.add(1);
                }
            }
            result
        }
    }

    pub struct Named<'a, T: Arg>(pub &'a [u8], pub T);

    unsafe impl<'a, T: Arg> Arg for Named<'a, T> {
        type Result = T::Result;

        fn add_to_syntax(&self, syntax: &mut SyntaxString) -> Result<usize, ()> {
            add_identifier(syntax, self.0)?;
            self.1.add_to_syntax(syntax)
        }
        fn read_result(&self, value: u32) -> Self::Result {
            self.1.read_result(value)
        }
    }

    fn add_identifier(syntax: &mut SyntaxString, id: &[u8]) -> Result<(), ()> {
        for &c in id {
            let is_allowed = (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z') || c == b'_';
            if !is_allowed {
                panic!("Invalid identifier {:?}", id);
            }
        }
        syntax.extend_from_slice(id)?;
        Ok(())
    }
}

pub unsafe trait Args {
    type Result;
    fn add_to_syntax(&self, syntax: &mut SyntaxString) -> Result<usize, ()>;
    fn read_result<'a>(&self, value: &'a [u32]) -> (Self::Result, &'a [u32]);
}

unsafe impl<T> Args for T
where
    T: Arg,
{
    type Result = T::Result;

    fn add_to_syntax(&self, syntax: &mut SyntaxString) -> Result<usize, ()> {
        (self as &T).add_to_syntax(syntax)
    }

    fn read_result<'a>(&self, values: &'a [u32]) -> (Self::Result, &'a [u32]) {
        ((self as &T).read_result(values[0]), &values[1..])
    }
}

unsafe impl<T1, T2> Args for (T1, T2)
where
    T1: Args,
    T2: Args,
{
    type Result = (T1::Result, T2::Result);

    fn add_to_syntax(&self, syntax: &mut SyntaxString) -> Result<usize, ()> {
        let mut count = self.0.add_to_syntax(syntax)?;
        syntax.push(b',').map_err(|_| ())?;
        count += self.1.add_to_syntax(syntax)?;
        Ok(count)
    }

    fn read_result<'a>(&self, values: &'a [u32]) -> (Self::Result, &'a [u32]) {
        let (res1, values) = self.0.read_result(values);
        let (res2, values) = self.1.read_result(values);
        ((res1, res2), values)
    }
}

pub fn parse_args<T: Args>(syntax: T) -> Option<T::Result> {
    let mut syntax_string = SyntaxString::new();
    let syntax = ((), syntax);
    let Ok(arg_count) = syntax.add_to_syntax(&mut syntax_string) else {
        panic!("Syntax string overflowed");
    };
    if let Err(_) = syntax_string.push(0) {
        panic!("Syntax string overflowed");
    }
    let mut output_buffer = [0u32; 64];
    let arg_string = sys::os::get_env().command;
    let success = unsafe {
        sys::os::read_args(
            syntax_string.as_ptr(),
            arg_string,
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
        )
    };
    if success {
        let (result, _) = syntax.read_result(&output_buffer[..arg_count]);
        Some(result.1)
    } else {
        None
    }
}

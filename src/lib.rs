#[derive(Debug, Copy, Clone)]
pub enum Action {
    Push,
    Pop,
    Replace,
}

pub struct DPDA {
    pub 
}

macro_rules! state {
    ($v:vis $s:ident $tok:ty, $( ($t:pat, $i:pat) -> ($n:ty, $a:path) ),+) => {
        #[derive(Debug, Copy, Clone)]
        $v struct $s;
        $(
            impl Next<>
        )*
    };
}

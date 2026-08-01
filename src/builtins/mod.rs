pub mod exit;
pub mod echo;
pub mod type_builtin;
pub mod pwd;
pub mod cd;

pub struct Builtin {
    pub name: &'static str,
    pub run: fn(Vec<String>) -> bool
}

pub static BUILTINS: &[Builtin]= &[
    Builtin {
        name: "echo",
        run: echo::run
    },
    Builtin {
        name: "exit",
        run: exit::run
    },
    Builtin {
        name: "type",
        run: type_builtin::run
    },
    Builtin {
        name: "pwd",
        run: pwd::run
    },
    Builtin {
        name: "cd",
        run: cd::run
    }
];

pub fn is_builtin(name: &str) -> bool {
    BUILTINS.iter().any(|builtin| builtin.name == name)
}

pub fn get_builtins() -> impl Iterator<Item = &'static str> {
    BUILTINS.iter().map(|x| x.name)
}
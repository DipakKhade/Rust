macro_rules! avec {
    ($x:ident) => {
        $x += 1
    };
}

fn foo() {
    let mut x = 2;
    avec!(x);
}

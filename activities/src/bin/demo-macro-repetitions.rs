macro_rules! myvec {
    (
        $( $element:expr ),+
        $(,)? // trailing comma
    ) => {{
        let mut v = Vec::new();
        $(
            v.push($element);
        )+
        v
    }};
}

fn main() {
        let v = myvec![1, 2, 3, 4];
        // let mut v = Vec::new();
        // v.push(1);
        // v.push(2);
        // v.push(3);
        // v.push(4);
        // v
        let v2 = myvec![1, 2, 3, 4,];
}

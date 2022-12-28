
/// Sequencer Generation Macro
/// Expands
///```
/// use mithril::seq;
/// seq!(
///     |mut x:i32| {
///         x = x + 1;
///        format!("{}: {}", x, x)
///    }
/// )
/// ```
/// to
///```
/// use mithril::map;
/// use mithril::misc::sequence::sequence_state::SequenceState;
/// Sequencer::new(
///     SequenceState::new(map!{x: i32::default()}),
///     |seq_state| {
///         let x: i32 = seq_state.own("x").unwrap();
///         let _cls = |mut x:i32| { x = x + 1; format!("{}: {}", x, x) };
///         let ret = _cls(x);
///         seq_state.disown("x", x);
///         ret
///     }
/// )
/// ```
#[macro_export]
macro_rules! seq {
    (|$( $name:ident: &mut $type:ty),* | $code:block) => {
        Sequence::new(
            SequenceState::new(map!{$($name => <$type>::default()),*}),
            Box::new(|seq_state: &mut SequenceState| {
                $(
                    let mut $name: $type = seq_state.own(stringify!($name)).unwrap();
                )*
                let _cls = |$($name:&mut $type),*| $code;
                let ret = _cls($(&mut $name),*);
                $(
                    seq_state.disown(stringify!($name), $name);
                )*
                ret
            })
        )
    };
}
/// expands map!{a: b, [c: d, [e: f, [ ... ]]]) to a hashmap containing the key-value pairs
#[macro_export]
macro_rules! map {
    { $($key:expr => $value:expr),* } => {
        {
            let mut map = ::std::collections::HashMap::<String, Box<(dyn Any + 'static)>>::new();
            $(
                map.insert(stringify!($key).to_string(), Box::new($value));
            )*
            map
        }
    };

}

use keyberon::action::Action::Trans;
use keyberon::action::{d, k, l, m};
use keyberon::key_code::KeyCode::*;
use keyberon::action::Action::{self, *};


const CUT: Action = m(&[LShift, Delete]);
const COPY: Action = m(&[LCtrl, Insert]);
const PASTE: Action = m(&[LShift, Insert]);

// const L1_SP: Action = HoldTap {
//     timeout: 200,
//     hold: &l(1),
//     tap: &k(Space),
// };
// const CENTER: Action = m(&[LCtrl, Enter]);

const L1_ALT: Action = HoldTap {
    timeout: 200,
    hold: &l(1),
    tap: &k(LAlt)
};

const L1_FN: Action = HoldTap {
    timeout: 200,
    hold: &l(1),
    tap: &k(No)
};


macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}

macro_rules! a {
    ($k:ident) => {
        m(&[RAlt, $k])
    };
}


#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers = &[
    &[
        &[k(Escape),    k(Kb1),     k(Kb2),        k(Kb3),  k(Kb4),     k(Kb5),     k(Kb6), k(Kb7), k(Kb8),  k(Kb9),      k(Kb0),   k(Minus),   k(Equal),      k(Bslash),      k(Grave)    ],
        &[k(Tab),       Trans,      k(Q),          k(W),    k(E),       k(R),       k(T),   k(Y),   k(U),    k(I),        k(O),     k(P),       k(LBracket),   k(RBracket),    k(BSpace)   ],
        &[k(RCtrl),     Trans,      k(A),          k(S),    k(D),       k(F),       k(G),   k(H),   k(J),    k(K),        k(L),     k(SColon),  k(Quote),      k(Return),      Trans       ],
        &[Trans,        k(LShift),  k(Z),          k(X),    k(C),       k(V),       k(B),   k(N),   k(M),    k(Comma),    k(Dot),   k(Slash),   Trans,         k(RShift),      L1_FN       ],
        &[Trans,        L1_ALT,     Trans,         k(LGui), Trans,      k(Space),   Trans,  Trans,  Trans,   k(RGui),     k(RAlt),  Trans,      Trans,         Trans,          Trans       ],
    ], &[
        &[k(F1),        k(F2),      k(F3),         k(F4),   k(F5),      k(F6),      Trans,  Trans,  Trans,   k(F7),       k(F8),    k(F9),      k(F10),        k(F11),         k(F12)       ],
        &[Trans,        Trans,      Trans,         Trans,   Trans,      Trans,      Trans,  Trans,  Trans,   k(Up),       Trans,    Trans,      Trans,         Trans,          k(Delete)    ],
        &[d(0),         d(1),       k(NumLock),    Trans,   k(Escape),  Trans,      Trans,  Trans,  k(Left), k(Down),     k(Left),  Trans,      Trans,         Trans,          Trans        ],
        &[Trans,        Trans,      CUT,           COPY,    PASTE,      Trans,      Trans,  Trans,  Trans,   Trans,       k(Home),  k(PgDown),  k(PgUp),       k(End),         Trans        ],
        &[Trans,        Trans,      Trans,         Trans,   Trans,      Trans,      Trans,  Trans,  Trans,   Trans,       Trans,   Trans,      Trans,         Trans,          Trans        ],
    ],
];
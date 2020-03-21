use std::fmt::Display;

#[derive(Hash)]
pub enum LengthUnit {
    Em,
    Ex,
    Ch,
    Rem,
    Vw,
    Vh,
    VMin,
    VMax,
    Cm,
    Mm,
    Q,
    In,
    Pc,
    Pt,
    Px,
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthUnit::Em => f.write_str("em"),
            LengthUnit::Ex => f.write_str("ex"),
            LengthUnit::Ch => f.write_str("ch"),
            LengthUnit::Rem => f.write_str("rem"),
            LengthUnit::Vw => f.write_str("vw"),
            LengthUnit::Vh => f.write_str("vh"),
            LengthUnit::VMin => f.write_str("vmin"),
            LengthUnit::VMax => f.write_str("vmax"),
            LengthUnit::Cm => f.write_str("cm"),
            LengthUnit::Mm => f.write_str("mm"),
            LengthUnit::Q => f.write_str("q"),
            LengthUnit::In => f.write_str("in"),
            LengthUnit::Pc => f.write_str("pc"),
            LengthUnit::Pt => f.write_str("pt"),
            LengthUnit::Px => f.write_str("px"),
        }
    }
}

#[derive(Hash)]
pub struct Length {
    value: i32,
    unit: LengthUnit,
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

pub trait ToLength {
    fn em(self) -> Length;
    fn ex(self) -> Length;
    fn ch(self) -> Length;
    fn rem(self) -> Length;
    fn vw(self) -> Length;
    fn vh(self) -> Length;
    fn vmin(self) -> Length;
    fn vmax(self) -> Length;
    fn cm(self) -> Length;
    fn mm(self) -> Length;
    fn q(self) -> Length;
    fn inch(self) -> Length;
    fn pc(self) -> Length;
    fn pt(self) -> Length;
    fn px(self) -> Length;
}

impl ToLength for i32 {
    fn em(self) -> Length {
        Length {
            unit: LengthUnit::Em,
            value: self,
        }
    }
    fn ex(self) -> Length {
        Length {
            unit: LengthUnit::Ex,
            value: self,
        }
    }
    fn ch(self) -> Length {
        Length {
            unit: LengthUnit::Ch,
            value: self,
        }
    }
    fn rem(self) -> Length {
        Length {
            unit: LengthUnit::Rem,
            value: self,
        }
    }
    fn vw(self) -> Length {
        Length {
            unit: LengthUnit::Vw,
            value: self,
        }
    }
    fn vh(self) -> Length {
        Length {
            unit: LengthUnit::Vh,
            value: self,
        }
    }
    fn vmin(self) -> Length {
        Length {
            unit: LengthUnit::VMin,
            value: self,
        }
    }
    fn vmax(self) -> Length {
        Length {
            unit: LengthUnit::VMax,
            value: self,
        }
    }
    fn cm(self) -> Length {
        Length {
            unit: LengthUnit::Cm,
            value: self,
        }
    }
    fn mm(self) -> Length {
        Length {
            unit: LengthUnit::Mm,
            value: self,
        }
    }
    fn q(self) -> Length {
        Length {
            unit: LengthUnit::Q,
            value: self,
        }
    }
    fn inch(self) -> Length {
        Length {
            unit: LengthUnit::In,
            value: self,
        }
    }
    fn pc(self) -> Length {
        Length {
            unit: LengthUnit::Pc,
            value: self,
        }
    }
    fn pt(self) -> Length {
        Length {
            unit: LengthUnit::Pt,
            value: self,
        }
    }
    fn px(self) -> Length {
        Length {
            unit: LengthUnit::Px,
            value: self,
        }
    }
}

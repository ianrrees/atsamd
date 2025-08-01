//! Type-level tools to configure SERCOM pads
//!
//! This module helps configure [`Pin`]s as SERCOM pads. It provides type-level
//! tools to convert `Pin`s to the correct [`PinMode`] and to enforce type-level
//! constraints at compile-time.
//!
//! # Overview
//!
//! A SERCOM pad is defined by two types, its corresponding [`Sercom`] instance
//! and its [`PadNum`], from [`Pad0`] to [`Pad3`]. However, a given SERCOM pad
//! can usually be mapped to several possible [`PinId`]s.
//!
//! There are two primary traits defined in this module:
//! - The [`IsPad`] trait is implemented on `Pin` types that are properly
//!   configured as SERCOM pads, with `PinMode` [`AlternateC`] or
//!   [`AlternateD`]. It acts as both a [type class] for SERCOM pads and as a
//!   [type-level function] to recover the corresponding [`Sercom`] and
//!   [`PadNum`] types from the `Pin`.
//! - The [`GetPad`] trait maps each [`PinId`] to its corresponding, pad-related
//!   types. The [`PadMode`] alias uses `GetPad` to recover the corresponding
//!   `PinMode` for a given SERCOM pad, while the [`Pad`] alias recovers the
//!   configured [`Pin`] type.
//!
//! [`AlternateC`]: crate::gpio::AlternateC
//! [`AlternateD`]: crate::gpio::AlternateD
//! [type class]: crate::typelevel#type-classes
//! [type-level function]: crate::typelevel#type-level-functions
//! # IOSET (SAMD51/SAME5x only)
//!
//! SAMx5x chips do not allow arbitrary combinations of `PinId` for a given
//! SERCOM. Instead, all `PinId`s must belong to the same IOSET. This module
//! defines a [type-level enum], [`IoSet`], to enforce this restriction, and the
//! [`InIoSet`] [type class] is responsible for labeling each `IsPad` type with
//! its corresponding, valid `IoSet`\(s).

use atsamd_hal_macros::{hal_cfg, hal_module};
use paste::paste;
use seq_macro::seq;

use super::Sercom;
#[hal_cfg(any("sercom0-d21", "sercom0-d5x"))]
use crate::gpio::OptionalPinId;
use crate::gpio::{AnyPin, OptionalPin, Pin, PinId, PinMode};
use crate::typelevel::{NoneT, Sealed};

#[hal_module(
    any("sercom0-d11", "sercom0-d21") => "pad/impl_pad_thumbv6m.rs",
    "sercom0-d5x" => "pad/impl_pad_thumbv7em.rs",
)]
mod impl_pad {}

//==============================================================================
// PadNum
//==============================================================================

/// Type-level enum representing a SERCOM pad number
///
/// It has variants [`Pad0`], [`Pad1`], [`Pad2`] & [`Pad3`]. See the [type-level
/// enum] documentation for an explanation of the pattern.
///
/// [type-level enum]: crate::typelevel#type-level-enum
pub trait PadNum: Sealed {}

seq!(N in 0..=3 {
    paste! {
        #[doc = "Type-level variant of [`PadNum`] representing SERCOM pad " N]
        ///
        /// See the [type-level enum] documentation for an explanation of the
        /// pattern.
        ///
        /// [type-level enum]: crate::typelevel#type-level-enum
        pub enum Pad~N {}
        impl Sealed for Pad~N {}
        impl PadNum for Pad~N {}
    }
});

//==============================================================================
// OptionalPadNum
//==============================================================================

/// Type-level equivalent of `Option<PadNum>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait OptionalPadNum: Sealed {}

impl OptionalPadNum for NoneT {}

impl<N: PadNum> OptionalPadNum for N {}

//==============================================================================
// IsPad
//==============================================================================

/// Type class for [`Pin`]s configured as SERCOM pads
///
/// This trait serves as both a [type class] for `Pin`s configured as SERCOM
/// pads and as a [type-level function] mapping each `Pin` type to its
/// corresponding [`Sercom`] and [`PadNum`].
///
/// [type class]: crate::typelevel#type-classes
/// [type-level function]: crate::typelevel#type-level-functions
pub trait IsPad: AnyPin {
    type Sercom: Sercom;
    type PadNum: PadNum;
}

//==============================================================================
// IsI2cPad
//==============================================================================

/// Type class for [`Pin`]s which can be used as I2C pads
///
/// This trait serves as a [type class] for `Pin`s configured as I2C pads.
///
/// [type class]: crate::typelevel#type-classes
pub trait IsI2cPad: IsPad {}

//==============================================================================
// OptionalPad
//==============================================================================

/// Type-level equivalent of `Option<Pad>`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait OptionalPad: OptionalPin {
    type PadNum: OptionalPadNum;
}

impl OptionalPad for NoneT {
    type PadNum = NoneT;
}

impl<P: IsPad> OptionalPad for P {
    type PadNum = P::PadNum;
}

/// Type-level equivalent of `Some(Pad)`
///
/// See the [`OptionalKind`] documentation for more details on the pattern.
///
/// [`OptionalKind`]: crate::typelevel#optionalkind-trait-pattern
pub trait SomePad: IsPad {}

impl<P: IsPad> SomePad for P {}

//==============================================================================
// GetPad
//==============================================================================

/// Type-level function mapping [`PinId`]s to SERCOM-pad-related types
///
/// For SAMD21 and SAMx5x chips, a [`Sercom`] and a [`PinId`] is enough
/// information to uniquely identify a pad, so this trait returns the
/// corresponding [`PadNum`] and [`PinMode`].
///
/// For SAMD11 chips, on the other hand, some `PinId`s can serve as two
/// different `PadNum`s for the *same* `Sercom`. For these chips, `GetPad`
/// requires a second type parameter to specify the `PadNum` and only returns
/// the `PinMode`.
///
/// See the documentation on [type-level functions] for more details.
///
/// [type-level functions]: crate::typelevel#type-level-functions
#[hal_cfg("sercom0-d11")]
pub trait GetPad<S, N>
where
    S: Sercom,
    N: PadNum,
    Self: PinId,
{
    type PinMode: PinMode;
}

/// Type-level function mapping [`PinId`]s to SERCOM-pad-related types
///
/// For SAMD21 and SAMx5x chips, a [`Sercom`] and a [`PinId`] is enough
/// information to uniquely identify a pad, so this trait returns the
/// corresponding [`PadNum`] and [`PinMode`].
///
/// For SAMD11 chips, on the other hand, some `PinId`s can serve as two
/// different `PadNum`s for the *same* `Sercom`. For these chips, `GetPad`
/// requires a second type parameter to specify the `PadNum` and only returns
/// the `PinMode`.
///
/// See the documentation on [type-level functions] for more details.
///
/// [type-level functions]: crate::typelevel#type-level-functions
#[hal_cfg(any("sercom0-d21", "sercom0-d5x"))]
pub trait GetPad<S>
where
    S: Sercom,
    Self: PinId,
{
    type PadNum: PadNum;
    type PinMode: PinMode;
}

//==============================================================================
// GetPad aliases
//==============================================================================

/// Type alias using [`GetPad`] to recover the [`PinMode`] for a given SERCOM
/// pad
#[hal_cfg("sercom0-d11")]
pub type PadMode<S, N, I> = <I as GetPad<S, N>>::PinMode;

/// Type alias using [`GetPad`] to recover the [`PinMode`] for a given SERCOM
/// pad
#[hal_cfg(any("sercom0-d21", "sercom0-d5x"))]
pub type PadMode<S, I> = <I as GetPad<S>>::PinMode;

/// Type alias to recover a [`Pin`] configured as a SERCOM pad in the correct
/// [`PadMode`]
#[hal_cfg("sercom0-d11")]
pub type Pad<S, N, I> = Pin<I, PadMode<S, N, I>>;

/// Type alias to recover a [`Pin`] configured as a SERCOM pad in the correct
/// [`PadMode`]
#[hal_cfg(any("sercom0-d21", "sercom0-d5x"))]
pub type Pad<S, I> = Pin<I, PadMode<S, I>>;

//==============================================================================
// GetOptionalPad
//==============================================================================

/// Type-level function mapping [`OptionalPinId`]s to their corresponding
/// [`OptionalPad`]s
///
/// This trait acts as a [type-level function] mapping `OptionalPinId`s to their
/// corresponding `OptionalPad`. In pseudo-Rust, it is the type-level equivalent
/// of starting with `Option<PinId>` and calling `.map(GetPad)` to recover an
/// `Option<Pad>`.
///
/// [type-level functions]: crate::typelevel#type-level-functions
#[hal_cfg(any("sercom0-d21", "sercom0-d5x"))]
pub trait GetOptionalPad<S: Sercom>: OptionalPinId {
    type PadNum: OptionalPadNum;
    type Pad: OptionalPad;
}

#[hal_cfg(any("sercom0-d21", "sercom0-d5x"))]
impl<S: Sercom> GetOptionalPad<S> for NoneT {
    type PadNum = NoneT;
    type Pad = NoneT;
}

#[hal_cfg(any("sercom0-d21", "sercom0-d5x"))]
impl<S, I> GetOptionalPad<S> for I
where
    S: Sercom,
    I: PinId + GetPad<S>,
    Pad<S, I>: IsPad,
{
    type PadNum = I::PadNum;
    type Pad = Pad<S, I>;
}

//==============================================================================
// IoSet
//==============================================================================

#[hal_cfg("sercom0-d5x")]
mod ioset {

    use super::*;
    use sorted_hlist::{HList, Intersect, NonEmptyHList, mk_hlist};

    /// Type-level enum representing a SERCOM IOSET
    ///
    /// The SAM D5x/E5x has particular sets of [`Pin`]s that are allowed to be
    /// used together for each [`Sercom`], and `Pin`s from different sets cannot
    /// be used together.  The valid combinations of `Pin`s are called IOSETs
    /// (or IO SETs) in the datasheet.  Additionally, some undocumented sets are
    /// used in commercially available boards (such as Adafruit's PyGamer and
    /// Feather M4).  This `IoSet` trait is used to constrain the various `Pads`
    /// types ([`spi::Pads`], [`uart::Pads`], and [`i2c::Pads`]) to only contain
    /// valid documented or undocumented sets of `Pin`s.
    ///
    /// See the [type-level enum] documentation for more details on the pattern.
    /// Typenum unsigned integers are used to make IoSets comparable
    ///
    /// [`spi::Pads`]: crate::sercom::spi::Pads
    /// [`uart::Pads`]: crate::sercom::uart::Pads
    /// [`i2c::Pads`]: crate::sercom::i2c::Pads
    /// [type-level enum]: crate::typelevel#type-level-enum
    pub trait IoSet: Sealed {
        type Order;
    }

    seq!(N in 1..=6 {
        paste! {
            #[doc = "Type-level variant of [`IoSet`] representing SERCOM IOSET " N]
            ///
            /// See the [type-level enum] documentation for more details on the
            /// pattern.
            ///
            /// [type-level enum]: crate::typelevel#type-level-enum
            pub enum IoSet~N {}
            impl Sealed for IoSet~N {}
            impl IoSet for IoSet~N {
                type Order = typenum::U~N;
            }
        }
    });

    // Implement IoSets for NoneT, making it act as a wildcard.
    seq!(N in 1..=6 {
        impl IoSets for NoneT {
            type SetList = mk_hlist! ( #(<IoSet~N as IoSet>::Order, )* <UndocIoSet1 as IoSet>::Order, <UndocIoSet2 as IoSet>::Order );
        }
    });

    /// Type-level variant of [`IoSet`] representing an undocumented SERCOM
    /// IOSET
    ///
    /// After implementing `IoSet` type checking, it became clear that some
    /// existing boards were using a combinations of pins that did not match any
    /// IOSET in the datasheet. From that, we infer that there must be at least
    /// two undocumented IOSETs, and we added these new `IoSet`s to account for
    /// it.
    ///
    /// As of writing this documentation, only two undocumented IOSETs have been
    /// discovered:
    /// - [`UndocIoSet1`]: PA16, PA17, PB22 & PB23 configured for `Sercom1`.
    ///   Both the pygamer & feather_m4 uses this combination.
    /// - [`UndocIoSet2`]: PA00, PA01, PB22 & PB23 configured for `Sercom1`. The
    ///   itsybitsy_m4 uses this combination.
    ///
    /// See the [type-level enum] documentation for more details on type-level
    /// variants.
    ///
    /// [type-level enum]: crate::typelevel#type-level-enum
    pub enum UndocIoSet1 {}
    impl Sealed for UndocIoSet1 {}
    impl IoSet for UndocIoSet1 {
        type Order = typenum::U8;
    }

    /// Type-level variant of [`IoSet`] representing an undocumented SERCOM
    /// IOSET
    ///
    /// After implementing `IoSet` type checking, it became clear that some
    /// existing boards were using a combinations of pins that did not match any
    /// IOSET in the datasheet. From that, we infer that there must be at least
    /// two undocumented IOSETs, and we added these new `IoSet`s to account for
    /// it.
    ///
    /// As of writing this documentation, only two undocumented IOSETs have been
    /// discovered:
    /// - [`UndocIoSet1`]: PA16, PA17, PB22 & PB23 configured for `Sercom1`.
    ///   Both the pygamer & feather_m4 uses this combination.
    /// - [`UndocIoSet2`]: PA00, PA01, PB22 & PB23 configured for `Sercom1`. The
    ///   itsybitsy_m4 uses this combination.
    ///
    /// See the [type-level enum] documentation for more details on type-level
    /// variants.
    ///
    /// [type-level enum]: crate::typelevel#type-level-enum
    pub enum UndocIoSet2 {}
    impl Sealed for UndocIoSet2 {}
    impl IoSet for UndocIoSet2 {
        type Order = typenum::U9;
    }

    /// Type class for SERCOM pads in a given [`IoSet`]
    ///
    /// This trait is used to label each [`Pin`] implementing [`IsPad`] with its
    /// corresponding [`IoSet`]\(s). Downstream types can use this trait as a
    /// [type class] to restrict [`Pin`]s to a given [`IoSet`]. See the [type
    /// class] documentation for more details on the pattern.
    ///
    /// [type class]: crate::typelevel#type-classes
    pub trait InIoSet<I>
    where
        Self: IsPad,
        I: IoSet,
    {
    }

    /// Type class for corresponding IoSet indices for OptionalPads, NoneT
    /// serves as a wildcard
    pub trait IoSets: OptionalPad {
        type SetList: HList;
    }

    /// Type class for accessing the intersection of IoSets of OptionalPads
    /// Currently implemented for tuples of 2 and 4 elements
    pub trait CommonIoSets {
        type IoSets: HList;
    }

    impl<P0: IoSets, P1: IoSets> CommonIoSets for (P0, P1)
    where
        P0::SetList: Intersect<P1::SetList>,
    {
        type IoSets = <P0::SetList as Intersect<P1::SetList>>::Output;
    }

    impl<P0: IoSets, P1: IoSets, P2: IoSets, P3: IoSets> CommonIoSets for (P0, P1, P2, P3)
    where
        P0::SetList: Intersect<P1::SetList>,
        <P0::SetList as Intersect<P1::SetList>>::Output: Intersect<P2::SetList>,
        <<P0::SetList as Intersect<P1::SetList>>::Output as Intersect<P2::SetList>>::Output:
            Intersect<P3::SetList>,
    {
        type IoSets = <<<P0::SetList as Intersect<P1::SetList>>::Output as Intersect<
            P2::SetList,
        >>::Output as Intersect<P3::SetList>>::Output;
    }

    /// Shortcut trait for Pad tuples that share at least one IoSet
    pub trait ShareIoSet {}
    impl<A> ShareIoSet for A
    where
        A: CommonIoSets,
        <A as CommonIoSets>::IoSets: NonEmptyHList,
    {
    }
}

#[hal_cfg("sercom0-d5x")]
pub use ioset::*;

#[cfg(doc)]
#[hal_cfg(not("sercom0-d5x"))]
/// This trait is not present with the selected feature set, defined for
/// documentation only
pub trait IoSet {}

#[cfg(doc)]
#[hal_cfg(not("sercom0-d5x"))]
/// This trait is not present with the selected feature set, defined for
/// documentation only
pub trait InIoSet {}

#[cfg(doc)]
#[hal_cfg(not("sercom0-d5x"))]
/// This type is not present with the selected feature set, defined for
/// documentation only
pub enum UndocIoSet1 {}

#[cfg(doc)]
#[hal_cfg(not("sercom0-d5x"))]
/// This type is not present with the selected feature set, defined for
/// documentation only
pub enum UndocIoSet2 {}

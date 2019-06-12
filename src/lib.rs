//! An implementation of Distance-Based Amplitude Panning as published by Trond Lossius, 2009.

use num_traits::Pow;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Scalar values compatible with the DBAP algorithm, used to represent distances, coefficients,
/// weights, etc.
///
/// The purpose of this trait is to allow the DBAP algorithm to be generic over the types of values
/// used (e.g. `f32`, `f64`).
pub trait Scalar:
    Sized
    + Copy
    + From<DefaultScalar>
    + PartialEq
    + Add<Self, Output = Self>
    + Div<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Neg<Output = Self>
    + Pow<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Sum<Self>
{
}

impl<T> Scalar for T where
    T: Sized
        + Copy
        + From<DefaultScalar>
        + PartialEq
        + Add<Self, Output = Self>
        + Div<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Neg<Output = Self>
        + Pow<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Sum<Self>
{
}

/// The default scalar type used to represent the space.
pub type DefaultScalar = f32;

/// A speaker within the DBAP space calculation.
#[derive(Copy, Clone, Debug)]
pub struct Speaker<S = DefaultScalar> {
    /// The speaker's distance from the virtual location.
    pub distance: S,
    /// The weight applied to the speaker, compared to all other speakers.
    pub weight: S,
}

/// An iterator yielding the gain for each given speaker, given their weights and distance from the
/// source position.
#[derive(Clone)]
pub struct SpeakerGains<'a, S = DefaultScalar> {
    speakers: &'a [Speaker<S>],
    a_coefficient: S,
    k_coefficient: S,
    i: usize,
}

impl<'a, S> SpeakerGains<'a, S>
where
    S: Scalar,
{
    /// Given:
    ///
    /// - a list of speaker distances from the virtual source:
    /// - weights for each of those speakers and
    /// - some decibell rolloff
    ///
    /// produce an iterator that returns the gain for each speaker given the source as an input.
    pub fn new(speakers: &'a [Speaker<S>], rolloff_db: S) -> Self {
        assert!(speakers.len() > 0);
        let a_coefficient = a_coefficient(rolloff_db);
        let k_coefficient = k_coefficient(a_coefficient, speakers);
        SpeakerGains {
            speakers,
            a_coefficient,
            k_coefficient,
            i: 0,
        }
    }
}

impl<'a, S> Iterator for SpeakerGains<'a, S>
where
    S: Scalar,
{
    type Item = S;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        if i >= self.speakers.len() {
            return None;
        }
        self.i += 1;
        let s = &self.speakers[i];
        let s_r_amp = v_speaker_relative_amplitude(s, self.k_coefficient, self.a_coefficient);
        Some(s_r_amp / s.distance)
    }
}

/// The same as a regular *distance* function but applies a subtle `blur` amount.
///
/// From the paper: "In 2D space, blur can be understood as a vertical displacement between source
/// and speakers. The larger ` gets, the less the source will be able to gravitate towards one
/// speaker only."
///
/// A non-zero blur will ensure that the distance is greater than `0.0` and that we never divide by 0.0.
pub fn blurred_distance_2<S>(source: [S; 2], speaker: [S; 2], blur: S) -> S
where
    S: Scalar,
{
    let x = speaker[0] - source[0];
    let y = speaker[1] - source[1];
    x * x + y * y + blur * blur
}

/// The relative amplitude for a speaker where:
///
/// - `k` is a coefficient depending on the position of the source and all speakers
/// - `a` is a coefficient calculated from the rolloff in decibels per doubling distance.
///
/// The speaker's `distance` field must be greater than zero or the result will be NaN.
pub fn v_speaker_relative_amplitude<S>(speaker: &Speaker<S>, k: S, a: S) -> S
where
    S: Scalar,
{
    k * speaker.weight / ((speaker.distance + speaker.distance) * a)
}

/// A coefficient calculated from the rolloff `r` in decibels per doubling of distance.
///
/// A rolloff of 6dB equals the inverse distance law for sound propagataing in a free field.
///
/// For closed or semi-closed environments `r` will generally be lower, in the range 3-5dB, and
/// depend on reflections and reverberation.
pub fn a_coefficient<S>(rolloff_db: S) -> S
where
    S: Scalar,
{
    S::from(10f32).pow(-rolloff_db / S::from(20.0))
}

/// `k` is a coefficient depending on the position of the source and all speakers.
///
/// Returns `0.0` if all speakers had a weight or distance of `0.0`.
///
/// **Panics** if there were no speakers in the list.
pub fn k_coefficient<S>(a: S, speakers: &[Speaker<S>]) -> S
where
    S: Scalar,
{
    let zero = S::from(0f32);
    let sum = speakers
        .iter()
        .map(|s| {
            if s.distance == zero {
                return zero;
            }
            let w2 = s.weight * s.weight;
            let d2 = s.distance * s.distance;
            w2 / d2
        })
        .sum();
    if sum == zero {
        zero
    } else {
        S::from(2.0) * a / sum
    }
}

#[test]
fn speaker_gains() {
    fn magnitude2<S>([x, y]: [S; 2]) -> S
    where
        S: Copy + Add<S, Output = S> + Mul<S, Output = S>,
    {
        x * x + y * y
    }

    fn distance2<S>([ax, ay]: [S; 2], [bx, by]: [S; 2]) -> S
    where
        S: Copy + Add<S, Output = S> + Mul<S, Output = S> + Sub<S, Output = S>,
    {
        magnitude2([bx - ax, by - ay])
    }

    let src = [5f64, 5.0];
    let speaker = |v: [f64; 2], w| Speaker {
        distance: distance2(v, src).sqrt(),
        weight: w,
    };
    let a = speaker([0.0, 0.0], 1.0);
    let b = speaker([10.0, 0.0], 1.0);
    let c = speaker([10.0, 10.0], 1.0);
    let d = speaker([0.0, 10.0], 1.0);
    let spkrs = vec![a, b, c, d];
    let r = 6.0; // free-field rolloff db.
    let gains = SpeakerGains::new(&spkrs, r).collect::<Vec<_>>();
    let g = gains[0];
    for gain in gains {
        assert_eq!(g, gain);
    }
}

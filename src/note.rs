/// A music note, either falling within the 24-note quarter-tone scale or the
/// 22-tone Grama system.  There is an option for custom hz.  The `i8` parameter
/// denote the octave (`Note::C(4)` is middle C).
pub enum Note<'a> {
	/// Shruti S’[Chandovatī] Just: 1/1 of C's hz - C / B# or  or ji or
	/// KABA ÇÂRGÂH (261.6256hz)
	C(i8),
	/// Just: 28/27 of C's hz - Quarter Tone (Kaba Nim Hicaz - 4 Commas from
	/// C)
	CCd(i8),
	/// Shruti r1 [Dayāvatī]: 256/243 of C's Hz
	Sr1(i8),
	/// Shruti r2 [Ranjanī]: Just: 16/15 of C's hz - C# or Db (Kaba Hicaz -
	/// 5 Commas from C)
	Cd(i8),
	/// Just: 12/11 or 11/10 Lesser or Greater Neutral Second Quarter Tone
	/// (Kaba Dik Hicaz - 8 Commas from C)
	CdD(i8),
	/// Shruti R1 [Raktikā]: 10/9 of C's Hz
	SR1(i8),
	/// Shruti R2 [Raudrī] / Just: 9/8 of C's hz - D (YEGÂH - 9 Commas from
	/// C)
	D(i8),
	/// Just: 15/13 of C's hz - Tridecimal Five Quarter Tone (Kaba Nim Hisar
	/// - 13 Commas from C)
	DDe(i8),
	/// Shruti g1 [Krodhā]: 32/27 of C's Hz
	Sg1(i8),
	/// Shruti g2 [Vajrikā] / Just: 6/5 of C's hz - D# or Eb (Kaba Hisar -
	/// 14 Commas from C)
	De(i8),
	/// Just: 11/9 of C's hz neutral 3rd Quarter Tone (Kaba Dik Hisar - 17
	/// Commas from C)
	DeE(i8),
	/// Shruti G1 [Prasāriṇī] / Just: 5/4 of C's hz - E or Fb (HÜSEYNÎ
	/// AŞÎRÂN - 18 Commas from C)
	E(i8),
	/// Shruti G2 [Prīti]: 81/64 of C's Hz
	SG2(i8),
	/// Just: 13/10 or 9/7 tridecimal/septimal major 3rd Quarter Tone
	/// (ACEM AŞÎRÂN - 22 Commas from C)
	Ef(i8),
	/// Shruti M1 [Mārjanī] / Just: 4/3 of C's hz - F or E# (Dik Acem AŞiran
	/// - 23 Commas from C)
	F(i8),
	/// Shruti M2 [Kṣhiti]: 27/20 of C's Hz
	SM2(i8),
	/// Just: 11/8 Major 4th Quarter Tone (Irak - 26 Commas from C)
	FFg(i8),
	/// Just: 7/5 of C's hz - F# or Gb (Gevest - 27 Commas from C)
	Fg(i8),
	/// Shruti m1 [Raktā]: 45/32 of C's Hz
	Sm1(i8),
	/// Shruti m2 [Sandīpanī]: 729/512 of C's Hz
	Sm2(i8),
	/// Just: 16/11 of C's hz Minor 5th Quarter Tone (Dik Gevest - 30 Commas
	/// from C)
	FgG(i8),
	/// Shruti P [Ālāpinī] / Just: 3/2 of C's hz - G (RAST - 31 Commas from
	/// C)
	G(i8),
	/// Just: 14/9 Subminor 6th Quarter Tone (Nim Zirgüle - 35 Commas from C)
	GGa(i8),
	/// Just: 8/5 of C's hz - G# or Ab (Zirgüle - 36 Commas from C)
	Ga(i8),
	/// Just: 18/11 Neutral 6th Quarter Tone (Dik Zirgüle - 39 Commas from C)
	GaA(i8),
	/// Just: 5/3 of C's hz - A (DÜGÂH - 40 Commas from C)
	A(i8),
	/// Just: 7/4 of C's hz Supermajor 6th / Subminor 7th Quarter Tone
	/// (Kürdi - 44 Commas from C)
	AAb(i8),
	/// Just: 16/9 of C's hz - A# or Bb (Dik Kürdi - 45 Commas from C)
	Ab(i8),
	/// Just: 11/6 or 20/11 minor neutral 7th or major neutral 7th Quarter
	/// Tone (Segah - 48 Commas from C)
	AbB(i8),
	/// Just: 15/8 of C's hz - B or Cb (BÛSELIK - 49 Commas from C)
	B(i8),
	/// Just: 35/18 or 27/14 semidiminished octave or supermajor seventh
	/// Quarter Tone (Dik Bûselik - 52 Commas from C)
	Bc(i8),



	/// 128/81 of C's Hz - Shruti d1 [Madantī], Flat G# or Ab
	Sd1(i8),
	/// 8/5 of C's Hz - Shruti d2 [Rohiṇī], Sharp G# or Ab
	Sd2(i8),
	/// 5/3 of C's Hz - Shruti D1 [Ramyā], Flat A
	SD1(i8),
	/// 27/16 of C's Hz - Shruti D2 [Ugrā], Sharp A
	SD2(i8),
	/// 16/9 of C's Hz - Shruti n1 [Kṣobhinī], Flat A# or Bb
	Sn1(i8),
	/// 9/5 of C's Hz - Shruti n2 [Tīvrā], Sharp A# or Bb
	Sn2(i8),
	/// 15/8 of C's Hz - Shruti N1 [Kumudvatī], Flat B or Cb
	SN1(i8),
	/// 243/128 of C's Hz - Shruti N2 [Mandā], Sharp B or Cb
	SN2(i8),

	/// A Pramana interval (22 cents) above a note
	Pramana(&'a Note<'a>),
	/// A Nyuna interval (70 cents) above a note
	Nyuna(&'a Note<'a>),
	/// A Puruna interval (90 cents) above a note
	Purana(&'a Note<'a>),

	/// 12 hz above note, add to other notes to make slightly out of tune,
	/// if played together in a chord, you will here a pulsating beat called
	/// (interference beating).
	Beat(&'a Note<'a>),

	/// Splits perfect fifth into 9 equal parts (20th root of 3/2 above
	/// note)
	Alpha(&'a Note<'a>),
	/// Splits perfect fifth into 11 equal parts (20th root of 3/2 above
	/// note)
	Beta(&'a Note<'a>),
	/// Splits perfect fifth into 20 equal parts (20th root of 3/2 above
	/// note)
	Gamma(&'a Note<'a>),
	/// Splits minor second into 8 equal parts (8th root of 16/15 above
	/// note)
	Delta(&'a Note<'a>),

	/// A Turkish Comma (53rd root of 2 above note)
	Comma(&'a Note<'a>),

	// Pelog, Indonesian Notes (Note: Pelog in key of D, but written as C
	// for the tonic for simplicity ) Subset of 9-tone equal temperament
	/// Db +, 
	Pro(i8), 
	/// Eb -,
	Plu(i8),
	/// Gb -,
	Ppat(i8),
	/// G,
	Pma(i8),
	/// Ab,
	Pnem(i8),
	/// Bb +,
	Ppi(i8),

	// Slendro, pentatonic subset of 17 tone equal temperament
	/// D +,
	Sro(i8),
	/// F (-)
	Slu(i8),
	/// G (+)
	Sma(i8),
	/// A +,
	Snam(i8),

	/// Custom Hertz
	Hz(f32, i8),
}

pub enum Temperament {
	/// Equal Temperament (12th root of 2 = 100 cents away)
	Equal,
	/// Tuned specifically for one key (no key modulation)
	Just,
	/// Alternative Just intonation for quarter tones
	AltJust,
	/// 5th altered to improve the sound of the third
	Meantone,
	/// Turkish Tuning (by commas)
	Makam,
}

//	E2, // Ledger below bass clef
//	A5, // a above treble

// Indian Microtonal Scales
// Gorakh Kalyan: R1, M1, P, D1, n1, S’
// Abhogi: R1, g1, M1, D1, S’
// Bageshri: R1, g1, M1, P, D1, n1, S’
// Dhani: g1, M1, P, n1, S’
// Yaman: R2, G1, M1, m1, P, D2, N1, S’
// Ahir Bhairav: r2, G1, M1, P, D1, n1, S’
// Puriya: r1, G1, m1, D1, N1, S’
// Malkaush: g1, M1, d1, n1, S’
// Keeravani: R2, g1, M1, P, d1, N1
// Bibhas: r1, G1, P, d1, S’
// Bhupal Todi: r1, g1, P, d1, S’

// Balinese modes
// Selisir: 1, 2, 3, 5, 6

// Javanese & Balinese modes
// Bem / Tembung: 1, 2, 4, 5, 6
// Barang / Sunaren: 2, 3, 5, 6, 7

// Turkish Scales - [] means tetra- or penta- chord on that note that continues
//	until the middle of the next bracket'd note
// Çârgâh makam (delinquency scale): B [C] D E F [G] A B [C]
// Bûselik makam (minor): G# [A] B C D [E] F G(#) [A] G F(#) E(½b) D C B A G#
// Rast makam (major): F# [G] A B(½b) C [D] E F# [G]
// Rast makam (alternative development): D E F# [G] A B(½b) C [D]
// Uşşâk makam (call to prayer): G [A] B(½b) C [D] E F G [A]
// Uşşâk makam (alternative): D E F# G [A] B(½b) C [D]

'''
MIT License

Copyright (c) 2025 Roy Vegard Ovesen & Christian Hågenvik

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
'''


import pyaga8
from pytest import approx, raises


gerg = pyaga8.Gerg2008()
detail = pyaga8.Detail()

FULL_COMPOSITION = pyaga8.Composition()
FULL_COMPOSITION.methane = 0.778_24
FULL_COMPOSITION.nitrogen = 0.02
FULL_COMPOSITION.carbon_dioxide = 0.06
FULL_COMPOSITION.ethane = 0.08
FULL_COMPOSITION.propane = 0.03
FULL_COMPOSITION.isobutane = 0.001_5
FULL_COMPOSITION.n_butane = 0.003
FULL_COMPOSITION.isopentane = 0.000_5
FULL_COMPOSITION.n_pentane = 0.001_65
FULL_COMPOSITION.hexane = 0.002_15
FULL_COMPOSITION.heptane = 0.000_88
FULL_COMPOSITION.octane = 0.000_24
FULL_COMPOSITION.nonane = 0.000_15
FULL_COMPOSITION.decane = 0.000_09
FULL_COMPOSITION.hydrogen = 0.004
FULL_COMPOSITION.oxygen = 0.005
FULL_COMPOSITION.carbon_monoxide = 0.002
FULL_COMPOSITION.water = 0.000_1
FULL_COMPOSITION.hydrogen_sulfide = 0.002_5
FULL_COMPOSITION.helium = 0.007
FULL_COMPOSITION.argon = 0.001

def test_gerg2008_set_composition():
    gerg.set_composition(FULL_COMPOSITION)

    invalid = pyaga8.Composition()
    with raises(ValueError, match='composition cannot be empty'):
        gerg.set_composition(invalid)

    invalid.ethane = 0.3
    with raises(ValueError, match='composition sum must be within 0.01 of 1.0'):
        gerg.set_composition(invalid)

def test_gerg2008_calc_density():
    gerg.set_composition(FULL_COMPOSITION)
    gerg.temperature = 400.0
    gerg.pressure = 50_000.0

    gerg.calc_density(0)
    assert gerg.d == approx(12.798_286_260_820_62)


def test_gerg2008_calc_density_low_pressure():
    gerg.set_composition(FULL_COMPOSITION)
    gerg.temperature = 400.0
    gerg.pressure = 0.0

    with raises(RuntimeError, match='density calculation failed to converge'):
        gerg.calc_density(0)


def test_gerg2008_calc_density_rejects_non_integer_flag():
    gerg.set_composition(FULL_COMPOSITION)
    gerg.temperature = 400.0
    gerg.pressure = 50_000.0

    with raises(TypeError):
        gerg.calc_density('spam')

def test_gerg2008_calc_pressure():
    gerg.set_composition(FULL_COMPOSITION)
    gerg.temperature = 18.0 + 273.15
    gerg.d = 7.558_334

    assert gerg.calc_pressure() == approx(13_050.037_472_144)

def test_gerg2008_calc_properties():
    gerg.set_composition(FULL_COMPOSITION)
    gerg.temperature = 400.0
    gerg.pressure = 50_000.0

    gerg.calc_density(0)
    gerg.calc_properties()

    assert gerg.d == approx(12.798_286_260_820_62)
    assert gerg.mm == approx(20.542_744_501_6)
    assert gerg.z == approx(1.174_690_666_383_717)
    assert gerg.dp_dd == approx(7_000.694_030_193_327)
    assert gerg.d2p_dd2 == approx(1_129.526_655_214_841)
    assert gerg.d2p_dtd == approx(34.309_428_436_792_17)
    assert gerg.dp_dt == approx(235.983_229_259_309_6)
    assert gerg.u == approx(-2_746.492_901_212_53)
    assert gerg.h == approx(1_160.280_160_510_973)
    assert gerg.s == approx(-38.575_903_924_090_89)
    assert gerg.cv == approx(39.029_482_181_563_72)
    assert gerg.cp == approx(58.455_220_510_003_66)
    assert gerg.w == approx(714.424_884_059_602_4)
    assert gerg.g == approx(16_590.641_730_147_33)
    assert gerg.jt == approx(7.155_629_581_480_913E-5)
    assert gerg.kappa == approx(2.683_820_255_058_032)

def test_gerg2008_calc_molar_mass():
    gerg.set_composition(FULL_COMPOSITION)
    gerg.calc_molar_mass()

    assert gerg.mm == approx(20.542_744_501_6)

def test_gerg2008_set_pressure():
    gerg.pressure = 42.14
    assert gerg.pressure == approx(42.14)

    with raises(TypeError):
        gerg.pressure = 'spam'

def test_gerg2008_set_temperature():
    gerg.temperature = 42.14
    assert gerg.temperature == approx(42.14)

    with raises(TypeError):
        gerg.temperature = 'spam'

def test_gerg2008_set_d():
    gerg.d = 6.42
    assert gerg.d == approx(6.42)

    with raises(TypeError):
        gerg.d = 'spam'


def test_detail_set_composition():
    detail.set_composition(FULL_COMPOSITION)

    invalid = pyaga8.Composition()
    with raises(ValueError, match='composition cannot be empty'):
        detail.set_composition(invalid)

    invalid.ethane = 0.3
    with raises(ValueError, match='composition sum must be within 0.01 of 1.0'):
        detail.set_composition(invalid)

def test_detail2008_calc_density():
    detail.set_composition(FULL_COMPOSITION)
    detail.temperature = 400.0
    detail.pressure = 50_000.0

    detail.calc_density()
    assert detail.d == approx(12.807_924_036_488_01)


def test_detail2008_calc_density_low_pressure():
    detail.set_composition(FULL_COMPOSITION)
    detail.temperature = 400.0
    detail.pressure = 0.0

    with raises(ValueError, match='pressure is too low for density calculation'):
        detail.calc_density()

def test_detail2008_calc_pressure():
    detail.set_composition(FULL_COMPOSITION)
    detail.temperature = 18.0 + 273.15
    detail.d = 7.558_334

    assert detail.calc_pressure() == approx(13_067.068_161_509_907)

def test_detail2008_calc_properties():
    detail.set_composition(FULL_COMPOSITION)
    detail.temperature = 400.0
    detail.pressure = 50_000.0

    detail.calc_density()
    detail.calc_properties()

    assert detail.d == approx(12.807_924_036_488_01)
    assert detail.mm == approx(20.543_330_51)
    assert detail.z == approx(1.173_801_364_147_326)
    assert detail.dp_dd == approx(6_971.387_690_924_09)
    assert detail.d2p_dd2 == approx(1_118.803_636_639_52)
    assert detail.d2p_dtd == approx(0.0)
    assert detail.dp_dt == approx(235.664_149_306_821_2)
    assert detail.u == approx(-2_739.134_175_817_231)
    assert detail.h == approx(1_164.699_096_269_404)
    assert detail.s == approx(-38.548_826_846_771_11)
    assert detail.cv == approx(39.120_761_544_303_32)
    assert detail.cp == approx(58.546_176_723_806_67)
    assert detail.w == approx(712.639_368_405_790_3)
    assert detail.g == approx(16_584.229_834_977_85)
    assert detail.jt == approx(7.432_969_304_794_577E-5)
    assert detail.kappa == approx(2.672_509_225_184_606)

def test_detail2008_calc_molar_mass():
    detail.set_composition(FULL_COMPOSITION)
    detail.calc_molar_mass()

    assert detail.mm == approx(20.543_330_51)

def test_detail2008_set_pressure():
    detail.pressure = 42.14
    assert detail.pressure == approx(42.14)

    with raises(TypeError):
        detail.pressure = 'spam'

def test_detail2008_set_temperature():
    detail.temperature = 42.14
    assert detail.temperature == approx(42.14)

    with raises(TypeError):
        detail.temperature = 'spam'

def test_detail2008_set_d():
    detail.d = 6.42
    assert detail.d == approx(6.42)

    with raises(TypeError):
        detail.d = 'spam'

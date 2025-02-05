use indexmap::IndexSet;
use oxipng::internal_tests::*;
use oxipng::*;
use std::fs::remove_file;
use std::path::Path;
use std::path::PathBuf;

const RGB: u8 = 2;
const INDEXED: u8 = 3;

fn get_opts(input: &Path) -> (OutFile, oxipng::Options) {
    let mut options = oxipng::Options {
        force: true,
        ..Default::default()
    };
    let mut filter = IndexSet::new();
    filter.insert(RowFilter::None);
    options.filter = filter;

    (
        OutFile::Path(Some(input.with_extension("out.png"))),
        options,
    )
}

fn test_it_converts(
    input: &str,
    interlace: Interlacing,
    color_type_in: u8,
    bit_depth_in: BitDepth,
    color_type_out: u8,
    bit_depth_out: BitDepth,
) {
    let input = PathBuf::from(input);
    let (output, mut opts) = get_opts(&input);
    let png = PngData::new(&input, &opts).unwrap();
    opts.interlace = Some(interlace);
    assert_eq!(png.raw.ihdr.color_type.png_header_code(), color_type_in);
    assert_eq!(png.raw.ihdr.bit_depth, bit_depth_in);
    assert_eq!(
        png.raw.ihdr.interlaced,
        if interlace == Interlacing::Adam7 {
            Interlacing::None
        } else {
            Interlacing::Adam7
        }
    );

    match oxipng::optimize(&InFile::Path(input), &output, &opts) {
        Ok(_) => (),
        Err(x) => panic!("{}", x),
    };
    let output = output.path().unwrap();
    assert!(output.exists());

    let png = match PngData::new(output, &opts) {
        Ok(x) => x,
        Err(x) => {
            remove_file(output).ok();
            panic!("{}", x)
        }
    };

    assert_eq!(png.raw.ihdr.color_type.png_header_code(), color_type_out);
    assert_eq!(png.raw.ihdr.bit_depth, bit_depth_out);

    remove_file(output).ok();
}

#[test]
fn deinterlace_rgb_16() {
    test_it_converts(
        "tests/files/interlaced_rgb_16_should_be_rgb_16.png",
        Interlacing::None,
        RGB,
        BitDepth::Sixteen,
        RGB,
        BitDepth::Sixteen,
    );
}

#[test]
fn deinterlace_rgb_8() {
    test_it_converts(
        "tests/files/interlaced_rgb_8_should_be_rgb_8.png",
        Interlacing::None,
        RGB,
        BitDepth::Eight,
        RGB,
        BitDepth::Eight,
    );
}

#[test]
fn deinterlace_palette_8() {
    test_it_converts(
        "tests/files/interlaced_palette_8_should_be_palette_8.png",
        Interlacing::None,
        INDEXED,
        BitDepth::Eight,
        INDEXED,
        BitDepth::Eight,
    );
}

#[test]
fn deinterlace_palette_4() {
    test_it_converts(
        "tests/files/interlaced_palette_4_should_be_palette_4.png",
        Interlacing::None,
        INDEXED,
        BitDepth::Four,
        INDEXED,
        BitDepth::Four,
    );
}

#[test]
fn deinterlace_palette_2() {
    test_it_converts(
        "tests/files/interlaced_palette_2_should_be_palette_2.png",
        Interlacing::None,
        INDEXED,
        BitDepth::Two,
        INDEXED,
        BitDepth::Two,
    );
}

#[test]
fn deinterlace_palette_1() {
    test_it_converts(
        "tests/files/interlaced_palette_1_should_be_palette_1.png",
        Interlacing::None,
        INDEXED,
        BitDepth::One,
        INDEXED,
        BitDepth::One,
    );
}

#[test]
fn interlace_rgb_16() {
    test_it_converts(
        "tests/files/rgb_16_should_be_rgb_16.png",
        Interlacing::Adam7,
        RGB,
        BitDepth::Sixteen,
        RGB,
        BitDepth::Sixteen,
    );
}

#[test]
fn interlace_rgb_8() {
    test_it_converts(
        "tests/files/rgb_8_should_be_rgb_8.png",
        Interlacing::Adam7,
        RGB,
        BitDepth::Eight,
        RGB,
        BitDepth::Eight,
    );
}

#[test]
fn interlace_palette_8() {
    test_it_converts(
        "tests/files/palette_8_should_be_palette_8.png",
        Interlacing::Adam7,
        INDEXED,
        BitDepth::Eight,
        INDEXED,
        BitDepth::Eight,
    );
}

#[test]
fn interlace_palette_4() {
    test_it_converts(
        "tests/files/palette_4_should_be_palette_4.png",
        Interlacing::Adam7,
        INDEXED,
        BitDepth::Four,
        INDEXED,
        BitDepth::Four,
    );
}

#[test]
fn interlace_palette_2() {
    test_it_converts(
        "tests/files/palette_2_should_be_palette_2.png",
        Interlacing::Adam7,
        INDEXED,
        BitDepth::Two,
        INDEXED,
        BitDepth::Two,
    );
}

#[test]
fn interlace_palette_1() {
    test_it_converts(
        "tests/files/palette_1_should_be_palette_1.png",
        Interlacing::Adam7,
        INDEXED,
        BitDepth::One,
        INDEXED,
        BitDepth::One,
    );
}

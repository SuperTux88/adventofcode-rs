use super::grid::print_grid;

pub fn print_image(image: &[&[bool]]) {
    print_grid(image, |b| match b {
        false => ' ',
        true => '█',
    });
}

pub fn read_message(image: &[&[bool]]) -> String {
    let char_width = match image.len() {
        6 => 5,
        10 => 6,
        _ => panic!("Unknown char size"),
    };
    let line_length = image[0].len() / char_width;

    let mut message = String::new();
    for i in 0..line_length {
        let char = &image
            .iter()
            .map(|line| &line[i * char_width..(i + 1) * char_width])
            .collect::<Vec<_>>();
        message.push(read_char(char));
    }
    message
}

#[rustfmt::skip]
fn read_char(char: &[&[bool]]) -> char {
    match char {
        [
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false]
        ] => 'A',
        [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false]
        ] => 'B',
        [
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, true, false],
            [false, true, true, false, false]
        ] => 'C',
        [
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false]
        ] => 'E',
        [
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false]
        ] => 'F',
        [
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, false, false],
            [true, false, true, true, false],
            [true, false, false, true, false],
            [false, true, true, true, false]
        ] => 'G',
        [
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false]
        ] => 'H',
        [
            [false, false, true, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false]
        ] => 'J',
        [
            [true, false, false, true, false],
            [true, false, true, false, false],
            [true, true, false, false, false],
            [true, false, true, false, false],
            [true, false, true, false, false],
            [true, false, false, true, false]
        ] => 'K',
        [
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false]
        ] => 'L',
        [
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false]
        ] => 'O',
        [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false]
        ] => 'P',
        [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
            [true, false, true, false, false],
            [true, false, false, true, false]
        ] => 'R',
        [
            [false, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [false, true, true, false, false],
            [false, false, false, true, false],
            [true, true, true, false, false]
        ] => 'S',
        [
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false]
        ] => 'U',
        [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false]
        ] => 'Y',
        [
            [true, true, true, true, false],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false]
        ] => 'Z',
        _ => '?',
    }
}

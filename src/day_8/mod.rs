use super::*;
use std::collections::HashMap;
use std::mem;

pub fn main(part: Part) -> Result<()> {
    let input = read_file("input/day_8")?;
    let input = input.trim_end_matches('\n');

    match part {
        Part::One => println!("{}", part_1(&input)?),
        Part::Two => part_2(&input)?,
    }

    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let layers = build_layers(input)?;

    let layer_with_fewest_zeros = layers
        .iter()
        .min_by_key(|layer| number_of(Pixel::Black, layer))
        .ok_or_else(|| anyhow!("no min layer"))?;

    let answer = number_of(Pixel::White, layer_with_fewest_zeros)
        * number_of(Pixel::Transparent, layer_with_fewest_zeros);

    assert_eq!(1215, answer);

    Ok(answer)
}

fn part_2(input: &str) -> Result<()> {
    let mut layers = build_layers(input)?;
    let mut image = blank_image();

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let pixel = layers
                .iter()
                .map(|layer| &layer[row][col])
                .find(|&&pixel| pixel != Pixel::Transparent)
                .unwrap_or_else(|| &Pixel::Transparent);

            image[row][col] = *pixel;
        }
    }

    let mut html = "<table cellspacing=0>".to_string();
    for row in image {
        html.push_str("<tr>");
        for pixel in row {
            match pixel {
                Pixel::White => html
                    .push_str("<td style='height:3px; width:1px; background-color: white;'></td>"),
                Pixel::Black | Pixel::Transparent => html
                    .push_str("<td style='height:3px; width:1px; background-color: black;'></td>"),
                Pixel::Black => html
                    .push_str("<td style='height:3px; width:1px;'></td>"),
            }
        }
        html.push_str("</tr>");
    }
    html.push_str("</table>");
    println!("<body style='background-color:black'>{}</body>", html);

    Ok(())
}

fn blank_image() -> Image {
    let mut image = Vec::<Row>::with_capacity(HEIGHT);
    for _ in 0..HEIGHT {
        let row = (0..WIDTH).map(|_| Pixel::Transparent).collect::<Row>();
        image.push(row);
    }
    image
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

type Row = Vec<Pixel>;
type Layer = Vec<Row>;
type Image = Vec<Row>;

fn build_layers(input: &str) -> Result<Vec<Layer>> {
    let mut data = input
        .chars()
        .map(parse_char)
        .collect::<Result<Vec<_>>>()?;

    let mut layers = vec![];

    while !data.is_empty() {
        let mut layer = Vec::with_capacity(HEIGHT);
        for _ in 0..HEIGHT {
            let row = data.split_off_keep_tail(WIDTH);
            layer.push(row);
        }
        layers.push(layer);
    }

    Ok(layers)
}

fn number_of(pixel: Pixel, layer: &Layer) -> usize {
    layer
        .iter()
        .map(|row| row.iter().filter(|n| **n == pixel).count())
        .sum::<usize>()
}

#[ext]
impl<T> Vec<T> {
    fn split_off_keep_tail(&mut self, at: usize) -> Vec<T> {
        let mut start = self.split_off(at);
        mem::swap(&mut start, self);
        start
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Pixel {
    Black,       // 0
    White,       // 1
    Transparent, // 2
}

#[inline]
fn parse_char(c: char) -> Result<Pixel> {
    match c {
        '0' => Ok(Pixel::Black),
        '1' => Ok(Pixel::White),
        '2' => Ok(Pixel::Transparent),
        other => Err(anyhow!("Invalid digit: {}", other)),
    }
}

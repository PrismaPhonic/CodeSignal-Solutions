type Canvas = Vec<Vec<char>>;
type Rectangle = Vec<usize>;

fn draw_rectangle(canvas: Canvas, rect: Vec<i32>) -> Canvas {
    let mut drawn = canvas.clone();
    let rect: Rectangle = rect.iter().map(|num| *num as usize).collect();
    draw_asterisks(&mut drawn, &rect);
    draw_hyphens(&mut drawn, &rect);
    draw_vertical_bars(&mut drawn, &rect);
    drawn
}

fn draw_asterisks(canvas: &mut Canvas, rect: &Rectangle) {
    // paint top two corner asterisks
    canvas[rect[1]][rect[0]] = '*';
    canvas[rect[3]][rect[0]] = '*';

    // paint bottom two corner asterisks
    canvas[rect[1]][rect[2]] = '*';
    canvas[rect[3]][rect[2]] = '*';
}

fn draw_hyphens(canvas: &mut Canvas, rect: &Rectangle) {
    [rect[1], rect[3]].iter().for_each(|y| {
        for x in rect[0] + 1..rect[2] {
            canvas[*y][x] = '-'
        }
    });
}

fn draw_vertical_bars(canvas: &mut Canvas, rect: &Rectangle) {
    (rect[1] + 1..rect[3]).into_iter().for_each(|y| {
        // left side
        canvas[y][rect[0]] = '|';
        // right side
        canvas[y][rect[2]] = '|';
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let canvas = vec![
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'],
            vec!['b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'],
        ];
        let rectangle = vec![1, 1, 4, 3];
        let output = vec![
            vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            vec!['a', '*', '-', '-', '*', 'a', 'a', 'a'],
            vec!['a', '|', 'a', 'a', '|', 'a', 'a', 'a'],
            vec!['b', '*', '-', '-', '*', 'b', 'b', 'b'],
            vec!['b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'],
        ];
        assert_eq!(draw_rectangle(canvas, rectangle), output);
    }
}

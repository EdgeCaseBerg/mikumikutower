
use crate::Rect;
use crate::grid_layout::GridLayout;

const FONTSHEET_LAYOUT: GridLayout = {
    GridLayout {
        area: Rect {
            x: 0,
            y: 0,
            width: 145,
            height: 1412,
        },
        rows: 83,
        columns: 16,
        cell_gap: 1,
    }
};

pub fn get_rects_for_str(str: &str) -> Vec<Rect> {
    // font sheet starts at space, so -32 from the character's ascii value
    // to get the index. Then we need to convert that index into
    // This is horrifically inefficient, but for testing it should be okay:
    let cells: Vec<Rect> = FONTSHEET_LAYOUT.iter_cells().map(|(_, _, r)| r).collect();
    str.chars()
        .map(|c| {
            let ascii = c as u32;
            if ascii > 127 {
                cells[0].clone()
            } else {
                cells[ascii as usize - 32].clone()
            }
        })
        .collect()
}
